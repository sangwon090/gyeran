#[derive(Debug)]
pub enum EggSignature {
    EGG = 0x41474745,
    FILE = 0x0A8590E3,
    BLOCK = 0x02B50C13,
    ENCRYPTION = 0x08D1470F,
    FILENAME = 0x0A8591AC,
    WINDOWS_FILE_INFO = 0x2C86950B,
    POSIX_FILE_INFO = 0x1EE922E5,
    COMMENT = 0x04C63672,
    SPLIT = 0x24F5A262,
    SOLID = 0x24E5A060,
    DUMMY = 0x07463307,
    SKIP = 0xFFFF0000,
    GLOBAL_ENCRYPTION = 0x08D144A8,
    END_OF_SIGNATURE = 0x08E28222,
    UNKNOWN,
}

impl From<u32> for EggSignature {
    fn from(value: u32) -> Self {
        match value {
            0x41474745 => EggSignature::EGG,
            0x0A8590E3 => EggSignature::FILE,
            0x02B50C13 => EggSignature::BLOCK,
            0x08D1470F => EggSignature::ENCRYPTION,
            0x0A8591AC => EggSignature::FILENAME,
            0x2C86950B => EggSignature::WINDOWS_FILE_INFO,
            0x1EE922E5 => EggSignature::POSIX_FILE_INFO,
            0x04C63672 => EggSignature::COMMENT,
            0x24F5A262 => EggSignature::SPLIT,
            0x07463307 => EggSignature::DUMMY,
            0xFFFF0000 => EggSignature::SKIP,
            0x08D144A8 => EggSignature::GLOBAL_ENCRYPTION,
            0x08E28222 => EggSignature::END_OF_SIGNATURE,
            _ => EggSignature::UNKNOWN
        }
    }
}