#[derive(Debug, Default, Clone)]
pub struct EggHeader {
    pub signature: u32,
    pub version: u16,
    pub header_id: u32,
    pub reserved: u32,
}

#[derive(Debug, Default, Clone)]
pub struct FileHeader {
    pub signature: u32,
    pub file_id: u32,
    pub file_length: u64,
}

#[derive(Debug, Default, Clone)]
pub struct BlockHeader {
    pub signature: u32,
    pub compression_method: u16,
    pub original_size: u32,
    pub compressed_size: u32,
    pub crc32: u32,
}

#[derive(Debug, Default, Clone)]
pub struct ExtraField16 {
    signature: u32,
    flag: u8,
    size: u16,
}

#[derive(Debug, Default, Clone)]
pub struct ExtraField32 {
    signature: u32,
    flag: u8,
    size: u32,
}

#[derive(Debug, Default, Clone)]
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

#[derive(Debug, Default, Clone)]
pub struct EncryptionHeader256 {
    signature: u32,
    flag: u8,
    size: [u8; 17],
    method: u8,
    verification: [u8; 12],
    crc32: u32,
    aes_header: [u8; 18],
    aes_footer: [u8; 10],
}

#[derive(Debug, Default, Clone)]
pub struct WindowsFileInfo {
    pub signature: u32,
    pub flag: u8,
    pub size: u16,
    pub last_modified: u64,
    pub attribute: u8,
}

#[derive(Debug, Default, Clone)]
pub struct PosixFileInfo {
    signature: u32,
    flag: u8,
    size: u16,
    mode: u32,
    uid: u32,
    gid: u32,
    last_modified: u64,
}

#[derive(Debug, Default, Clone)]
pub struct DummyHeader {
    signature: u32,
    flag: u8,
    size: u16,
    data: Vec<u8>,
}

#[derive(Debug, Default, Clone)]
pub struct FilenameHeader {
    pub signature: u32,
    pub flag: u8,
    pub size: u16,
    pub locale: Option<u16>,
    pub parent_path_id: Option<u32>,
    pub name: Vec<u8>,
}

#[derive(Debug, Default, Clone)]
pub struct CommentHeader {
    signature: u32,
    flag: u8,
    size: u16,
    comment: Vec<u8>,
}

#[derive(Debug, Default, Clone)]
pub struct SplitHeader {
    signature: u32,
    flag: u8,
    size: u16,
    prev_file_id: u32,
    next_file_id: u32,
}

#[derive(Debug, Default, Clone)]
pub struct SolidHeader {
    signature: u32,
    flag: u8,
    size: u16,
}