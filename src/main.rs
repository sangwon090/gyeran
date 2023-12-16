use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};

use gyeran::egg::EggArchive;

fn main() {
    let args: Vec<String> = env::args().collect();
    //let file: File = File::open(&args[1]).unwrap();
    let archive: EggArchive = EggArchive::new(File::open("eggtest.egg").unwrap()).unwrap();

    for file in archive.files {
        let filename = String::from_utf8(file.filename_header.unwrap().name).unwrap();

        println!("- {}", filename);
        println!("  original size: {}", file.block_header.original_size);
        println!("  compressed size: {}", file.block_header.compressed_size);
        println!("  compression algorithm: {} (0: None, 1: Deflate, 2: Bzip2, 3: AZO, 4: LZMA)", file.block_header.compression_method);
        println!(" ");
    }
}