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
        decompress(filepath, output)
    } else {
        Ok(())
    }
}

fn decompress (fp: &str, mut writer: std::fs::File) -> Result<(), Error> {
    let mut content = String::new();
    let mut hd_content = String::new();
    let input = std::fs::File::open(fp)?;
    let reader = BufReader::new(input);
    let mut header : header::RinexHeader = header::RinexHeader::default();
    let mut decompressor = hatanaka::Decompressor::new(8); // TODO maximal order

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
                header = rinex::header::RinexHeader::from_str(&hd_content)?;
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
