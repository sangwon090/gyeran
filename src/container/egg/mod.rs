pub mod error;
pub mod header;
pub mod reader;
pub mod signature;
pub mod writer;

use std::io::{Read, Seek, SeekFrom};

use error::EggError;
use header::*;
use signature::EggSignature;

use self::reader::EggReader;

pub struct EggArchive {
    header: EggHeader,
}

impl EggArchive {
    pub fn new(mut reader: impl Read + Seek) -> Result<EggArchive, EggError> {
        reader.seek(SeekFrom::Start(0)).unwrap();

        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer).unwrap();

        // EGG Header
        let (mut buffer, header) = EggReader::read_egg_header(&buffer).unwrap();

        if header.signature != EggSignature::Egg as u32 {
            return Err(EggError::InvalidSignature);
        }
        println!("EGG Header: {:?}", header);

        // Extra Field 1
        loop {
            let (buf, signature) = EggReader::read_signature(&buffer).unwrap();
            buffer = buf;

            match signature {
                EggSignature::Split => {
                    unimplemented!();
                },
                EggSignature::Solid => {
                    unimplemented!();
                },
                EggSignature::GlobalEncryption => {
                    unimplemented!();
                },
                EggSignature::EndOfSignature => break,
                _ => {
                    eprintln!("unexpected signature: {:#?}", signature);
                    break;
                },
            }
        }
        println!("Extra field done.");

        'outer: loop {
            // File Header
            'file_loop: loop {
                let (buf, file_header) = EggReader::read_file_header(&buffer).unwrap();
                buffer = buf;

                if file_header.signature != EggSignature::File as u32 {
                    println!("not a file signature. {:X} found", file_header.signature);
                    return Err(EggError::InvalidSignature);
                }

                // Extra Field 2
                loop {
                    let (buf, signature) = EggReader::read_signature(&buffer).unwrap();
                    buffer = buf;

                    match signature {
                        EggSignature::Filename => {
                            // TODO: need to decide whether signature is consumed inside reader.
                            let (buf, filename) = EggReader::read_filename_header(&buffer).unwrap();
                            buffer = buf;

                            println!("filename: {}", String::from_utf8(filename.name).unwrap());
                        },
                        EggSignature::Comment => {
                            unimplemented!();
                        },
                        EggSignature::WindowsFileInfo => {
                            unimplemented!();
                        },
                        EggSignature::PosixFileInfo => {
                            unimplemented!();
                        },
                        EggSignature::EndOfSignature => {
                            unimplemented!();
                        },
                        EggSignature::EndOfSignature => break 'file_loop,
                        _ => {
                            eprintln!("unexpected signature: {:#?}", signature);
                            break 'file_loop;
                        },
                    }
                }
            }

            // Block Header
            'block_loop: loop {
                break;
            }
        }

        Err(EggError::ParseError)
    }
}