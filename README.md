# Hatanaka 

[![crates.io](https://img.shields.io/crates/v/hatanaka.svg)](https://crates.io/crates/hatanaka)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue?style=flat-square)](https://github.com/gwbres/hatanaka/blob/main/LICENSE-APACHE)
[![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](https://github.com/gwbres/hatanaka/blob/main/LICENSE-MIT) 
[![crates.io](https://img.shields.io/crates/d/hatanaka.svg)](https://crates.io/crates/hatanaka)    
[![Rust](https://github.com/gwbres/hatanaka/actions/workflows/rust.yml/badge.svg)](https://github.com/gwbres/hatanaka/actions/workflows/rust.yml)
[![crates.io](https://docs.rs/hatanaka/badge.svg)](https://docs.rs/hatanaka/badge.svg)

`RINEX` file compression and decompression tool.  

This tool is based on the 
[RINEX crate](https://crates.io/crates/rinex).

*Yuki Hatanaka* created a simple yet efficient method to compress
RINEX files, it's called CRINEX,   
latest revision is `CRINEX3` and is specified 
[here](https://www.gsi.go.jp/ENGLISH/Bulletin55.html).

For more information on the actual compression algorithm, refer to the [hatanaka section](https://crates.io/crates/rinex)
of the library.

## Supported revisions

* [ ] CRINEX1 
* [x] CRINEX3  

CRINEX2 was never released

## CRINEX

RINEX Compression is an algorithm designed for Observation Data RINEX.

## Getting started

Decompress a `CRINEX` file with `-d`

```bash
cargo run -- -d --filepath /tmp/data.22d
```

To change the default output file `output.rnx`, use the `-o` flag :

```bash
cargo run -- -d --filepath /tmp/data.22d -o /tmp/myfile
cargo run -- -d --filepath /tmp/data.22d --output /tmp/custom
```

### `--strict` flag for modern OBS Data

`CRX2RNX` violates RINEX standard 
when decompressing V > 2 (modern) RINEX Observation data,   
because decompressed epochs are not contrainted to 80 characters.    

By default and at the moment, this tool behaves like `CRX2RNX`.  
But the next release will propose a flag to change that behavior and
strictly follow RINEX specifications:

```bash
cargo run -- -d -s --filepath data/V3/KUNZ00CZE.cnx
```

This flag has no impact when manipulating an old RINEX files.

## Epoch events 

`COMMENTS` are preserved through compression / decompression, as you would expect.   
Just like `CRX2RNX`, epochs with special events (flag > 2) are left untouched.  
Therefore, explanations on these epochs events are preserved.

## Compression algorithm & limitations 

This tool uses an M=8 maximal compression order, which should be fine for all CRINEX ever produced,   
considering they were probably produced by `CRX2RNX` which hardcodes an M=5 limitation.   

Unlike `CRX2RNX`, this tool is not limited to an hardcoded M value, 
you can increase the default value if you think higher   
compression will be encountered in a given file: 
```bash
cargo run -- -d -M 8 --filepath data/V3/KUNZ00CZE.cnx
```

According to Y. Hatanaka's publication, optimum compression performances are obtained for a 4th order compression,   
which is handled with default parameters.
