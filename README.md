Sonic
=====

[![Build Status](https://travis-ci.org/valeriansaliou/sonic.svg?branch=master)](https://travis-ci.org/valeriansaliou/sonic) [![Dependency Status](https://deps.rs/repo/github/valeriansaliou/sonic/status.svg)](https://deps.rs/repo/github/valeriansaliou/sonic) [![Buy Me A Coffee](https://img.shields.io/badge/buy%20me%20a%20coffee-donate-yellow.svg)](https://www.buymeacoffee.com/valeriansaliou)

**Sonic is a fast, lightweight and schema-less search backend. It ingests search texts and identifier tuples, that can then be queried against.**

Sonic can be used as a simple alternative to super-heavy and full-featured search backends such as Elasticsearch in some use-cases. It is capable of normalizing natural language search queries, auto-completing a search query and providing the most relevant results for a query.

A strong attention to performance and code cleanliness has been given when designing Sonic. It aims at being crash-free, super-fast and puts minimum strain on server resources (our measurements have shown that Sonic - when under load - responds to search queries in the μs range, eats ~30MB RAM and has a north-to-null CPU footprint; [see our benchmarks](#how-fast--lightweight-is-it)).

**🇫🇷 Crafted in Nantes, France.**

**:newspaper: The Sonic project was initially announced in [a post on my personal journal](https://journal.valeriansaliou.name/announcing-sonic-a-super-light-alternative-to-elasticsearch/).**

![Sonic](https://valeriansaliou.github.io/sonic/images/banner.jpg)

> _« Sonic » is the mascot of the Sonic project. I drew it to look like a psychedelic hipster hedgehog._

## Who uses it?

<table>
<tr>
<td align="center"><a href="https://crisp.chat/"><img src="https://valeriansaliou.github.io/sonic/images/logo-crisp.png" height="64" /></a></td>
</tr>
<tr>
<td align="center">Crisp</td>
</tr>
</table>

_👋 You use Sonic and you want to be listed there? [Contact me](https://valeriansaliou.name/)._

## Demo

Sonic is integrated in all Crisp search products on the [Crisp](https://crisp.chat/) platform. It is used to index half a billion objects on a $5/mth 1-vCPU SSD cloud server (as of 2019). Crisp users use it to search in their messages, conversations, contacts, helpdesk articles and more.

**You can test Sonic live on: [Crisp Helpdesk](https://help.crisp.chat/), and get an idea of the speed and relevance of Sonic search results.**

![Demo on Crisp Helpdesk search](https://valeriansaliou.github.io/sonic/images/crisp-search-demo.gif)

> _Sonic fuzzy search in helpdesk articles at its best. Lookup for any word or group of terms, get results instantly._

## Features

* **Search terms are stored in collections, organized in buckets**; you may use a single bucket, or a bucket per user on your platform if you need to search in separate indexes.
* **Search results return object identifiers**, that can be resolved from an external database if you need to enrich the search results. This makes Sonic a simple word index, that points to identifier results. Sonic doesn't store any direct textual data in its index, but it still holds a word graph for auto-completion and typo corrections.
* **Search query typos are corrected** if there are not enough exact-match results for a given word in a search query, Sonic tries to correct the word and tries against alternate words. You're allowed to make mistakes when searching.
* **Insert and remove items in the index**; index-altering operations are light and can be commited to the server while it is running. A background tasker handles the job of consolidating the index so that the entries you have pushed or popped are quickly made available for search.
* **Auto-complete any word** in real-time via the suggest operation. This helps build a snappy word suggestion feature in your end-user search interface.
* **Networked channel interface (Sonic Channel)**, that let you search your index, manage data ingestion (push in the index, pop from the index, flush a collection, flush a bucket, etc.) and perform administrative actions. The Sonic Channel protocol was designed to be lightweight on resources and simple to integrate with (the protocol is specified in the sections below).
* **Easy-to-use libraries**, that let you connect to Sonic Channel from your apps.

## Limitations

* **Indexed data limits**: Sonic is designed for large search indexes split over thousands of search buckets per collection. An IID (ie. Internal-ID) is stored in the index as a 32 bits number, which theoretically allow up to ~4.2 billion objects to be indexed (ie. OID) per bucket. We've observed storage savings of 30% to 40%, which justifies the trade-off on large databases (versus Sonic using 64 bits IIDs). Also, Sonic only keeps the N most recently pushed results for a given word, in a sliding window way (the sliding window width can be configured).
* **Search query limits**: Sonic Natural Language Processing system (NLP) does not work at the sentence-level, for storage compactness reasons (we keep the FST graph shallow as to reduce time and space complexity). It works at the word-level, and is thus able to search per-word and can predict a word based on user input, though it is unable to predict the next word in a sentence.
* **Real-time limits**: the FST needs to be re-built every time a word is pushed or popped from the bucket graph. As this is quite heavy, Sonic batches rebuild cycles. If you have just pushed a new word to the index and you are not seeing it in the `SUGGEST` command yet, wait for the next rebuild cycle to kick-in, or force it with `TRIGGER consolidate` in a `control` channel.
* **Interoperability limits**: Sonic Channel protocol is the only way to read and write search entries to the Sonic search index. Sonic does not expose any HTTP API. Sonic Channel has been built with performance and minimal network footprint in mind. If you need to access Sonic Channel from an unsupported programming language, you can either [open an issue](https://github.com/valeriansaliou/sonic/issues/new) or look at the reference [node-sonic-channel](https://github.com/valeriansaliou/node-sonic-channel) implementation and build it in your target programming language.
* **Hardware limits**: Sonic performs the search on the file-system directly; ie. it does not fit the index in RAM. A search query results in a lot of random accesses on the disk, which means that it will be quite slow on old-school HDDs and super-fast on newer SSDs. Do store the Sonic database on SSD-backed file systems only.

## How to use it?

### Installation

Sonic is built in Rust. To install it, either download a version from the [Sonic releases](https://github.com/valeriansaliou/sonic/releases) page, use `cargo install` or pull the source code from `master`.

**Install from source:**

If you pulled the source code from Git, you can build it using `cargo`:

```bash
cargo build --release
```

You can find the built binaries in the `./target/release` directory.

_Install `clang` to be able to compile the required RocksDB dependency._

**Install from Cargo:**

You can install Sonic directly with `cargo install`:

```bash
cargo install sonic-server
```

Ensure that your `$PATH` is properly configured to source the Crates binaries, and then run Sonic using the `sonic` command.

**Install from Docker Hub:**

You might find it convenient to run Sonic via Docker. You can find the pre-built Sonic image on Docker Hub as [valeriansaliou/sonic](https://hub.docker.com/r/valeriansaliou/sonic/).

First, pull the `valeriansaliou/sonic` image:

```bash
docker pull valeriansaliou/sonic:v1.1.0
```

Then, seed it a configuration file and run it (replace `/path/to/your/sonic/config.cfg` with the path to your configuration file):

```bash
docker run -p 1491:1491 -v /path/to/your/sonic/config.cfg:/etc/sonic.cfg -v /path/to/your/sonic/store/:/var/lib/sonic/store/ valeriansaliou/sonic:v1.1.0
```

In the configuration file, ensure that:

* `channel.inet` is set to `0.0.0.0:1491` (this lets Sonic Channel be reached from outside the container)
* `store.kv.path` is set to `/var/lib/sonic/store/kv/` (this lets the external KV store directory be reached by Sonic)
* `store.fst.path` is set to `/var/lib/sonic/store/fst/` (this lets the external FST store directory be reached by Sonic)

Sonic Channel will be reachable from `tcp://localhost:1491`.

### Configuration

Use the sample [config.cfg](https://github.com/valeriansaliou/sonic/blob/master/config.cfg) configuration file and adjust it to your own environment.

**Available configuration options are commented below, with allowed values:**

**[server]**

* `log_level` (type: _string_, allowed: `debug`, `info`, `warn`, `error`, default: `error`) — Verbosity of logging, set it to `error` in production

**[channel]**

* `inet` (type: _string_, allowed: IPv4 / IPv6 + port, default: `[::1]:1491`) — Host and TCP port Sonic Channel should listen on
* `tcp_timeout` (type: _integer_, allowed: seconds, default: `300`) — Timeout of idle/dead client connections to Sonic Channel
* `auth_password` (type: _string_, allowed: password values, default: none) — Authentication password required to connect to the channel (optional but recommended)

**[channel.search]**

* `query_limit_default` (type: _integer_, allowed: numbers, default: `10`) — Default search results limit for a query command (if the LIMIT command modifier is not used when issuing a QUERY command)
* `query_limit_maximum` (type: _integer_, allowed: numbers, default: `100`) — Maximum search results limit for a query command (if the LIMIT command modifier is being used when issuing a QUERY command)
* `query_alternates_try` (type: _integer_, allowed: numbers, default: `4`) — Number of alternate words that look like query word to try if there are not enough query results (if zero, no alternate will be tried; if too high there may be a noticeable performance penalty)
* `suggest_limit_default` (type: _integer_, allowed: numbers, default: `5`) — Default suggested words limit for a suggest command (if the LIMIT command modifier is not used when issuing a SUGGEST command)
* `suggest_limit_maximum` (type: _integer_, allowed: numbers, default: `20`) — Maximum suggested words limit for a suggest command (if the LIMIT command modifier is being used when issuing a SUGGEST command)

**[store]**

**[store.kv]**

* `path` (type: _string_, allowed: UNIX path, default: `./data/store/kv/`) — Path to the Key-Value database store
* `retain_word_objects` (type: _integer_, allowed: numbers, default: `1000`) — Maximum number of objects a given word in the index can be linked to (older objects are cleared using a sliding window)

**[store.kv.pool]**

* `inactive_after` (type: _integer_, allowed: seconds, default: `1800`) — Time after which a cached database is considered inactive and can be closed (if it is not used, ie. re-activated)

**[store.kv.database]**

* `compress` (type: _boolean_, allowed: `true`, `false`, default: `true`) — Whether to compress database or not (uses LZ4)
* `parallelism` (type: _integer_, allowed: numbers, default: `2`) — Limit on the number of compaction and flush threads that can run at the same time
* `max_files` (type: _integer_, allowed: numbers, default: `100`) — Maximum number of database files kept open at the same time per-database (this should be balanced)
* `max_compactions` (type: _integer_, allowed: numbers, default: `1`) — Limit on the number of concurrent database compaction jobs
* `max_flushes` (type: _integer_, allowed: numbers, default: `1`) — Limit on the number of concurrent database flush jobs

**[store.fst]**

* `path` (type: _string_, allowed: UNIX path, default: `./data/store/fst/`) — Path to the Finite-State Transducer database store

**[store.fst.pool]**

* `inactive_after` (type: _integer_, allowed: seconds, default: `300`) — Time after which a cached graph is considered inactive and can be closed (if it is not used, ie. re-activated)

**[store.fst.graph]**

* `consolidate_after` (type: _integer_, allowed: seconds, default: `180`) — Time after which a graph that has pending updates should be consolidated (increase this delay if you encounter high-CPU usage issues when a consolidation task kicks-in; this value should be lower than `store.fst.pool.inactive_after`)

### Run Sonic

Sonic can be run as such:

`./sonic -c /path/to/config.cfg`

## Perform searches and manage objects

Both searches and object management (ie. data ingestion) is handled via the Sonic Channel protocol only. As we want to keep things simple with Sonic (similarly to how Redis does), connecting to Sonic Channel is the way to go when you need to interact with the Sonic search database.

Sonic distributes official libraries, that let you integrate Sonic to your apps easily. Click on a library below to see library integration documentation and code.

_If you are looking for details on the raw Sonic Channel TCP-based protocol, you can read our [detailed protocol documentation](https://github.com/valeriansaliou/sonic/blob/master/PROTOCOL.md). It can prove handy if you are looking to code your own Sonic Channel library._

### 📦 Sonic Channel Libraries

Sonic distributes official Sonic Channel bindings for your programming language:

* **NodeJS**: **[node-sonic-channel](https://www.npmjs.com/package/sonic-channel)**

👉 Cannot find the library for your programming language? Build your own and be referenced here! ([contact me](https://valeriansaliou.name/))

## Which text languages are supported?

Sonic supports a wide range of languages in its lexing system. If a language is not in this list, you will still be able to push this language to the search index, but stop-words will not be eluded, which could lead to lower-quality search results.

**The languages supported by the lexing system are:**

* 🇿🇦 Afrikaans
* 🇸🇦 Arabic
* 🇦🇿 Azerbaijani
* 🇧🇩 Bengali
* 🇧🇬 Bulgarian
* 🇲🇲 Burmese
* 🇨🇳 Chinese (Mandarin)
* 🇭🇷 Croatian
* 🇨🇿 Czech
* 🇩🇰 Danish
* 🇳🇱 Dutch
* 🇺🇸 English
* 🏳 Esperanto
* 🇪🇪 Estonian
* 🇫🇮 Finnish
* 🇫🇷 French
* 🇩🇪 German
* 🇬🇷 Greek
* 🇳🇬 Hausa
* 🇮🇱 Hebrew
* 🇮🇳 Hindi
* 🇭🇺 Hungarian
* 🇮🇩 Indonesian
* 🇮🇹 Italian
* 🇯🇵 Japanese
* 🇰🇭 Khmer
* 🇰🇷 Korean
* 🏳 Kurdish
* 🇱🇻 Latvian
* 🇱🇹 Lithuanian
* 🇮🇳 Marathi
* 🇳🇵 Nepali
* 🇮🇷 Persian
* 🇵🇱 Polish
* 🇵🇹 Portuguese
* 🇮🇳 Punjabi
* 🇷🇺 Russian
* 🇸🇮 Slovene
* 🇸🇴 Somali
* 🇪🇸 Spanish
* 🇸🇪 Swedish
* 🇵🇭 Tagalog
* 🇮🇳 Tamil
* 🇹🇭 Thai
* 🇹🇷 Turkish
* 🇺🇦 Ukrainian
* 🇵🇰 Urdu
* 🇻🇳 Vietnamese
* 🇮🇱 Yiddish
* 🇳🇬 Yoruba
* 🇿🇦 Zulu

## How fast & lightweight is it?

Sonic was built for [Crisp](https://crisp.chat/) from the start. As Crisp was growing and indexing more and more search data into a full-text search SQL database, we decided it was time to switch to a proper search backend system. When reviewing Elasticsearch (ELS) and others, we found those were full-featured heavyweight systems that did not scale well with Crisp's freemium-based cost structure.

At the end, we decided to build our own search backend, designed to be simple and lightweight on resources.

You can run function-level benchmarks with the command: `cargo bench --features benchmark`

### 👩‍🔬 Benchmark #1

#### ➡️ Scenario

We performed an extract of all messages from the Crisp team used for [Crisp](https://crisp.chat/) own customer support.

We want to import all those messages into a clean Sonic instance, and then perform searches on the index we built. We will measure the time that Sonic spent executing each operation (ie. each `PUSH` and `QUERY` commands over Sonic Channel), and group results per 1,000 operations (this outputs a mean time per 1,000 operations).

#### ➡️ Context

**Our benchmark is ran on the following computer:**

* **Device**: MacBook Pro (Retina, 15-inch, Mid 2014)
* **OS**: MacOS 10.14.3
* **Disk**: 512GB SSD (formatted under the AFS file system)
* **CPU**: 2.5 GHz Intel Core i7
* **RAM**: 16 GB 1600 MHz DDR3

**Sonic is compiled as following:**

* **Sonic version**: 1.0.1
* **Rustc version**: `rustc 1.35.0-nightly (719b0d984 2019-03-13)`
* **Compiler flags**: `release` profile (`-03` with `lto`)

**Our dataset is as such:**

* **Number of objects**: ~1,000,000 messages
* **Total size**: ~100MB of raw message text (this does not account for identifiers and other metas)

#### ➡️ Scripts

**The scripts we used to perform the benchmark are:**

1. **PUSH script**: [sonic-benchmark_batch-push.js](https://gist.github.com/valeriansaliou/e5ab737b28601ebd70483f904d21aa09)
2. **QUERY script**: [sonic-benchmark_batch-query.js](https://gist.github.com/valeriansaliou/3ef8315d7282bd173c2cb9eba64fa739)

#### ⏬ Results

**Our findings:**

* We imported ~1,000,000 messages of dynamic length (some very long, eg. emails);
* Once imported, the search index weights 20MB (KV) + 1.4MB (FST) on disk;
* CPU usage during import averaged 75% of a single CPU core;
* RAM usage for the Sonic process peaked at 28MB during our benchmark;
* We used a single Sonic Channel TCP connection, which limits the import to a single thread (we could have load-balanced this across as many Sonic Channel connections as there are CPUs);
* We get an import RPS approaching 4,000 operations per second (per thread);
* We get a search query RPS approaching 1,000 operations per second (per thread);
* On the hyper-threaded 4-cores CPU used, we could have parallelized operations to 8 virtual cores, thus theoretically increasing the import RPS to 32,000 operations / second, while the search query RPS would be increased to 8,000 operations / second (we may be SSD-bound at some point though);

**Compared results per operation (on a single object):**

We took a sample of 8 results from our batched operations, which produced a total of 1,000 results (1,000,000 items, with 1,000 items batched per measurement report).

_This is not very scientific, but it should give you a clear idea of Sonic performances._

**Time spent per operation:**

Operation | Average | Best  | Worst
--------- | ------- | ----- | -----
PUSH      | 275μs   | 190μs | 363μs
QUERY     | 880μs   | 852μs | 1ms

**Batch PUSH results as seen from our terminal (from initial index of: 0 objects):**

![Batch PUSH benchmark](https://valeriansaliou.github.io/sonic/images/benchmark-batch-push.png)

**Batch QUERY results as seen from our terminal (on index of: 1,000,000 objects):**

![Batch QUERY benchmark](https://valeriansaliou.github.io/sonic/images/benchmark-batch-query.png)

## :fire: Report A Vulnerability

If you find a vulnerability in Sonic, you are more than welcome to report it directly to [@valeriansaliou](https://github.com/valeriansaliou) by sending an encrypted email to [valerian@valeriansaliou.name](mailto:valerian@valeriansaliou.name). Do not report vulnerabilities in public GitHub issues, as they may be exploited by malicious people to target production servers running an unpatched Sonic instance.

**:warning: You must encrypt your email using [@valeriansaliou](https://github.com/valeriansaliou) GPG public key: [:key:valeriansaliou.gpg.pub.asc](https://valeriansaliou.name/files/keys/valeriansaliou.gpg.pub.asc).**

**:gift: Based on the severity of the vulnerability, I may offer a $200 (US) bounty to whomever reported it.**
