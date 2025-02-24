extern crate flate2;

use flate2::write::GzEncoder; // <- enxoding purpose
use flate2::Compression; // <- for copmressoin purpose
use std::env::args; // <- taking a file name
use std::fs::File; //<- taking a file
use std::io::copy; //
use std::io::BufReader;
use std::time::Instant;

fn main() {
    if args().len() != 3 {
        eprint!("Usage: `source`, target");
        return;
    }

    let mut input = BufReader::new(File::open(args().nth(1).unwrap()).unwrap());
    let output = File::create(args().nth(2).unwrap()).unwrap();
    let mut encoder = GzEncoder::new(output, Compression::default());
    let start = Instant::now();
    copy(&mut input, &mut encoder).unwrap();
    let output = encoder.finish().unwrap();
    println!(
        "Source len: {:?}",
        input.get_ref().metadata().unwrap().len()
    );

    println!("Target len: {:?}", output.metadata().unwrap().len());

    println!("Elapsed: {:?}", start.elapsed());
}
