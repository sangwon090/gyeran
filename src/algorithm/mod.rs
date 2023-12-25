#[derive(Debug, Eq, PartialEq)]
pub enum Algorithm {
    None = 0,
    Deflate = 1,
    AZO = 2,
    LZMA = 3,
    Unknown,
}

impl From<u8> for Algorithm {
    fn from(value: u8) -> Self {
        match value {
            0x00 => Algorithm::None,
            0x01 => Algorithm::Deflate,
            0x02 => Algorithm::AZO,
            0x03 => Algorithm::LZMA,
            _ => Algorithm::Unknown,
        }
    }
}