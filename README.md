# RINEX Cli 
Command line tool to handle, manage and analyze RINEX files

[![crates.io](https://img.shields.io/crates/v/hatanaka.svg)](https://crates.io/crates/hatanaka)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue?style=flat-square)](https://github.com/gwbres/hatanaka/blob/main/LICENSE-APACHE)
[![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](https://github.com/gwbres/hatanaka/blob/main/LICENSE-MIT) 
[![crates.io](https://img.shields.io/crates/d/hatanaka.svg)](https://crates.io/crates/hatanaka)    
[![Rust](https://github.com/gwbres/hatanaka/actions/workflows/rust.yml/badge.svg)](https://github.com/gwbres/hatanaka/actions/workflows/rust.yml)
[![crates.io](https://docs.rs/hatanaka/badge.svg)](https://docs.rs/hatanaka/badge.svg)

RINEX compression and decompression tool.

This command line interface implements the latest 
[Rinex crate](https://crates.io/crates/rinex)
and allows easy RINEX files manipulation.

## Getting started

Run with `cargo`

```bash
cargo run
```

Set the output path with `-o` (otherwise, default name is "output"):

```bash
cargo run -- --fp /tmp/data.22d -o /tmp/data.22o -d
```

## Epoch events 

All comments contained in the `RINEX` record are
left as is. Just like `CRX2RNX`, epochs with weird events are left untouched.
Therefore, explanations on these epochs events, 
usually described in the form of `COMMENTS` are maintained.   

## Compression algorithm 

This tool follows `CRX2RNX` behavior but it is not limited
to a compression / decompression order of 5 in the algorithm.   
It actually dynamically adapts and will never fail, as long
as the input content is a valid CRINEX.

