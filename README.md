# crawlcore

Shared crawl library: robots glob, SimHash dedup, recrawl scheduling, trap predicates, hashing (SHA-1/256, MD5, BLAKE2b), INFLATE, HTTP/1.1 wire layer.

Part of the [astrx suite](https://github.com/Vxmdzhb/astrx-suite): the shared library its engines are built on. The suite repository assembles every engine into the single `astrx` binary (`astrx doctor`) and deploys it.

## Build

    cargo build                  # the pure library: no third-party crates
    cargo test --all-features

Features: `net`. The default build uses the standard library only; `net` (and `rand`) add the suite's two vetted dependencies, tokio and getrandom. CI asserts the default build stays first-party.

## History

This repository was split out of https://github.com/Vxmdzhb/astrx-suite with the full history of its files, each followed back through its renames. The cross-check goldens are explained in [GOLDENS.md](GOLDENS.md).
