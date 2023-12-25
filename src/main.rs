use std::{env, fs};
use std::fs::File;
use std::io::prelude::*;
use flate2::read::{DeflateDecoder, ZlibDecoder};
use nom::AsBytes;

use gyeran::algorithm::Algorithm;
use gyeran::egg::EggArchive;


fn main() {
    let args: Vec<String> = env::args().collect();
    let file: File = File::open(&args[1]).unwrap();
    let archive: EggArchive = EggArchive::new(file).unwrap();

    for file in archive.files {
        let filename = String::from_utf8(file.filename_header.unwrap().name).unwrap();

        println!("- {}", filename);
        println!("  original size: {}", file.block_header.original_size);
        println!("  compressed size: {}", file.block_header.compressed_size);
        println!(" ");

        let compression_method = Algorithm::from((file.block_header.compression_method & 0xFF) as u8);
        let decompressed_data = match compression_method {
            Algorithm::None => {
                file.data
            },
            Algorithm::Deflate => {
                let mut d = DeflateDecoder::new(file.data.as_bytes());
                let mut b: Vec<u8> = Vec::new();
                d.read_to_end(&mut b).unwrap();

                b
            },
            Algorithm::AZO => {
                unimplemented!("AZO algorithm not supported yet.");
            }
            Algorithm::LZMA => {
                let mut d = ZlibDecoder::new(file.data.as_bytes());
                let mut b: Vec<u8> = Vec::new();
                d.read_to_end(&mut b).unwrap();

                b
            }
            Algorithm::Unknown => {
                panic!("unknown algorithm.");
            }
        };

        fs::create_dir_all("./out/").unwrap();

        let mut new_file = fs::OpenOptions::new()
            .create(true)
            .write(true)
            .open(format!("./out/{}", filename)).unwrap();

        new_file.write_all(&decompressed_data).unwrap();
    }
}