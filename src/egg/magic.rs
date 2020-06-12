pub enum EggMagic {
    EGG_HEADER = 0x41474745,
    FILE_HEADER = 0x0A8590E3,
    BLOCK_HEADER = 0x02B50C13,
    ENCRYPTION_HEADER = 0x08D1470F,
    FILENAME_HEADER = 0x0A8591AC,
    WINDOWS_FILE_INFO_HEADER = 0x2C86950B,
    POSIX_FILE_INFO_HEADER = 0x1EE922E5,
    COMMENT_HEADER = 0x04C63672,
    SPLIT_COMPRESSION_HEADER = 0x24F5A262,
    DUMMY_HEADER = 0x07463307,
    SKIP_HEADER = 0xFFFF0000,
    GLOBAL_ENCRYPT_HEADER = 0x08D144A8,
    END_OF_SIGNATURE = 0x08E28222,
    UNKNOWN,
}

impl From<u32> for EggMagic {
    fn from(value: u32) -> Self {
        match value {
            0x41474745 => EggMagic::EGG_HEADER,
            0x0A8590E3 => EggMagic::FILE_HEADER,
            0x02B50C13 => EggMagic::BLOCK_HEADER,
            0x08D1470F => EggMagic::ENCRYPTION_HEADER,
            0x0A8591AC => EggMagic::FILENAME_HEADER,
            0x2C86950B => EggMagic::WINDOWS_FILE_INFO_HEADER,
            0x1EE922E5 => EggMagic::POSIX_FILE_INFO_HEADER,
            0x04C63672 => EggMagic::COMMENT_HEADER,
            0x24F5A262 => EggMagic::SPLIT_COMPRESSION_HEADER,
            0x07463307 => EggMagic::DUMMY_HEADER,
            0xFFFF0000 => EggMagic::SKIP_HEADER,
            0x08D144A8 => EggMagic::GLOBAL_ENCRYPT_HEADER,
            0x08E28222 => EggMagic::END_OF_SIGNATURE,
            _ => EggMagic::UNKNOWN
        }
    }
}