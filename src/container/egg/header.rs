pub struct EggHeader {
    pub signature: u32,
    pub version: u16,
    pub header_id: u32,
    pub reserved: u32,
}

pub struct FileHeader {
    signature: u32,
    file_id: u32,
    file_length: u64,
}

pub struct BlockHeader {
    signature: u32,
    compression_method: u16,
    original_size: u32,
    compressed_size: u32,
    crc32: u32,
}

pub struct ExtraField16 {
    signature: u32,
    flag: u8,
    size: u16,
}

pub struct ExtraField32 {
    signature: u32,
    flag: u8,
    size: u32,
}

pub struct EncryptionHeader128 {
    signauture: u32,
    flag: u8,
    size: [u8; 17],
    method: u8,
    verification: [u8; 12],
    crc32: u32,
    aes_header: [u8; 10],
    aes_footer: [u8; 10],
}

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

pub struct WindowsFileInfo {
    signature: u32,
    flag: u8,
    size: u16,
    last_modified: u64,
    attribute: u8,
}

pub struct PosixFileInfo {
    signature: u32,
    flag: u8,
    size: u16,
    mode: u32,
    uid: u32,
    gid: u32,
    last_modified: u64,
}

pub struct DummyHeader {
    signature: u32,
    flag: u8,
    size: u16,
    data: Vec<u8>,
}

pub struct FilenameHeader {
    signature: u32,
    flag: u8,
    size: u16,
    locale: Option<u16>,
    parent_path_id: Option<u32>,
    name: Vec<u8>,
}

pub struct CommentHeader {
    signature: u32,
    flag: u8,
    size: u16,
    comment: Vec<u8>,
}

pub struct SplitHeader {
    signature: u32,
    flag: u8,
    size: u16,
    prev_file_id: u32,
    next_file_id: u32,
}

pub struct SolidHeader {
    signature: u32,
    flag: u8,
    size: u16,
}