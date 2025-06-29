use crate::entry_header::BND4EntryHeader;
use crate::errors::{BND4Error, CryptoError};
#[cfg(feature = "ctr")]
use aes::cipher::StreamCipher;
#[cfg(feature = "cbc")]
use cbc::cipher::{block_padding::Pkcs7, BlockDecryptMut, KeyIvInit};

/// A parsed BND4 entry.
///
/// In most cases, this would represent an in-game save slot for a character.
#[derive(Debug)]
pub struct BND4Entry {
    /// The header for this entry.
    pub header: BND4EntryHeader,
    /// The name assigned to the entry, typically (USER_DATA###)
    pub name: String,
    /// The entry-specific data, potentially encrypted.
    /// Use [`Self::decrypt`] for encrypted entry data.
    pub data: Vec<u8>,
}

#[cfg(feature = "cbc")]
type CbcDec = cbc::Decryptor<aes::Aes128>;
#[cfg(feature = "ctr")]
type CtrDec = ctr::Ctr128LE<aes::Aes128>;

const AES_BLOCK_SIZE: usize = 16;

#[derive(Debug, Clone, Copy)]
pub enum CipherMode {
    #[cfg(feature = "cbc")]
    CBC,
    #[cfg(feature = "ctr")]
    CTR,
}

/// Represents
impl BND4Entry {
    /// Decrypt the BND4 [data](BND4Entry::data) field.
    #[cfg(feature = "cbc")]
    fn decrypt_cbc(&mut self, key: &[u8]) -> Result<(), BND4Error> {
        let aes = CbcDec::new_from_slices(&key, &self.data[0..AES_BLOCK_SIZE])
            .map_err(CryptoError::InvalidLength)
            .map_err(BND4Error::Crypto)?;

        aes.decrypt_padded_mut::<Pkcs7>(&mut self.data[AES_BLOCK_SIZE..])
            .map_err(CryptoError::UnpadError)
            .map_err(BND4Error::Crypto)?;

        Ok(())
    }

    #[cfg(feature = "ctr")]
    fn decrypt_ctr(&mut self, key: &[u8]) -> Result<(), BND4Error> {
        let mut aes = CtrDec::new_from_slices(&key, &self.data[0..AES_BLOCK_SIZE])
            .map_err(CryptoError::InvalidLength)
            .map_err(BND4Error::Crypto)?;

        // Hmmm, this could panic ...
        aes.apply_keystream(&mut self.data[AES_BLOCK_SIZE..]);

        Ok(())
    }

    pub fn decrypt(&mut self, mode: CipherMode, key: &[u8]) -> Result<(), BND4Error> {
        match mode {
            #[cfg(feature = "cbc")]
            CipherMode::CBC => self.decrypt_cbc(key),
            #[cfg(feature = "ctr")]
            CipherMode::CTR => self.decrypt_ctr(key),
        }
    }
}
