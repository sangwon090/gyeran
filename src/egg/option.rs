use super::encryption::EggEncryptionHeader;

pub enum EggOptionHeader {
    SplitCompression(EggSplitCompressionHeader),
    SolidCompression(EggSolidCompressionHeader),
    GloablEncryption(EggEncryptionHeader),
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