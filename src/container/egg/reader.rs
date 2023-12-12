use nom::number::streaming::{le_u16, le_u32};

use nom::sequence::tuple;
use nom::error::{ErrorKind};
use super::header::*;
use super::error::EggError;
use super::signature::EggSignature;

pub struct EggReader;

impl EggReader {
    pub fn read_signature(buf: &[u8]) -> Result<(&[u8], EggSignature), EggError> {
        let mut parser = le_u32::<_, (_, ErrorKind)>;
        let (buf, signature) = parser(buf).unwrap();

        return Ok((buf, signature.into()))
    }

    pub fn read_egg_header(buf: &[u8]) -> Result<(&[u8], EggHeader), EggError> {
        let mut parser = tuple((
            le_u32::<_, (_, ErrorKind)>,
            le_u16::<_, (_, ErrorKind)>,
            le_u32::<_, (_, ErrorKind)>,
            le_u32::<_, (_, ErrorKind)>
        ));
        let (buf, (signature, version, header_id, reserved)): (&[u8], (u32, u16, u32, u32)) = parser(buf).unwrap();

        return Ok((buf, EggHeader {
            signature,
            version,
            header_id,
            reserved,
        }));
    }
}