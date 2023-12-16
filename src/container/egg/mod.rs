pub mod error;
pub mod file;
pub mod header;
pub mod reader;
pub mod signature;
pub mod writer;

use std::io::{Read, Seek, SeekFrom, Write};

use error::EggError;
use file::EggFile;
use header::*;
use signature::EggSignature;

use self::reader::EggReader;

#[derive(Debug)]
pub struct EggArchive {
    pub header: EggHeader,
    pub files: Vec<EggFile>,
}

impl EggArchive {
    pub fn new(mut reader: impl Read + Seek) -> Result<EggArchive, EggError> {
        reader.seek(SeekFrom::Start(0)).unwrap();

        let mut files: Vec<EggFile> = Vec::new();
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer).unwrap();

        // EGG Header
        let (mut buffer, header) = EggReader::read_egg_header(&buffer).unwrap();

        if header.signature != EggSignature::Egg as u32 {
            return Err(EggError::InvalidSignature);
        }

        // Extra Field 1
        loop {
            let (_, signature) = EggReader::read_signature(&buffer[..4]).unwrap();

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
                    let (buf, _) = EggReader::read_signature(&buffer).unwrap();
                    buffer = buf;

                    break;
                },
                _ => {
                    eprintln!("unexpected signature: {:#?}", signature);
                    break;
                },
            }
        }



        loop {
            let mut file: EggFile = EggFile::default();

            let (_, signature) = EggReader::read_signature(&buffer[..4]).unwrap();

            if signature == EggSignature::EndOfSignature {
                break;
            }

            // File Header
            'file_loop: loop {
                let (buf, file_header) = EggReader::read_file_header(&buffer).unwrap();
                buffer = buf;

                if file_header.signature != EggSignature::File as u32 {
                    return Err(EggError::InvalidSignature);
                } else {
                    file.file_header = file_header;
                }

                // Extra Field 2
                loop {
                    let (_, signature) = EggReader::read_signature(&buffer[..4]).unwrap();

                    match signature {
                        EggSignature::Filename => {
                            let (buf, filename) = EggReader::read_filename_header(&buffer).unwrap();
                            file.filename_header = Some(filename);
                            buffer = buf;
                        },
                        EggSignature::Comment => {
                            unimplemented!()
                        },
                        EggSignature::WindowsFileInfo => {
                            let (buf, windows_file_info) = EggReader::read_windows_file_info(&buffer).unwrap();
                            file.windows_file_info = Some(windows_file_info);
                            buffer = buf;
                        },
                        EggSignature::PosixFileInfo => {
                            unimplemented!();
                        },
                        EggSignature::EndOfSignature => {
                            let (buf, _) = EggReader::read_signature(&buffer).unwrap();
                            buffer = buf;

                            break 'file_loop;
                        },
                        _ => {
                            eprintln!("unexpected signature: {:#?}", signature);
                            break 'file_loop;
                        },
                    }
                }
            }



            // Block Header
            let (buf, block_header) = EggReader::read_block_header(&buffer).unwrap();
            buffer = buf;

            if block_header.signature != EggSignature::Block as u32 {
                return Err(EggError::InvalidSignature);
            } else {
                file.block_header = block_header.clone();
            }

            let (buf, signature) = EggReader::read_signature(&buffer).unwrap();
            buffer = buf;

            if signature != EggSignature::EndOfSignature {
                return Err(EggError::InvalidSignature);
            }

            // TODO: need to write the offset and the size of the data, instead of copying it all.
            let (buf, data) = EggReader::read_data(&buffer, block_header.compressed_size).unwrap();
            file.data = Vec::from(data);
            buffer = buf;

            files.push(file);
        }

        Ok(EggArchive {
            header,
            files,
        })
    }
}