// Sonic
//
// Fast, lightweight and schema-less search backend
// Copyright: 2019, Valerian Saliou <valerian@valeriansaliou.name>
// License: Mozilla Public License v2.0 (MPL v2.0)

use byteorder::{ByteOrder, LittleEndian, ReadBytesExt};
use hashbrown::HashMap;
use rocksdb::{
    DBCompactionStyle, DBCompressionType, DBVector, Error as DBError, Options as DBOptions, DB,
};
use std::fs;
use std::io::Cursor;
use std::path::PathBuf;
use std::sync::{Arc, Mutex, RwLock};
use std::time::SystemTime;
use std::vec::Drain;

use super::generic::{
    StoreGeneric, StoreGenericActionBuilder, StoreGenericBuilder, StoreGenericPool,
};
use super::identifiers::*;
use super::keyer::{StoreKeyerBuilder, StoreKeyerHasher};
use crate::APP_CONF;

pub struct StoreKVPool;
pub struct StoreKVBuilder;

pub struct StoreKV {
    database: DB,
    last_used: Arc<RwLock<SystemTime>>,
    pub lock: RwLock<bool>,
}

pub struct StoreKVActionBuilder;

pub struct StoreKVAction {
    store: Option<StoreKVBox>,
}

#[derive(PartialEq)]
pub enum StoreKVAcquireMode {
    Any,
    OpenOnly,
}

type StoreKVAtom = u32;
type StoreKVBox = Arc<StoreKV>;
type StoreKVKey = (StoreKVAtom, StoreKVAtom);

lazy_static! {
    pub static ref STORE_ACCESS_LOCK: Arc<RwLock<bool>> = Arc::new(RwLock::new(false));
    static ref STORE_WRITE_LOCK: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
    static ref STORE_POOL: Arc<RwLock<HashMap<StoreKVKey, StoreKVBox>>> =
        Arc::new(RwLock::new(HashMap::new()));
}

impl StoreKVPool {
    pub fn acquire<'a, T: Into<&'a str>>(
        mode: StoreKVAcquireMode,
        collection: T,
        bucket: T,
    ) -> Result<Option<StoreKVBox>, ()> {
        let (collection_str, bucket_str) = (collection.into(), bucket.into());

        let pool_key = (
            StoreKeyerHasher::to_compact(collection_str),
            StoreKeyerHasher::to_compact(bucket_str),
        );

        // Acquire general lock, and reference it in context
        // Notice: this prevents database to be opened while also erased; or 2 databases on the \
        //   same collection to be opened at the same time.
        let _write = STORE_WRITE_LOCK.lock().unwrap();

        // Acquire a thread-safe store pool reference in read mode
        let store_pool_read = STORE_POOL.read().unwrap();

        if let Some(store_kv) = store_pool_read.get(&pool_key) {
            Self::proceed_acquire_cache("kv", collection_str, bucket_str, pool_key, &store_kv)
                .map(|result| Some(result))
        } else {
            info!(
                "kv store not in pool for collection: {} <{:x?}> / bucket: {} <{:x?}>, opening it",
                collection_str, pool_key.0, bucket_str, pool_key.1
            );

            // Important: we need to drop the read reference first, to avoid \
            //   dead-locking when acquiring the RWLock in write mode in this block.
            drop(store_pool_read);

            // Check if can open database?
            let can_open_db = if mode == StoreKVAcquireMode::OpenOnly {
                StoreKVBuilder::path(pool_key.0, Some(pool_key.1)).exists()
            } else {
                true
            };

            // Open KV database? (ie. we do not need to create a new KV database file tree if \
            //   the database does not exist yet on disk and we are just looking to read data from \
            //   it)
            if can_open_db == true {
                Self::proceed_acquire_open("kv", collection_str, bucket_str, pool_key, &*STORE_POOL)
                    .map(|result| Some(result))
            } else {
                Ok(None)
            }
        }
    }

    pub fn janitor() {
        Self::proceed_janitor("kv", &*STORE_POOL, APP_CONF.store.kv.pool.inactive_after)
    }
}

impl StoreGenericPool<StoreKVAtom, StoreKV, StoreKVBuilder> for StoreKVPool {}

impl StoreKVBuilder {
    fn open(collection_hash: StoreKVAtom, bucket_hash: StoreKVAtom) -> Result<DB, DBError> {
        debug!(
            "opening key-value database for collection: <{:x?}>",
            collection_hash
        );

        // Configure database options
        let db_options = Self::configure();

        // Open database at path for collection
        DB::open(&db_options, Self::path(collection_hash, Some(bucket_hash)))
    }

    fn path(collection_hash: StoreKVAtom, bucket_hash: Option<StoreKVAtom>) -> PathBuf {
        let mut final_path = APP_CONF
            .store
            .kv
            .path
            .join(format!("{:x?}", collection_hash));

        if let Some(bucket_hash) = bucket_hash {
            final_path = final_path.join(format!("{:x?}", bucket_hash));
        }

        final_path
    }

    fn configure() -> DBOptions {
        debug!("configuring key-value database");

        // Make database options
        let mut db_options = DBOptions::default();

        db_options.create_if_missing(true);
        db_options.set_use_fsync(false);
        db_options.set_compaction_style(DBCompactionStyle::Level);

        db_options.set_compression_type(if APP_CONF.store.kv.database.compress == true {
            DBCompressionType::Lz4
        } else {
            DBCompressionType::None
        });

        db_options.increase_parallelism(APP_CONF.store.kv.database.parallelism as i32);
        db_options.set_max_open_files(APP_CONF.store.kv.database.max_files as i32);
        db_options
            .set_max_background_compactions(APP_CONF.store.kv.database.max_compactions as i32);
        db_options.set_max_background_flushes(APP_CONF.store.kv.database.max_flushes as i32);

        db_options
    }
}

impl StoreGenericBuilder<StoreKVAtom, StoreKV> for StoreKVBuilder {
    fn new(collection_hash: StoreKVAtom, bucket_hash: StoreKVAtom) -> Result<StoreKV, ()> {
        Self::open(collection_hash, bucket_hash)
            .map(|db| StoreKV {
                database: db,
                last_used: Arc::new(RwLock::new(SystemTime::now())),
                lock: RwLock::new(false),
            })
            .or_else(|err| {
                error!("failed opening kv: {}", err);

                Err(())
            })
    }
}

impl StoreKV {
    pub fn get(&self, key: &[u8]) -> Result<Option<DBVector>, DBError> {
        self.database.get(key)
    }

    pub fn put(&self, key: &[u8], data: &[u8]) -> Result<(), DBError> {
        self.database.put(key, data)
    }

    pub fn delete(&self, key: &[u8]) -> Result<(), DBError> {
        self.database.delete(key)
    }
}

impl StoreGeneric for StoreKV {
    fn ref_last_used<'a>(&'a self) -> &'a RwLock<SystemTime> {
        &self.last_used
    }
}

impl StoreKVActionBuilder {
    pub fn access(store: Option<StoreKVBox>) -> StoreKVAction {
        Self::build(store)
    }

    pub fn erase<'a, T: Into<&'a str>>(collection: T, bucket: Option<T>) -> Result<u32, ()> {
        Self::dispatch_erase(
            "kv",
            collection,
            bucket,
            &*STORE_ACCESS_LOCK,
            &*STORE_WRITE_LOCK,
        )
    }
}

impl StoreGenericActionBuilder<Option<StoreKVBox>, StoreKVAction> for StoreKVActionBuilder {
    fn build(store: Option<StoreKVBox>) -> StoreKVAction {
        StoreKVAction { store: store }
    }

    fn proceed_erase_collection(collection_str: &str) -> Result<u32, ()> {
        let collection_atom = StoreKeyerHasher::to_compact(collection_str);
        let collection_path = StoreKVBuilder::path(collection_atom, None);

        if collection_path.exists() == true {
            debug!(
                "kv collection store exists, erasing: {}/* at path: {:?}",
                collection_str, &collection_path
            );

            // Scan collection directory for contained bucket files
            let mut buckets: Vec<String> = Vec::new();

            if let Ok(entries) = fs::read_dir(&collection_path) {
                for entry in entries {
                    if let Ok(entry) = entry {
                        if let Ok(entry_type) = entry.file_type() {
                            if entry_type.is_dir() == true {
                                if let Ok(entry_name) = entry.file_name().into_string() {
                                    buckets.push(entry_name);
                                }
                            }
                        }
                    }
                }
            } else {
                error!(
                    "failed reading directory for kv erasure: {:?}",
                    collection_path
                );
            }

            // Force a KV store close (on all contained buckets)
            {
                let mut store_pool_write = STORE_POOL.write().unwrap();

                for bucket in buckets {
                    debug!(
                        "forcibly closing kv store bucket: {}/{}",
                        collection_str, bucket
                    );

                    store_pool_write
                        .remove(&(collection_atom, StoreKeyerHasher::to_compact(&bucket)));
                }
            }

            // Remove KV store storage from filesystem
            let erase_result = fs::remove_dir_all(&collection_path);

            if erase_result.is_ok() == true {
                debug!("done with kv collection erasure");

                Ok(1)
            } else {
                Err(())
            }
        } else {
            debug!(
                "kv collection store does not exist, consider already erased: {}/* at path: {:?}",
                collection_str, &collection_path
            );

            Ok(0)
        }
    }

    fn proceed_erase_bucket(collection_str: &str, bucket_str: &str) -> Result<u32, ()> {
        debug!(
            "sub-erase on kv bucket: {} for collection: {}",
            bucket_str, collection_str
        );

        let (collection_atom, bucket_atom) = (
            StoreKeyerHasher::to_compact(collection_str),
            StoreKeyerHasher::to_compact(bucket_str),
        );

        let bucket_path = StoreKVBuilder::path(collection_atom, Some(bucket_atom));

        if bucket_path.exists() == true {
            debug!(
                "kv bucket store exists, erasing: {}/{} at path: {:?}",
                collection_str, bucket_str, &bucket_path
            );

            // Force a KV store close
            {
                STORE_POOL
                    .write()
                    .unwrap()
                    .remove(&(collection_atom, bucket_atom));
            }

            // Remove KV store storage from filesystem
            let erase_result = fs::remove_dir_all(&bucket_path);

            if erase_result.is_ok() == true {
                debug!("done with kv bucket erasure");

                Ok(1)
            } else {
                Err(())
            }
        } else {
            debug!(
                "kv bucket store does not exist, consider already erased: {}/{} at path: {:?}",
                collection_str, bucket_str, &bucket_path
            );

            Ok(0)
        }
    }
}

impl StoreKVAction {
    /// Meta-to-Value mapper
    ///
    /// [IDX=0] ((meta)) ~> ((value))
    pub fn get_meta_to_value(&self, meta: StoreMetaKey) -> Result<Option<StoreMetaValue>, ()> {
        if let Some(ref store) = self.store {
            let store_key = StoreKeyerBuilder::meta_to_value(&meta);

            debug!("store get meta-to-value: {}", store_key);

            match store.get(&store_key.as_bytes()) {
                Ok(Some(value)) => {
                    debug!("got meta-to-value: {}", store_key);

                    Ok(if let Some(value) = value.to_utf8() {
                        match meta {
                            StoreMetaKey::IIDIncr => value
                                .parse::<StoreObjectIID>()
                                .ok()
                                .map(|value| StoreMetaValue::IIDIncr(value))
                                .or(None),
                        }
                    } else {
                        None
                    })
                }
                Ok(None) => {
                    debug!("no meta-to-value found: {}", store_key);

                    Ok(None)
                }
                Err(err) => {
                    error!(
                        "error getting meta-to-value: {} with trace: {}",
                        store_key, err
                    );

                    Err(())
                }
            }
        } else {
            Ok(None)
        }
    }

    pub fn set_meta_to_value(&self, meta: StoreMetaKey, value: StoreMetaValue) -> Result<(), ()> {
        if let Some(ref store) = self.store {
            let store_key = StoreKeyerBuilder::meta_to_value(&meta);

            debug!("store set meta-to-value: {}", store_key);

            let value_string = match value {
                StoreMetaValue::IIDIncr(iid_incr) => iid_incr.to_string(),
            };

            store
                .put(&store_key.as_bytes(), value_string.as_bytes())
                .or(Err(()))
        } else {
            Err(())
        }
    }

    /// Term-to-IIDs mapper
    ///
    /// [IDX=1] ((term)) ~> [((iid))]
    pub fn get_term_to_iids(
        &self,
        term_hashed: StoreTermHashed,
    ) -> Result<Option<Vec<StoreObjectIID>>, ()> {
        if let Some(ref store) = self.store {
            let store_key = StoreKeyerBuilder::term_to_iids(term_hashed);

            debug!("store get term-to-iids: {}", store_key);

            match store.get(&store_key.as_bytes()) {
                Ok(Some(value)) => {
                    debug!(
                        "got term-to-iids: {} with encoded value: {:?}",
                        store_key, &*value
                    );

                    Self::decode_u32_list(&*value)
                        .or(Err(()))
                        .map(|value_decoded| {
                            debug!(
                                "got term-to-iids: {} with decoded value: {:?}",
                                store_key, &value_decoded
                            );

                            Some(value_decoded)
                        })
                }
                Ok(None) => {
                    debug!("no term-to-iids found: {}", store_key);

                    Ok(None)
                }
                Err(err) => {
                    error!(
                        "error getting term-to-iids: {} with trace: {}",
                        store_key, err
                    );

                    Err(())
                }
            }
        } else {
            Ok(None)
        }
    }

    pub fn set_term_to_iids(
        &self,
        term_hashed: StoreTermHashed,
        iids: &[StoreObjectIID],
    ) -> Result<(), ()> {
        if let Some(ref store) = self.store {
            let store_key = StoreKeyerBuilder::term_to_iids(term_hashed);

            debug!("store set term-to-iids: {}", store_key);

            // Encode IID list into storage serialized format
            let iids_encoded = Self::encode_u32_list(iids);

            debug!(
                "store set term-to-iids: {} with encoded value: {:?}",
                store_key, iids_encoded
            );

            store.put(&store_key.as_bytes(), &iids_encoded).or(Err(()))
        } else {
            Err(())
        }
    }

    pub fn delete_term_to_iids(&self, term_hashed: StoreTermHashed) -> Result<(), ()> {
        if let Some(ref store) = self.store {
            let store_key = StoreKeyerBuilder::term_to_iids(term_hashed);

            debug!("store delete term-to-iids: {}", store_key);

            store.delete(&store_key.as_bytes()).or(Err(()))
        } else {
            Err(())
        }
    }

    /// OID-to-IID mapper
    ///
    /// [IDX=2] ((oid)) ~> ((iid))
    pub fn get_oid_to_iid(&self, oid: &StoreObjectOID) -> Result<Option<StoreObjectIID>, ()> {
        if let Some(ref store) = self.store {
            let store_key = StoreKeyerBuilder::oid_to_iid(oid);

            debug!("store get oid-to-iid: {}", store_key);

            match store.get(&store_key.as_bytes()) {
                Ok(Some(value)) => {
                    debug!(
                        "got oid-to-iid: {} with encoded value: {:?}",
                        store_key, &*value
                    );

                    Self::decode_u32(&*value).or(Err(())).map(|value_decoded| {
                        debug!(
                            "got oid-to-iid: {} with decoded value: {:?}",
                            store_key, &value_decoded
                        );

                        Some(value_decoded)
                    })
                }
                Ok(None) => {
                    debug!("no oid-to-iid found: {}", store_key);

                    Ok(None)
                }
                Err(err) => {
                    error!(
                        "error getting oid-to-iid: {} with trace: {}",
                        store_key, err
                    );

                    Err(())
                }
            }
        } else {
            Ok(None)
        }
    }

    pub fn set_oid_to_iid(&self, oid: &StoreObjectOID, iid: StoreObjectIID) -> Result<(), ()> {
        if let Some(ref store) = self.store {
            let store_key = StoreKeyerBuilder::oid_to_iid(oid);

            debug!("store set oid-to-iid: {}", store_key);

            // Encode IID
            let iid_encoded = Self::encode_u32(iid);

            debug!(
                "store set oid-to-iid: {} with encoded value: {:?}",
                store_key, iid_encoded
            );

            store.put(&store_key.as_bytes(), &iid_encoded).or(Err(()))
        } else {
            Err(())
        }
    }

    pub fn delete_oid_to_iid(&self, oid: &StoreObjectOID) -> Result<(), ()> {
        if let Some(ref store) = self.store {
            let store_key = StoreKeyerBuilder::oid_to_iid(oid);

            debug!("store delete oid-to-iid: {}", store_key);

            store.delete(&store_key.as_bytes()).or(Err(()))
        } else {
            Err(())
        }
    }

    /// IID-to-OID mapper
    ///
    /// [IDX=3] ((iid)) ~> ((oid))
    pub fn get_iid_to_oid(&self, iid: StoreObjectIID) -> Result<Option<StoreObjectOID>, ()> {
        if let Some(ref store) = self.store {
            let store_key = StoreKeyerBuilder::iid_to_oid(iid);

            debug!("store get iid-to-oid: {}", store_key);

            match store.get(&store_key.as_bytes()) {
                Ok(Some(value)) => Ok(value.to_utf8().map(|value| value.to_string())),
                Ok(None) => Ok(None),
                Err(_) => Err(()),
            }
        } else {
            Ok(None)
        }
    }

    pub fn set_iid_to_oid(&self, iid: StoreObjectIID, oid: &StoreObjectOID) -> Result<(), ()> {
        if let Some(ref store) = self.store {
            let store_key = StoreKeyerBuilder::iid_to_oid(iid);

            debug!("store set iid-to-oid: {}", store_key);

            store.put(&store_key.as_bytes(), oid.as_bytes()).or(Err(()))
        } else {
            Err(())
        }
    }

    pub fn delete_iid_to_oid(&self, iid: StoreObjectIID) -> Result<(), ()> {
        if let Some(ref store) = self.store {
            let store_key = StoreKeyerBuilder::iid_to_oid(iid);

            debug!("store delete iid-to-oid: {}", store_key);

            store.delete(&store_key.as_bytes()).or(Err(()))
        } else {
            Err(())
        }
    }

    /// IID-to-Terms mapper
    ///
    /// [IDX=4] ((iid)) ~> [((term))]
    pub fn get_iid_to_terms(
        &self,
        iid: StoreObjectIID,
    ) -> Result<Option<Vec<StoreTermHashed>>, ()> {
        if let Some(ref store) = self.store {
            let store_key = StoreKeyerBuilder::iid_to_terms(iid);

            debug!("store get iid-to-terms: {}", store_key);

            match store.get(&store_key.as_bytes()) {
                Ok(Some(value)) => {
                    debug!(
                        "got iid-to-terms: {} with encoded value: {:?}",
                        store_key, &*value
                    );

                    Self::decode_u32_list(&*value)
                        .or(Err(()))
                        .map(|value_decoded| {
                            debug!(
                                "got iid-to-terms: {} with decoded value: {:?}",
                                store_key, &value_decoded
                            );

                            if value_decoded.is_empty() == false {
                                Some(value_decoded)
                            } else {
                                None
                            }
                        })
                }
                Ok(None) => Ok(None),
                Err(_) => Err(()),
            }
        } else {
            Ok(None)
        }
    }

    pub fn set_iid_to_terms(
        &self,
        iid: StoreObjectIID,
        terms_hashed: &[StoreTermHashed],
    ) -> Result<(), ()> {
        if let Some(ref store) = self.store {
            let store_key = StoreKeyerBuilder::iid_to_terms(iid);

            debug!("store set iid-to-terms: {}", store_key);

            // Encode term list into storage serialized format
            let terms_hashed_encoded = Self::encode_u32_list(terms_hashed);

            debug!(
                "store set iid-to-terms: {} with encoded value: {:?}",
                store_key, terms_hashed_encoded
            );

            store
                .put(&store_key.as_bytes(), &terms_hashed_encoded)
                .or(Err(()))
        } else {
            Err(())
        }
    }

    pub fn delete_iid_to_terms(&self, iid: StoreObjectIID) -> Result<(), ()> {
        if let Some(ref store) = self.store {
            let store_key = StoreKeyerBuilder::iid_to_terms(iid);

            debug!("store delete iid-to-terms: {}", store_key);

            store.delete(&store_key.as_bytes()).or(Err(()))
        } else {
            Err(())
        }
    }

    pub fn batch_flush_bucket(
        &self,
        iid: StoreObjectIID,
        oid: &StoreObjectOID,
        iid_terms_hashed: &[StoreTermHashed],
    ) -> Result<u32, ()> {
        let mut count = 0;

        debug!(
            "store batch flush bucket: {} with hashed terms: {:?}",
            iid, iid_terms_hashed
        );

        // Delete OID <> IID association
        match (
            self.delete_oid_to_iid(&oid),
            self.delete_iid_to_oid(iid),
            self.delete_iid_to_terms(iid),
        ) {
            (Ok(_), Ok(_), Ok(_)) => {
                // Delete IID from each associated term
                for iid_term in iid_terms_hashed {
                    if let Ok(Some(mut iid_term_iids)) = self.get_term_to_iids(*iid_term) {
                        if iid_term_iids.contains(&iid) == true {
                            count += 1;

                            // Remove IID from list of IIDs
                            iid_term_iids.retain(|cur_iid| cur_iid != &iid);
                        }

                        let is_ok = if iid_term_iids.is_empty() == true {
                            self.delete_term_to_iids(*iid_term).is_ok()
                        } else {
                            self.set_term_to_iids(*iid_term, &iid_term_iids).is_ok()
                        };

                        if is_ok == false {
                            return Err(());
                        }
                    }
                }

                Ok(count)
            }
            _ => Err(()),
        }
    }

    pub fn batch_truncate_object(
        &self,
        term_hashed: StoreTermHashed,
        term_iids_drain: Drain<StoreObjectIID>,
    ) -> Result<u32, ()> {
        let mut count = 0;

        for term_iid_drain in term_iids_drain {
            debug!("store batch truncate object iid: {}", term_iid_drain);

            // Nuke term in IID to Terms list
            if let Ok(Some(mut term_iid_drain_terms)) = self.get_iid_to_terms(term_iid_drain) {
                count += 1;

                term_iid_drain_terms.retain(|cur_term| cur_term != &term_hashed);

                // IID to Terms list is empty? Flush whole object.
                if term_iid_drain_terms.is_empty() == true {
                    // Acquire OID for this drained IID
                    if let Ok(Some(term_iid_drain_oid)) = self.get_iid_to_oid(term_iid_drain) {
                        if self
                            .batch_flush_bucket(term_iid_drain, &term_iid_drain_oid, &Vec::new())
                            .is_err()
                            == true
                        {
                            error!(
                                "failed executing store batch truncate object batch-flush-bucket"
                            );
                        }
                    } else {
                        error!("failed getting store batch truncate object iid-to-oid");
                    }
                } else {
                    // Update IID to Terms list
                    if self
                        .set_iid_to_terms(term_iid_drain, &term_iid_drain_terms)
                        .is_err()
                        == true
                    {
                        error!("failed setting store batch truncate object iid-to-terms");
                    }
                }
            }
        }

        Ok(count)
    }

    fn encode_u32(decoded: u32) -> [u8; 4] {
        let mut encoded = [0; 4];

        LittleEndian::write_u32(&mut encoded, decoded);

        encoded
    }

    fn decode_u32(encoded: &[u8]) -> Result<u32, ()> {
        Cursor::new(encoded).read_u32::<LittleEndian>().or(Err(()))
    }

    fn encode_u32_list(decoded: &[u32]) -> Vec<u8> {
        let mut encoded = Vec::new();

        for decoded_item in decoded {
            encoded.extend(&Self::encode_u32(*decoded_item))
        }

        encoded
    }

    fn decode_u32_list(encoded: &[u8]) -> Result<Vec<u32>, ()> {
        let mut decoded = Vec::new();

        for encoded_chunk in encoded.chunks(4) {
            if let Ok(decoded_chunk) = Self::decode_u32(encoded_chunk) {
                decoded.push(decoded_chunk);
            } else {
                return Err(());
            }
        }

        Ok(decoded)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_acquires_database() {
        assert!(StoreKVPool::acquire(StoreKVAcquireMode::Any, "c:test:1", "b:test:1").is_ok());
    }

    #[test]
    fn it_janitors_database() {
        StoreKVPool::janitor();
    }

    #[test]
    fn it_proceeds_primitives() {
        let store = StoreKVPool::acquire(StoreKVAcquireMode::Any, "c:test:2", "b:test:2")
            .unwrap()
            .unwrap();

        assert!(store.get(&[0]).is_ok());
        assert!(store.put(&[0], &[1, 0, 0, 0]).is_ok());
        assert!(store.delete(&[0]).is_ok());
    }

    #[test]
    fn it_proceeds_actions() {
        let store = StoreKVPool::acquire(StoreKVAcquireMode::Any, "c:test:3", "b:test:3").unwrap();
        let action = StoreKVActionBuilder::access(store);

        assert!(action.get_meta_to_value(StoreMetaKey::IIDIncr).is_ok());
        assert!(action
            .set_meta_to_value(StoreMetaKey::IIDIncr, StoreMetaValue::IIDIncr(1))
            .is_ok());

        assert!(action.get_term_to_iids(1).is_ok());
        assert!(action.set_term_to_iids(1, &[0, 1, 2]).is_ok());
        assert!(action.delete_term_to_iids(1).is_ok());

        assert!(action.get_oid_to_iid(&"s".to_string()).is_ok());
        assert!(action.set_oid_to_iid(&"s".to_string(), 4).is_ok());
        assert!(action.delete_oid_to_iid(&"s".to_string()).is_ok());

        assert!(action.get_iid_to_oid(4).is_ok());
        assert!(action.set_iid_to_oid(4, &"s".to_string()).is_ok());
        assert!(action.delete_iid_to_oid(4).is_ok());

        assert!(action.get_iid_to_terms(4).is_ok());
        assert!(action.set_iid_to_terms(4, &[45402]).is_ok());
        assert!(action.delete_iid_to_terms(4).is_ok());
    }

    #[test]
    fn it_encodes_atom() {
        assert_eq!(StoreKVAction::encode_u32(0), [0, 0, 0, 0]);
        assert_eq!(StoreKVAction::encode_u32(1), [1, 0, 0, 0]);
        assert_eq!(StoreKVAction::encode_u32(45402), [90, 177, 0, 0]);
    }

    #[test]
    fn it_decodes_atom() {
        assert_eq!(StoreKVAction::decode_u32(&[0, 0, 0, 0]), Ok(0));
        assert_eq!(StoreKVAction::decode_u32(&[1, 0, 0, 0]), Ok(1));
        assert_eq!(StoreKVAction::decode_u32(&[90, 177, 0, 0]), Ok(45402));
    }

    #[test]
    fn it_encodes_atom_list() {
        assert_eq!(
            StoreKVAction::encode_u32_list(&[0, 2, 3]),
            [0, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0]
        );
        assert_eq!(StoreKVAction::encode_u32_list(&[45402]), [90, 177, 0, 0]);
    }

    #[test]
    fn it_decodes_atom_list() {
        assert_eq!(
            StoreKVAction::decode_u32_list(&[0, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0]),
            Ok(vec![0, 2, 3])
        );
        assert_eq!(
            StoreKVAction::decode_u32_list(&[90, 177, 0, 0]),
            Ok(vec![45402])
        );
    }
}
