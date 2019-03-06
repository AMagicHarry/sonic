// Sonic
//
// Fast, lightweight and schema-less search backend
// Copyright: 2019, Valerian Saliou <valerian@valeriansaliou.name>
// License: Mozilla Public License v2.0 (MPL v2.0)

use crate::lexer::token::TokenLexer;
use crate::store::item::StoreItem;
use crate::store::kv::{StoreKVActionBuilder, StoreKVPool};

pub struct ExecutorPop;

impl ExecutorPop {
    pub fn execute<'a>(store: StoreItem<'a>, mut lexer: TokenLexer<'a>) -> Result<u64, ()> {
        // TODO: only pop token lexer terms

        if let StoreItem(collection, Some(bucket), Some(object)) = store {
            if let Ok(kv_store) = StoreKVPool::acquire(collection) {
                let action = StoreKVActionBuilder::write(bucket, kv_store);

                let pop_result: Result<u64, ()>;

                // Try to resolve existing OID to IID (if it does not exist, there is nothing to \
                //   be flushed)
                let oid = object.as_str().to_owned();

                if let Ok(iid_value) = action.get_oid_to_iid(&oid) {
                    let mut count_popped = 0;

                    if let Some(iid) = iid_value {
                        // Resolve terms associated to IID
                        let iid_terms = action
                            .get_iid_to_terms(iid)
                            .ok()
                            .unwrap_or(None)
                            .unwrap_or(Vec::new());

                        // Delete OID <> IID association
                        action.delete_oid_to_iid(&oid).ok();
                        action.delete_iid_to_oid(iid).ok();
                        action.delete_iid_to_terms(iid).ok();

                        // Delete IID from each associated term
                        for iid_term in iid_terms {
                            if let Ok(Some(mut iid_term_iids)) = action.get_term_to_iids(&iid_term)
                            {
                                if iid_term_iids.contains(&iid) == true {
                                    count_popped += 1;

                                    iid_term_iids.remove_item(&iid);
                                }

                                if iid_term_iids.is_empty() == true {
                                    action.delete_term_to_iids(&iid_term).ok();
                                } else {
                                    action.set_term_to_iids(&iid_term, &iid_term_iids).ok();
                                }
                            }
                        }
                    }

                    pop_result = Ok(count_popped);
                } else {
                    pop_result = Err(());
                }

                return pop_result;
            }
        }

        Err(())
    }
}
