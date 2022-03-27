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
* [x] CRINEX3  

CRINEX2 does not technically exist

## Getting started

Decompress a `CRINEX` file with `-d`

```bash
cargo run -- -d --filepath /tmp/data.22d
```

This produces an "output.rnx" RINEX file.   

Use `-o` to set the output file name:

```bash
cargo run -- -d --filepath /tmp/data.22d -o /tmp/myfile
cargo run -- -d --filepath /tmp/data.22d --output /tmp/custom
```

## Modern Observation Data and `--strict` flag 

`CRX2RNX` violates RINEX standard 
when decompressing V > 2 (modern) RINEX Observation data,   
because decompressed epochs are not contrainted to 80 characters.    

## Epoch events 

`COMMENTS` are preserved through compression / decompression, as you would expect.   
Just like `CRX2RNX`, epochs with weird events are left untouched.  
Therefore, explanations on these epochs events, 
usually described in the form of `COMMENTS` are preserved. 

## Compression behavior & limitations 

This tool uses an M=8 maximal compression order,   
which should be fine for all CRINEX ever produced,   
considering they were probably produced with `CRX2RNX`   
which hardcodes an M=5 limitation.   

Unlike `CRX2RNX`, this tool is not limited to M,   
you can increase the default value if you think "higher"
compression will be encountered in a given file: 
```bash
cargo run -- -d -m 8 --filepath /tmp/data.22d
```

Best compression performances seem to be obtained for m=4  
which is handled by default.
