//! hatanaka    
//! command line tool to compress RINEX files   
//! and decompress CRINEX files
use rinex::header;
use rinex::record::Sv;
use rinex::version::Version;
use rinex::hatanaka::{Kernel,Dtype};
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
    #[error("this is not a CRINEX file")]
    NotACrinexError,
    #[error("only OBS RINEX supported")]
    NotObsRinexData,
    #[error("failed to parse RINEX header")]
    ParseHeaderError(#[from] rinex::header::Error),
    #[error("failed to parse sat. vehicule")]
    ParseSvError(#[from] rinex::record::ParseSvError),
    #[error("failed to parse integer number")]
    ParseIntError(#[from] std::num::ParseIntError),
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
    let input = std::fs::File::open(fp)?;
    let reader = BufReader::new(input);
    let mut header : header::RinexHeader = header::RinexHeader::default();
    let mut crx_version = Version::default(); 
    let mut rnx_version = Version::default(); 
    let mut obs_codes : HashMap<Constellation, Vec<String>> = HashMap::new();
    // textdiff 
    let mut epoch_krn = Kernel::new(1);
    let mut recovered_epoch : String = String::from("");
    // numerical differentiators
    let mut clock_krn = Kernel::new(8);
    let mut obs_krn : HashMap<Sv, Vec<Kernel>> = HashMap::new();

    let mut header_parsed = false;
    let mut new_epoch = true;
    let mut first_epoch = true;
    let mut is_clock_offset = true;
    let mut epoch_flag : u8 = 0;
    let mut epoch_size : u16 = 0;
    let mut epoch_count : u16 = 0;
    
    println!("Decompressing file \"{}\"", fp);

    for l in reader.lines() {
        let line = &l.unwrap();
        if rinex::is_comment!(line) { // COMMMENT special case
            // --> always preserve
            writeln!(writer, "{}", line)?;
            continue
        }
        if !header_parsed {
            if !line.contains("CRINEX VERS") {
                if !line.contains("CRINEX PROG") {
                    writeln!(writer, "{}", line)? // get rid of CRINEX header
                }
            }
            content.push_str(line);
            content.push_str("\n");
            if line.contains("END OF HEADER") {
                header = rinex::header::RinexHeader::from_str(&content)?;
                if !header.is_crinex() {
                    return Err(Error::NotACrinexError)
                }
                crx_version = header.crinex.unwrap().version.clone();
                if header.rinex_type != rinex::Type::ObservationData {
                    return Err(Error::NotObsRinexData)
                }
                rnx_version = header.version.clone();
                obs_codes = header.obs_codes.unwrap().clone();
                println!("RINEX header identified");
                header_parsed = true
            }
        } else { // RINEX record
            if new_epoch {
                if first_epoch {
                    epoch_krn.init(0, Dtype::Text(line.to_string()))
                    .unwrap();
                    match crx_version.major { 
                        1 => { // old CRINEX
                            if !line.starts_with("&") {
                                panic!("first epoch does not match CRINEX1 standard")
                            }
                        },
                        3 => { // CRINEX3
                            if !line.starts_with("> ") {
                                panic!("first epoch does not match CRINEX3 standard")
                            }
                        },
                        _ => panic!("non supported CRINEX revision")
                    }
                    first_epoch = false
                }
                // identify # of epochs to be parsed
                let recovered = epoch_krn.recover(Dtype::Text(line.to_string()))
                    .unwrap();
                recovered_epoch = recovered.as_text()
                    .unwrap();
                let mut offset : usize =
                    2    // Y
                    +2+1 // m
                    +2+1 // d
                    +2+1 // h
                    +2+1 // m
                    +11  // s
                    +1;  // ">" or "&" init marker
                if rnx_version.major > 2 {
                    offset += 2; // Y is 4 digit
                }
                if recovered_epoch.starts_with("> ") {
                    offset += 1 // CRINEX3 has 1 extra whitespace
                }
                let (_, rem) = &recovered_epoch.split_at(offset);
                let (e_flag, rem) = rem.split_at(3);
                let (n, _) = rem.split_at(3);
                //println!("RECOVERED \"{}\"", recovered_epoch.trim());
                epoch_flag = u8::from_str_radix(e_flag.trim(), 10)?;
                epoch_size = u16::from_str_radix(n.trim(), 10)?;
                new_epoch = false;
                is_clock_offset = true
            } else if is_clock_offset && epoch_flag < 2 {
                let clock_offset : Option<i64> = match line.contains("&") { 
                    true => {
                        // kernel (re)init
                        let (n, rem) = line.split_at(1);
                        let n = u8::from_str_radix(n, 10)?;
                        let (_, num) = rem.split_at(1);
                        let num = i64::from_str_radix(num, 10)?;
                        clock_krn.init(n.into(), Dtype::Numerical(num))
                            .unwrap();
                        Some(num)
                    },
                    false => {
                        if let Ok(num) = i64::from_str_radix(line.trim(), 10) {
                            Some(num)
                        } else {
                            None
                        }
                    },
                };
                // output recovered epoch
                // TODO squeeze recovered clk offset
                let epo = recovered_epoch.as_str();
                let (epo, systems) = epo.split_at(35);
                writeln!(writer, "{}", epo)?;
                match rnx_version.major {
                    1 | 2 => { // Old RINEX
                        //TODO wrap system#id on as many lines as needed
                        // relire src/observation.rs pour voir # max par line
                    },
                    _ => { // Modern RINEX
                        if let Some(clk) = clock_offset {
                            writeln!(writer, "         {}", (clk as f64)/1000.0_f64)?
                        }
                    },
                }
                is_clock_offset = false
            } else {
                epoch_count += 1;
                if epoch_flag > 2 { // epoch event!
                    // maintain content as is
                    writeln!(writer, "{}", line)?;
                    if epoch_count == epoch_size {
                        epoch_count = 0;
                        new_epoch = true
                    }
                    continue
                }
                
                let epo = recovered_epoch.as_str();
                let offset : usize = std::cmp::min((41 + 3*epoch_count).into(), epo.len());
                let system = epo.split_at(offset.into()).0;
                let system = system.split_at(system.len()-3).1; // grab last XXX
                let sv = Sv::from_str(system)?;
                let codes = &obs_codes[&sv.constellation];
                println!("SYSTEM: \"{}\"", system); 
                // creates kernel for later use
                if !obs_krn.contains_key(&sv) {
                    // first init for this system obs code
                    let mut map : Vec<Kernel> = Vec::with_capacity(12);
                    for code in codes { // one per OBS code
                        map.push(Kernel::new(8))
                    }
                    obs_krn.insert(sv, map);
                }
            
                let mut obs_count : usize = 0;
                let mut content = String::new();
                let mut rem = line.as_str();
                
                loop { // retrieves all observation data + flags
                    if obs_count == codes.len() {
                        println!("FLAGS! \"{}\"", rem);
                    }
                    // from that given epoch line
                    let next_whitespace = match rem.find(' ') {
                        Some(offset) => offset+1,
                        None => break, //EOL
                    };
                    let (roi, r) = rem.split_at(next_whitespace);
                    rem = r;
                    println!("CODE : \"{}\" - ROI \"{}\"", codes[obs_count], roi);
                    if roi == " " {
                        obs_count += 1;
                        continue
                    }
                    let (init_order, data) : (Option<u16>, i64) = match roi.contains('&') {
                        false => {
                            (None, i64::from_str_radix(roi.trim(),10)?)
                        },
                        true => {
                            let (init_order, remainder) = roi.split_at(1);
                            let (_, data) = remainder.split_at(1);
                            (Some(u16::from_str_radix(init_order,10)?),
                            i64::from_str_radix(data.trim(),10)?)
                        },
                    };
                    if let Some(order) = init_order {
                        // (re)init related kernel
                        /*let mut kernel = &mut obs_krn[&sv][obs_count];
                        kernel
                            .init(
                                order.into(),
                                Dtype::Numerical(data))
                                .unwrap()*/
                    }
                    obs_count += 1;
                }
                // convert this epoch content back to RINEX
                /*match rnx_version.major {
                    1 | 2 => { // Old RINEX
                    },
                    _ => { // Modern RINEX
                    }
                }*/
                epoch_count += 1;
                if epoch_count == epoch_size {
                    epoch_count = 0;
                    new_epoch = true
                }
            }
        }
    }
    Ok(())
}
