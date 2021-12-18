extern crate icns;

use icns::{IconFamily, IconType, OSType};
use std::env::args;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::process::exit;
use std::str::FromStr;

fn main() {

    // Read arguments
    let args: Vec<String> = args().collect();
    if args.len() != 3 {
        // Print program usage
        println!(r#"Usage:
    icnsutil <path to file.icns> <icon type>"#);
        exit(1);
    }

    // Read in ICNS and load icon family
    println!("[-] Reading input file \"{}\"...", args.get(1).unwrap());
    let file = BufReader::new(
        File::open(
            args.get(1).unwrap()
        ).unwrap()
    );
    let icon_family = IconFamily::read(file).unwrap();

    // Get icon type from string
    let icon_type = IconType::from_ostype(
        OSType::from_str(
            args.get(2).unwrap().as_str()
        ).unwrap()
    ).unwrap();

    // Extract PNG from icon family
    println!("[-] Extracting {:?} from icon family...", icon_type);
    let image = icon_family.get_icon_with_type(icon_type).unwrap();

    // Create output buffer
    println!("[-] Creating output buffer...");
    let output_filename = args.get(1).unwrap().replace(".icns", ".png");
    let output = BufWriter::new(
        File::create(&output_filename).unwrap()
    );

    // Write output PNG
    println!("[-] Writing output file \"{}\"...", &output_filename);
    image.write_png(output).unwrap();

    println!("[+] Done!");
}
