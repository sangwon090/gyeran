pub struct EggBlock {
    header: EggBlockHeader,
    extra_header: Vec<EggExtraHeader>,
    compressed_data: Vec<u8>,
}

pub struct EggBlockHeader {
    magic: u32,
    method: u16,
    original_size: u32,
    compressed_size: u32,
    crc32: u32,
}

pub struct EggExtraHeader {
    magic: u32,
    bit_flag: u8,
    size: u32, // 2 bytes or 4 bytes
}