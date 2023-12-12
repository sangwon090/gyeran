#[derive(Debug)]
pub enum EggSignature {
    Egg = 0x41474745,
    File = 0x0A8590E3,
    Block = 0x02B50C13,
    Encryption = 0x08D1470F,
    Filename = 0x0A8591AC,
    WindowsFileInfo = 0x2C86950B,
    PosixFileInfo = 0x1EE922E5,
    Comment = 0x04C63672,
    Split = 0x24F5A262,
    Solid = 0x24E5A060,
    Dummy = 0x07463307,
    Skip = 0xFFFF0000,
    GlobalEncryption = 0x08D144A8,
    EndOfSignature = 0x08E28222,
    Unknown,
}

impl From<u32> for EggSignature {
    fn from(value: u32) -> Self {
        match value {
            0x41474745 => EggSignature::Egg,
            0x0A8590E3 => EggSignature::File,
            0x02B50C13 => EggSignature::Block,
            0x08D1470F => EggSignature::Encryption,
            0x0A8591AC => EggSignature::Filename,
            0x2C86950B => EggSignature::WindowsFileInfo,
            0x1EE922E5 => EggSignature::PosixFileInfo,
            0x04C63672 => EggSignature::Comment,
            0x24F5A262 => EggSignature::Split,
            0x07463307 => EggSignature::Dummy,
            0xFFFF0000 => EggSignature::Skip,
            0x08D144A8 => EggSignature::GlobalEncryption,
            0x08E28222 => EggSignature::EndOfSignature,
            _ => EggSignature::Unknown,
        }
    }
}