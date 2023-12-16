use nom::number::streaming::{le_u8, le_u16, le_u32, le_u64};
use nom::bytes::streaming::take;

use nom::sequence::tuple;
use nom::error::ErrorKind;
use super::header::*;
use super::error::EggError;
use super::signature::EggSignature;

pub struct EggReader;

impl EggReader {
    pub fn read_signature(buf: &[u8]) -> Result<(&[u8], EggSignature), EggError> {
        let parser = le_u32::<_, (_, ErrorKind)>;
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

    pub fn read_file_header(buf: &[u8]) -> Result<(&[u8], FileHeader), EggError> {
        let mut parser = tuple((
            le_u32::<_, (_, ErrorKind)>,
            le_u32::<_, (_, ErrorKind)>,
            le_u64::<_, (_, ErrorKind)>,
        ));
        let (buf, (signature, file_id, file_length)): (&[u8], (u32, u32, u64)) = parser(buf).unwrap();

        return Ok((buf, FileHeader {
            signature,
            file_id,
            file_length,
        }));
    }

    pub fn read_filename_header(buf: &[u8]) -> Result<(&[u8], FilenameHeader), EggError> {
        let mut parser0 = tuple((
            le_u32::<_, (_, ErrorKind)>,
            le_u8::<_, (_, ErrorKind)>,
            le_u16::<_, (_, ErrorKind)>,
        ));
        let (buf, (signature, flag, size)): (&[u8], (u32, u8, u16)) = parser0(buf).unwrap();
        let (buf, locale): (&[u8], Option<u16>) = if (flag & 0b1000) != 0 {
            let (buf, locale) = le_u16::<_, (_, ErrorKind)>(buf).unwrap();
            (buf, Some(locale))
        } else {
            (buf, None)
        };
        let (buf, parent_path_id): (&[u8], Option<u32>) = if (flag & 0b10000) != 0 {
            let (buf, parent_path_id) = le_u32::<_, (_, ErrorKind)>(buf).unwrap();
            (buf, Some(parent_path_id))
        } else {
            (buf, None)
        };
        let (buf, name) = take::<_, _, (_, ErrorKind)>(size)(buf).unwrap();

        return Ok((buf, FilenameHeader {
            signature,
            flag,
            size,
            locale,
            parent_path_id,
            name: Vec::from(name),
        }));
    }

    pub fn read_windows_file_info(buf: &[u8]) -> Result<(&[u8], WindowsFileInfo), EggError> {
        let mut parser = tuple((
            le_u32::<_, (_, ErrorKind)>,
            le_u8::<_, (_, ErrorKind)>,
            le_u16::<_, (_, ErrorKind)>,
            le_u64::<_, (_, ErrorKind)>,
            le_u8::<_, (_, ErrorKind)>,
        ));

        let (buf, (signature, flag, size, last_modified, attribute)): (&[u8], (u32, u8, u16, u64, u8)) = parser(buf).unwrap();
        
        return Ok((buf, WindowsFileInfo {
            signature,
            flag,
            size,
            last_modified,
            attribute,
        }));
    }

    pub fn read_block_header(buf: &[u8]) -> Result<(&[u8], BlockHeader), EggError> {
        let mut parser = tuple((
            le_u32::<_, (_, ErrorKind)>,
            le_u16::<_, (_, ErrorKind)>,
            le_u32::<_, (_, ErrorKind)>,
            le_u32::<_, (_, ErrorKind)>,
            le_u32::<_, (_, ErrorKind)>,
        ));

        let (buf, (signature, compression_method, original_size, compressed_size, crc32)): (&[u8], (u32, u16, u32, u32, u32)) = parser(buf).unwrap();
        
        return Ok((buf, BlockHeader {
            signature,
            compression_method,
            original_size,
            compressed_size,
            crc32,
        }));
    }

    pub fn read_data(buf: &[u8], size: u32) -> Result<(&[u8], &[u8]), EggError> {
        let (buf, data) = take::<_, _, (_, ErrorKind)>(size)(buf).unwrap();
        
        return Ok((buf, data));
    }
}