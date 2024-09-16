use flate2::write::GzEncoder;
use flate2::Compression;
use std::io::BufReader;
use std::fs::File;
use std::time::Instant;
use std::io::copy;

pub fn compress() {
    let mut input = BufReader::new(File::open("Cover.pdf").unwrap());
    let output = File::create("compressed_cover").unwrap();
    let mut encoder = GzEncoder::new(output, Compression::default());
    let start = Instant::now();
    copy(&mut input, &mut encoder).unwrap();
    let output = encoder.finish().unwrap();
    println!("Length of Source : {}", input.get_ref().metadata().unwrap().len());
    println!("Length of Target : {}", output.metadata().unwrap().len());
    println!("Elasped Time : {:?}", start.elapsed());
}