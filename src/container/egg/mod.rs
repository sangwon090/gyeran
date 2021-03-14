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
        
        let header = EggReader::read_egg_header(&mut reader).unwrap();

        if header.signature != EggSignature::EGG as u32 {
            return Err(EggError::InvalidSignature);
        }

        loop {
            let signature = EggReader::read_signature(&mut reader);

            match signature {
                EggSignature::SPLIT => {
                    unimplemented!();
                },
                EggSignature::SOLID => {
                    unimplemented!();
                },
                EggSignature::GLOBAL_ENCRYPTION => {
                    unimplemented!();
                },
                EggSignature::END_OF_SIGNATURE => break,
                _ => eprintln!("unexpected signature: {:#?}", signature),
            }
        }

        Err(EggError::ParseError)
    }
}