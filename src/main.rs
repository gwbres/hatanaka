//! hatanaka    
//! command line tool to compress RINEX files   
//! and decompress CRINEX files
use rinex::header;
use rinex::hatanaka;
use rinex::record::Sv;
use rinex::version::Version;
use rinex::constellation::Constellation;

use clap::App;
use clap::load_yaml;
use thiserror::Error;
use std::str::FromStr;
use std::collections::HashMap;
use std::io::{Write, BufRead, BufReader};

#[derive(Error, Debug)]
enum Error {
    #[error("io error")]
    IoError(#[from] std::io::Error),
    #[error("failed to parse RINEX header")]
    ParseHeaderError(#[from] header::Error),
    #[error("hatanaka error")]
    HatanakaError(#[from] hatanaka::Error),
}

fn main() -> Result<(), Error> {
    let yaml = load_yaml!("cli.yml");
    let app = App::from_yaml(yaml);
    let matches = app.get_matches();
    
    let filepath = matches.value_of("filepath")
        .unwrap();
    
    let crx2rnx = matches.is_present("crx2rnx");
    let rnx2crx = matches.is_present("rnx2crx");
    let m = u16::from_str_radix(matches.value_of("m")
        .unwrap_or("8"),10).unwrap();
    let strict_flag = matches.is_present("strict");

    let outpath : String = match crx2rnx {
        true => { //CRX2RNX
            let output = matches.value_of("output")
                .unwrap_or("output");
            let mut out = String::from(output);
            out.push_str(".rnx");
            out
        },
        _ => { // RNX2CRX
            let output = matches.value_of("output")
                .unwrap_or("output");
            let mut out = String::from(output);
            out.push_str(".crx");
            out
        },
    };
 
    let mut output = std::fs::File::create(outpath)?;
    if crx2rnx {
        decompress(filepath, m, output)?;
        println!("RINEX file extracted");
        Ok(())
    } else {
        Ok(())
    }
}

/// Decompresses given file,   
/// fp : filepath   
/// m : maximal compression order for core algorithm    
/// writer: stream
fn decompress (fp: &str, m: u16, mut writer: std::fs::File) -> Result<(), Error> {
    let mut content = String::new();
    let mut hd_content = String::new();
    let input = std::fs::File::open(fp)?;
    let reader = BufReader::new(input);
    let mut header : header::Header = header::Header::default();
    let mut decompressor = hatanaka::Decompressor::new(m.into());

    let mut first_epoch = true;
    let mut epoch_len : usize = 0;
    let mut header_parsed = false;
    let mut new_epoch = true;
    println!("Decompressing file \"{}\"", fp);
    for l in reader.lines() {
        let line = &l.unwrap();
        if !header_parsed {
            hd_content.push_str(line);
            hd_content.push_str("\n");
            if !line.contains("CRINEX VERS") && !line.contains("CRINEX PROG") {
                // strip CRINEX special header
                content.push_str(line);
                content.push_str("\n");
            }
            if line.contains("END OF HEADER") {
                // identify header section
                header = rinex::header::Header::from_str(&hd_content)?;
                println!("RINEX Header parsed");
                write!(writer, "{}", content)?;
                // reset for record section
                content.clear();
                header_parsed = true;
            }
        } else { // RINEX record
            let mut content : String = String::from(line);
            if content.len() == 0 {
                content = String::from(" ");
            }
            let recovered = decompressor.recover(&header, &content)?;
            write!(writer, "{}", recovered)?
        }
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use assert_cmd::prelude::*;
    use predicates::prelude::*;
    use std::process::Command;
    /// Runs `diff` to determines whether f1 & f2 
    /// are strictly identical or not
    fn diff_is_strictly_identical (f1: &str, f2: &str) -> Result<bool, std::string::FromUtf8Error> {
        let output = Command::new("diff")
            .arg("-q")
            .arg("-Z")
            .arg(f1)
            .arg(f2)
            .output()
            .expect("failed to execute \"diff\"");
        let output = String::from_utf8(output.stdout)?;
        println!("OUPTUT: {}", output);
        Ok(output.len()==0)
    }
    #[test]
    /// Tests CRINEX3 decompression
    fn test_decompression_v3()  -> Result<(), Box<dyn std::error::Error>> { 
        let testpool = env!("CARGO_MANIFEST_DIR").to_owned() + "/data/V3";
        let path = std::path::PathBuf::from(testpool.to_owned());
        for e in std::fs::read_dir(path).unwrap() {
            let entry = e.unwrap();
            let path = entry.path();
            let full_path = &path.to_str()
                .unwrap();
            let is_hidden = entry.file_name()
                .to_str()
                .unwrap()
                .starts_with(".");
            let is_crinex = entry.file_name()
                .to_str()
                .unwrap()
                .ends_with(".crx");
            if !path.is_dir() && !is_hidden && is_crinex {
                let compare = full_path.strip_suffix(".crx").unwrap();
                let compare = compare.to_owned() +".rnx";
                let mut cmd = Command::cargo_bin("hatanaka")?;
                cmd.arg("-d")
                   .arg("--filepath")
                   .arg(&path);
                cmd.assert()
                   .success();
                let diff = diff_is_strictly_identical("output.rnx", &compare)
                    .unwrap(); 
                assert_eq!(diff,true)
            }
        }
        Ok(())
    }
}
