use nom::{do_parse, named};
use nom::number::streaming::{le_u16, le_u32};

use std::io::{Read, Seek};
use super::header::*;
use super::error::EggError;
use super::signature::EggSignature;

pub struct EggReader;

impl EggReader {
    pub fn read_signature(mut reader: impl Read + Seek) -> EggSignature {
        let mut data: [u8; 4] = [0; 4];
        reader.read(&mut data).unwrap();

        u32::from_le_bytes(data).into()
    }

    pub fn read_egg_header(mut reader: impl Read + Seek) -> Result<EggHeader, EggError> {
        named!(parse<EggHeader>,
            do_parse!(
                signature: le_u32 >>
                version: le_u16 >>
                header_id: le_u32 >>
                reserved: le_u32 >>
                (EggHeader {
                    signature: signature,
                    version: version,
                    header_id: header_id,
                    reserved: reserved,
                })
            )
        );

        let mut data: [u8; 14] = [0; 14];
        reader.read(&mut data).unwrap();

        Ok(parse(&data).unwrap().1)
    }
}