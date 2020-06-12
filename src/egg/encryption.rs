pub struct EggEncryptionHeader {
    bit_flag: u8,
    size: u32,                   // 17 bytes
    method: EggEncryptionMethod, // 1 byte
    verify_data: u128,           // 12 bytes
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