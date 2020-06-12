use super::EggCommentHeader;
use super::EggEncryptionHeader;

pub struct EggFile {
    metadata: Vec<EggFileMetadata>,
    block: Vec<EggBlock>,
}

pub struct EggFileMetadata {
    header: EggFileMetadataHeader,
    options: Vec<EggFileOptionHeader>,
}

pub struct EggFileMetadataHeader {
    magic: u32,
    file_id: u32,
    file_length: u64,
}

pub enum EggFileOptionHeader {
    Filename(EggFilenameHeader),
    Comment(EggCommentHeader),
    WindowsFileInfo(EggWindowsFileInfoHeader),
    PosixFileInfo(EggPosixFileInfoHeader),
    Encryption(EggEncryptionHeader),
}

pub struct EggFilenameHeader {
    magic: u32,
    bit_flag: u8,
    size: u16,
    locale: Option<u16>,
    parent_path_id: Option<u32>,
    name: String,
}

pub struct EggWindowsFileInfoHeader {
    magic: u32,
    bit_flag: u8,
    size: u16,
    datetime: u64,
    attribute: u8,
}

pub struct EggPosixFileInfoHeader {
    magic: u32,
    bit_flag: u8,
    size: u16,
    mode: u32,
    uid: u32,
    gid: u32,
    last_modified: u64,
}

pub struct EggBlock {
    header: EggBlockHeader,
    extra_data: Vec<EggExtraData>,
    compressed_data: Vec<u8>,
}

pub struct EggBlockHeader {
    method: u16,
    original_size: u32,
    compressed_size: u32,
    crc32: u32,
}

pub struct EggExtraData {
    bit_flag: u8,
    size: u32, // 2 bytes or 4 bytes
}