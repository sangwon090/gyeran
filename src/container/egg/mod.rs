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

        let (buffer, header) = EggReader::read_egg_header(&buffer).unwrap();

        if header.signature != EggSignature::Egg as u32 {
            return Err(EggError::InvalidSignature);
        }

        loop {
            let (buffer, signature) = EggReader::read_signature(&buffer).unwrap();

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
                EggSignature::EndOfSignature => {
                    println!("EndOfSignature found");
                    break; },
                _ => {
                    eprintln!("unexpected signature: {:#?}", signature);
                    break;
                },
            }
        }

        println!("{:#?}", header);

        Err(EggError::ParseError)
    }
}