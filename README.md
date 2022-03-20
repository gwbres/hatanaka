# Hatanaka 

[![crates.io](https://img.shields.io/crates/v/hatanaka.svg)](https://crates.io/crates/hatanaka)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue?style=flat-square)](https://github.com/gwbres/hatanaka/blob/main/LICENSE-APACHE)
[![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](https://github.com/gwbres/hatanaka/blob/main/LICENSE-MIT) 
[![crates.io](https://img.shields.io/crates/d/hatanaka.svg)](https://crates.io/crates/hatanaka)    
[![Rust](https://github.com/gwbres/hatanaka/actions/workflows/rust.yml/badge.svg)](https://github.com/gwbres/hatanaka/actions/workflows/rust.yml)
[![crates.io](https://docs.rs/hatanaka/badge.svg)](https://docs.rs/hatanaka/badge.svg)

`RINEX` file compression and decompression tool.  
This tool implements the latest `RINEX`
[crate](https://crates.io/crates/rinex)
which allows powerful interaction with these complex files.

*Yuki Hatanaka* came up with a simple yet efficient compression algorithm for
`RINEX` Data,  
latest revision is `CRINEX3` and is specified 
[here](https://www.gsi.go.jp/ENGLISH/Bulletin55.html).

For more information on the compression core algorithm,   
refer to the `hatanaka` section of the library

## Supported revisions

* [ ] CRINEX1  
* [ ] CRINEX3  

CRINEX2 does not technically exist

## Getting started

Decompress a `CRINEX` file with `-d`

```bash
cargo run -- --filepath /tmp/data.22d -d
```

This produces an "output.rnx" RINEX file.   

Use `-o` to set the output file name:

```bash
cargo run -- -fp /tmp/data.22d -o /tmp/data.22o -d
cargo run -- -fp /tmp/data.22d --output /tmp/data.22o -d
```

## Modern Observation Data and `--strict` flag 

`CRX2RNX` violates RINEX standard 
when decompressing V > 2 (modern) RINEX Observation data,   
because decompressed epochs are not wrapped and are larger than 80 characters.    

This tool behaves like `CRX2RNX` by default,  
but you can change that behavior
to follow `RINEX` definitnios strictly, by passing the `--strict` flag:

```bash
cargo run -- -fp data.22d -d -s
```

## Epoch events 

`COMMENTS` are preserved through compression / decompression, as you would expect.   
Just like `CRX2RNX`, epochs with weird events are left untouched.  
Therefore, explanations on these epochs events, 
usually described in the form of `COMMENTS` are preserved. 

## Compression algorithm 

Unlike `CRX2RNX`, this tool is not limited to 
an M=5 maximal compression / decompression order
in the core algorithm.   
It actually dynamically adapts and will never fail, as long
as the input content is a valid CRINEX.
