use super::EggEncryptionHeader;

pub struct EggFile {
    file_header: EggFileHeader,
    file_data: Vec<EggFileData>,
}

pub struct EggFileHeader {
    magic: u32,
    file_id: u32,
    file_length: u64,
}

pub enum EggFileData {
    Filename(EggFilenameHeader),
    Comment(EggCommentHeader),
    WindowsFileInfo(EggWindowsFileInfoHeader),
    PosixFileInfo(EggPosixFileInfoHeader),
    Encryption(EggEncryptionHeader)
}

pub struct EggFilenameHeader {
    magic: u32,
    bit_flag: u8,
    size: u16,
    locale: Option<u16>,
    parent_path_id: Option<u32>,
    name: String,
}

pub struct EggCommentHeader {
    magic: u32,
    bit_flag: u8,
    size: u16,
    comment: String,
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