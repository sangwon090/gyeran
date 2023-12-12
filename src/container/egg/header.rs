#[derive(Debug)]
pub struct EggHeader {
    pub signature: u32,
    pub version: u16,
    pub header_id: u32,
    pub reserved: u32,
}

#[derive(Debug)]
pub struct FileHeader {
    pub signature: u32,
    pub file_id: u32,
    pub file_length: u64,
}

#[derive(Debug)]
pub struct BlockHeader {
    signature: u32,
    compression_method: u16,
    original_size: u32,
    compressed_size: u32,
    crc32: u32,
}

#[derive(Debug)]
pub struct ExtraField16 {
    signature: u32,
    flag: u8,
    size: u16,
}

#[derive(Debug)]
pub struct ExtraField32 {
    signature: u32,
    flag: u8,
    size: u32,
}

#[derive(Debug)]
pub struct EncryptionHeader128 {
    signature: u32,
    flag: u8,
    size: [u8; 17],
    method: u8,
    verification: [u8; 12],
    crc32: u32,
    aes_header: [u8; 10],
    aes_footer: [u8; 10],
}

#[derive(Debug)]
pub struct EncryptionHeader256 {
    signauture: u32,
    flag: u8,
    size: [u8; 17],
    method: u8,
    verification: [u8; 12],
    crc32: u32,
    aes_header: [u8; 18],
    aes_footer: [u8; 10],
}

#[derive(Debug)]
pub struct WindowsFileInfo {
    signature: u32,
    flag: u8,
    size: u16,
    last_modified: u64,
    attribute: u8,
}

#[derive(Debug)]
pub struct PosixFileInfo {
    signature: u32,
    flag: u8,
    size: u16,
    mode: u32,
    uid: u32,
    gid: u32,
    last_modified: u64,
}

#[derive(Debug)]
pub struct DummyHeader {
    signature: u32,
    flag: u8,
    size: u16,
    data: Vec<u8>,
}

#[derive(Debug)]
pub struct FilenameHeader {
    pub signature: u32,
    pub flag: u8,
    pub size: u16,
    pub locale: Option<u16>,
    pub parent_path_id: Option<u32>,
    pub name: Vec<u8>,
}

#[derive(Debug)]
pub struct CommentHeader {
    signature: u32,
    flag: u8,
    size: u16,
    comment: Vec<u8>,
}

#[derive(Debug)]
pub struct SplitHeader {
    signature: u32,
    flag: u8,
    size: u16,
    prev_file_id: u32,
    next_file_id: u32,
}

#[derive(Debug)]
pub struct SolidHeader {
    signature: u32,
    flag: u8,
    size: u16,
}