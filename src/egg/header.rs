pub struct EggHeader {
    magic: u32,
    version: u16,
    header_id: u32,
    reserved: u32,
}

pub enum EggOptionalHeader {
    SplitCompression(EggSplitCompressionHeader),
    SolidCompression(EggSolidCompressionHeader),
    GloablEncryption(EggSplitCompressionHeader),
}

pub struct EggSplitCompressionHeader {
    magic: u32,
    bit_flag: u8,
    size: u16,
    prev_file_id: u32,
    next_file_id: u32,
}

pub struct EggSolidCompressionHeader {
    magic: u32,
    big_flag: u8,
    size: u16,
}

pub struct EggGloablEncryptionHeader {
    magic: u32,
    bit_flag: u8,
    size: u32, // 17 bytes
    method: EggEncryptionMethod, // 1 byte
    verify_data: u128, // 12 bytes
    crc32: u32,
    encryption_header: Vec<u8>, // 128 bit: 10 bytes, 256 bit: 18 bytes
    encryption_footer: Vec<u8>, // 10 bytes
}

pub enum EggEncryptionMethod {
    XOR = 0,
    AES128 = 1,
    AES256 = 2,
    LEA128 = 5,
    LEA256 = 6,
}