use clap::App;
use clap::load_yaml;
use rinex::hatanaka;
use std::str::FromStr;
use std::io::{Write, BufRead, BufReader, Error};

fn main() -> Result<(), Error> {
    let yaml = load_yaml!("cli.yml");
    let mut app = App::from_yaml(yaml);

    let matches = app.get_matches();

    let filepath = matches.value_of("filepath")
        .unwrap();
    
    let crx2rnx = matches.is_present("crx2rnx");

    if crx2rnx {
        println!("Decompressing \"{}\"", filepath);

        let mut line = String::new();
        let input = std::fs::File::open(filepath)?;
        let mut buffer = BufReader::new(input);
        let mut output = std::fs::File::create("output.rnx")?;

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
        let rnx_version = line.split_at(20).0;
        let rnx_version : u8 = f32::from_str(rnx_version.trim()).unwrap() as u8;

        let mut inside_body = false;

        for line in buffer.lines() {
            let l = line.unwrap();

            if inside_body {

            } else {
                // still inside header,
                // straight copy..
                writeln!(output, "{}", l).unwrap();
                inside_body = l.contains("END OF HEADER");
                if (inside_body) {
                    println!("End of RINEX header.\nStarting record decompression..")
                }
            }
        }
    }
    println!("DONE");
    Ok(())
}
