use std::fmt;

#[derive(Debug)]
pub enum EggError {
    InvalidSignature,
    ParseError,
}

impl fmt::Display for EggError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EggError::InvalidSignature => write!(f, "invalid signature"),
            EggError::ParseError => write!(f, "parse error"),
        }
    }
}