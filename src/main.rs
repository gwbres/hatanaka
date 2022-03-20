//! hatanaka main.rs    
//! command line tool to compress RINEX files   
//! and decompress CRINEX files
use clap::App;
use clap::load_yaml;
use rinex::hatanaka;
use thiserror::Error;
use std::str::FromStr;
use std::io::{Write, BufRead, BufReader};

#[derive(Error, Debug)]
enum Error {
    #[error("io error")]
    IoError(#[from] std::io::Error),
    #[error("failed to parse # epochs")]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error("hatanaka error")]
    HatanakaError(#[from] rinex::hatanaka::Error),
}

fn main() -> Result<(), Error> {
    let yaml = load_yaml!("cli.yml");
    let app = App::from_yaml(yaml);

    let matches = app.get_matches();

    let filepath = matches.value_of("filepath")
        .unwrap();
    let mut outpath = String::from(matches.value_of("output")
        .unwrap_or("output"));
    
    let crx2rnx = matches.is_present("crx2rnx");
    let rnx2crx = matches.is_present("rnx2crx");

    if crx2rnx {
        outpath.push_str(".rnx");
        let mut output = std::fs::File::create("output.rnx")?;
        println!("Decompressing \"{}\"", filepath);

        let mut line = String::new();
        let input = std::fs::File::open(filepath)?;
        let mut buffer = BufReader::new(input);

        buffer.read_line(&mut line)?;
        if !line.contains("COMPACT RINEX FORMAT") {
            panic!("this is not a valid CRINEX");
        }
        let crx_version = line.split_at(20).0;
        let crx_version : u8 = f32::from_str(crx_version.trim()).unwrap() as u8;
        
        buffer.read_line(&mut line)?;
        buffer.read_line(&mut line)?;
        if !line.contains("RINEX VERSION / TYPE") {
            panic!("this is not a valid RINEX");
        }
        writeln!(output, "{}", line)?;
        let rnx_version = line.split_at(20).0;
        let rnx_version : u8 = f32::from_str(rnx_version.trim()).unwrap() as u8;

        let mut body = false;
        let mut new_epoch = true;
        let mut is_clock_offset = true;
        let mut first_epoch = true;
        let mut epoch_flag : u8 = 0;
        let mut epoch_count: u8 = 0;
        let mut epoch_size : u8 = 0;
        // text differentiator
        let mut epoch_krn = hatanaka::Kernel::new(1); 
        // numerical differentiators
        let mut clock_krn = hatanaka::Kernel::new(8); 

        for line in buffer.lines() {
            let l = &line.unwrap();
            if body {
                if rinex::is_comment!(l) {
                    // special comment case:
                    // leave as is !
                    writeln!(output, "{}", l).unwrap();
                    continue
                }
                if new_epoch {
                    if first_epoch {
                        // TODO this only works for V3
                        // attention a l'offset 11 ou 8 de epoch (V2/V3)
                        
                        // init epoch kernel
                        epoch_krn.init(0, hatanaka::Dtype::Text(l.to_string()))
                            .unwrap();
                        // parse system infos
                        match crx_version {
                            1 | 2 => { // old CRINEX
                                if !l.starts_with("&") {
                                    panic!("1st epoch does not match CRINEX1 standard")
                                }
                            },
                            _ => { // modern CRINEX
                                if !l.starts_with("> ") {
                                    panic!("1st epoch does not match CRINEX3 standard")
                                }
                            },
                        }
                        
                        epoch_size = 24; //TODO parse sat#id
                        first_epoch = false;
                    } 
                    // identify # of epochs to be parsed
                    let recovered = epoch_krn.recover(hatanaka::Dtype::Text(l.to_string()))
                        .unwrap();
                    let recovered = recovered.as_text()
                        .unwrap();
                    println!("RECOVERED \"{}\"", recovered);
                    let mut offset :usize = 
                        2  // Y
                        +2+1 // m
                        +2+1 // d
                        +2+1 // h
                        +2+1 // m
                        +11  // s
                        +1;  // ">" or "&" init. marker
                    if crx_version > 2 {
                        offset += 2  // Y is 4 digit
                    }
                    if recovered.starts_with("> ") {
                        offset += 1 // CRINEX3 "> " marker
                    }
                    let (_, rem) = &recovered.split_at(offset);
                    let (e_flag, rem) = rem.split_at(3);
                    epoch_flag = u8::from_str_radix(e_flag.trim(), 10)?;
                    let (n, _) = rem.split_at(3);
                    epoch_size = u8::from_str_radix(n.trim(), 10)?; 
                    new_epoch = false;
                    is_clock_offset = true

                } else if is_clock_offset {
                    if l.contains("&") { // kernel re-init
                        let (n, rem) = l.split_at(1);
                        let n = u8::from_str_radix(n, 10)?;
                        let (_, num) = rem.split_at(1);
                        let num = i64::from_str_radix(num, 10)?;
                        clock_krn.init(n.into(), hatanaka::Dtype::Numerical(num))
                            .unwrap()
                    } else {
                        let num = i64::from_str_radix(l, 10)?;
                        let recovered = clock_krn.recover(hatanaka::Dtype::Numerical(num))
                            .unwrap();
                        let recovered = recovered.as_numerical()
                            .unwrap();
                        println!("RECOVERED CLOCK OFFSET {}", recovered);
                    }
                    is_clock_offset = false;
                
                } else { // epoch parsing
                    // epoch flag > 2 is left untouched
                    // because a comment is usually attached to it
                    // match on epoch_flag
                    // epoch_flag > 2 => leave this epoch as is!
                    // anEd add following content
                    epoch_count += 1;
                    if epoch_count == epoch_size {
                        epoch_count = 0;
                        new_epoch = true
                    }
                }
            } else {
                // still inside header,
                writeln!(output, "{}", l).unwrap(); // straight copy..
                body = l.contains("END OF HEADER");
                if body {
                    println!("End of RINEX header.\nStarting record decompression..")
                }
            }
        }
    }

    if rnx2crx {
        //TODO
    }

    Ok(())
}
