use std::{env, fs};
use std::fs::File;
use std::io::{prelude::*, BufReader};
use flate2::read::DeflateDecoder;
use nom::AsBytes;

use gyeran::egg::EggArchive;

fn main() {
    let args: Vec<String> = env::args().collect();
    //let file: File = File::open(&args[1]).unwrap();
    let archive: EggArchive = EggArchive::new(File::open("test.egg").unwrap()).unwrap();

    for file in archive.files {
        let filename = String::from_utf8(file.filename_header.unwrap().name).unwrap();

        println!("- {}", filename);
        println!("  original size: {}", file.block_header.original_size);
        println!("  compressed size: {}", file.block_header.compressed_size);
        println!("  compression algorithm: {}", file.block_header.compression_method);
        println!(" ");

        let compression_method =  file.block_header.compression_method & 0xFF;
        let decompressed_data = match compression_method {
            0 => {
                file.data
            },
            1 => {
                let mut d = DeflateDecoder::new(file.data.as_bytes());
                let mut b: Vec<u8> = Vec::new();
                d.read_to_end(&mut b).unwrap();

                b
            },
            _ => {
                unimplemented!("compression algorithm not supported.");
            }
        };

        let mut new_file = fs::OpenOptions::new()
            .create(true)
            .write(true)
            .open(format!("./out/{}", filename)).unwrap();

        new_file.write_all(&decompressed_data).unwrap();
    }
}