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
cargo run -- args
```

Decompress a `CRINEX` file with `-d`

```bash
cargo run -- --filepath /tmp/data.22d -o /tmp/data.22o -d
```

This produces an "output.rnx" RINEX file.   
Use `-o` to set the output file name:

```bash
cargo run -- -fp /tmp/data.22d -o /tmp/data.22o -d
cargo run -- -fp /tmp/data.22d --output /tmp/data.22o -d
```

## `--strict` flag for modern Observation Data

CRX2RNX seems to violate RINEX definitions 
when decompressing V > 2 (modern) RINEX Observation data, 
because decompressed epochs span more than 60 characters per line.

This tool behaves like CRX2RNX by default, but you can strictly
follow RINEX definitions with the `--strict` flag.

This only affects modern Observation data decompression

## Epoch events 

All comments contained in the `RINEX` record are
left as is. Just like `CRX2RNX`, epochs with weird events are left untouched.
Therefore, explanations on these epochs events, 
usually described in the form of `COMMENTS` are preserved. 

## Compression algorithm 

Unlikie CRX2RNX, this tool is not limited to 
an M=5 maximal compression / decompression order
in the core algorithm.   
It actually dynamically adapts and will never fail, as long
as the input content is a valid CRINEX.
