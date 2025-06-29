use aes::cipher::block_padding::UnpadError;
use std::fmt::{self, Display};

#[derive(Debug)]
pub enum BND4Error {
    IoError(std::io::Error),
    OffsetOutOfRange,
    BinReadError(binrw::Error),
    Crypto(CryptoError),
}

#[derive(Debug)]
pub enum CryptoError {
    InvalidLength(aes::cipher::InvalidLength),
    UnpadError(UnpadError),
}

impl Display for CryptoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CryptoError::InvalidLength(e) => write!(f, "Invalid Length: {}", e),
            CryptoError::UnpadError(e) => write!(f, "Unpad Error: {}", e),
        }
    }
}

pub type Result<T> = std::result::Result<T, BND4Error>;

impl Display for BND4Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
        
            BND4Error::IoError(e) => write!(f, "IO Error: {}", e),
            BND4Error::OffsetOutOfRange => write!(f, "Offset out of range"),
            BND4Error::BinReadError(e) => write!(f, "BinRead Error: {}", e),
            BND4Error::Crypto(e) => write!(f, "Crypto Error: {}", e),
        }
    }

}