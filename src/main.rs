use clap::App;
use clap::load_yaml;
use rinex::hatanaka;
use std::str::FromStr;
use std::io::{Write, BufRead, BufReader};
use thiserror::Error;

#[derive(Error, Debug)]
enum Error {
    #[error("io error")]
    IoError(#[from] std::io::Error),
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
        let mut epoch_size : usize = 0;
        let mut epoch_krn = hatanaka::Kernel::new(1);
        let mut clock_krn = hatanaka::Kernel::new(1);

        for line in buffer.lines() {
            let l = line.unwrap();

            if body {
                if new_epoch {
                    if first_epoch {
                        // TODO this only works for V3
                        // attention a l'offset 11 ou 8 de epoch (V2/V3)
                        if !l.starts_with("> ") {
                            panic!("First epoch is faulty! Cannot intialize epoch kernel")
                        }
                        first_epoch = false;
                        epoch_size = 24; //TODO parse sat#id
                        epoch_krn.init(0, hatanaka::Dtype::Text(l))
                            .unwrap();
                    } else {
                        epoch_size = 0; //TODO parse sat#id
                        let recovered = epoch_krn.recover(hatanaka::Dtype::Text(l))
                            .as_text()
                            .unwrap();
                        writeln!(output, "{}", recovered)?
                    }
                    is_clock_offset = true
                } else {

                    if is_clock_offset {
                        is_clock_offset = false;
                    } else {
                        for _ in 0..epoch_size {

                        }
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
