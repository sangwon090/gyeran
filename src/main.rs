use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};

use gyeran::egg::EggArchive;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file: File = File::open(&args[1]).unwrap();
    let archive: EggArchive = EggArchive::new(file).unwrap();

    
}