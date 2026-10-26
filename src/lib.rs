//! crawlcore: the shared crawl library of the astrx suite.
//!
//! Pure, dependency-free building blocks used by every engine: a ReDoS-safe
//! robots path-glob matcher, SimHash near-duplicate bit math, recrawl
//! scheduling arithmetic, stateless structural bot-trap predicates, shared
//! hashing (SHA-1/SHA-256/MD5 and BLAKE2b), and a DEFLATE/gzip/zlib inflater.
//!
//! It also holds the pieces that are easy to get wrong when written more than
//! once: [`budget::Budget`], which makes a length from the wire impossible to
//! overflow past a cap; [`atomicfile::write_atomic`], which publishes a
//! snapshot by rename so a torn write can't destroy the previous good one; and
//! [`http`], the HTTP/1.1 client wire layer `websearch` and `onioncrawler`
//! share. Ported from the Python crawlcore, with its tests carried over as the
//! spec.
//!
//! # Feature tiers
//!
//! The default build uses only the standard library, with no third-party
//! dependencies; that's the crate's contract. The one exception is the
//! *streaming* half of [`http`], which needs `tokio::io::AsyncRead` to read a
//! body off a socket; it sits behind the opt-in `net` feature
//! (`net = ["dep:tokio"]`), like the engines' net tiers, so
//! `cargo build -p crawlcore` still pulls in nothing.
#![forbid(unsafe_code)]

pub mod atomicfile;
pub mod blake2b;
pub mod budget;
pub mod dedup;
pub mod globmatch;
pub mod hash;
pub mod http;
pub mod inflate;
pub mod json;
pub mod logfmt;
pub mod metrics;
pub mod scheduler;
pub mod segstore;
pub mod traps;
pub mod urlparse;

pub mod storefile;
