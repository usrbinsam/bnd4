use crate::encodings;
use crate::entry::BND4Entry;
use crate::entry_header::BND4EntryHeader;
use crate::errors::BND4Error;
use crate::header::BND4Header;
use binrw::BinRead;
use std::fs::File;
use std::io::{Seek, SeekFrom};
use std::os::windows::prelude::FileExt;

#[derive(Debug)]
pub struct BND4File {
    pub header: BND4Header,
    pub entries: Vec<BND4Entry>,
}
impl BND4File {
    pub fn from_file(mut f: &File) -> Result<Self, BND4Error> {
        let file_size = f.metadata().map_err(BND4Error::IoError)?.len();

        let header: BND4Header = BND4Header::read(&mut f).map_err(BND4Error::BinReadError)?;
        let mut entries = vec![];

        for _ in 0..header.file_count {
            let entry_header = BND4EntryHeader::read(&mut f).map_err(BND4Error::BinReadError)?;
            let next_seek = f.stream_position().map_err(BND4Error::IoError)?;
            let mut entry_data: Vec<u8> = vec![0u8; entry_header.size as usize];

            if entry_header.data_offset as u64 > file_size
                || entry_header.name_offset as u64 > file_size
            {
                return Err(BND4Error::OffsetOutOfRange);
            }

            f.seek(SeekFrom::Start(entry_header.name_offset as u64))
                .map_err(BND4Error::IoError)?;
            let entry_name = if header.is_unicode {
                encodings::read_utf16(&mut f)
            } else {
                encodings::read_shift_jis(&mut f)
            }?;

            f.seek_read(&mut entry_data, entry_header.data_offset as u64)
                .map_err(BND4Error::IoError)?;

            entries.push(BND4Entry {
                header: entry_header,
                name: entry_name,
                data: entry_data,
            });
            f.seek(SeekFrom::Start(next_seek))
                .map_err(BND4Error::IoError)?;
        }

        Ok(Self { header, entries })
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::entry::CipherMode;
    use hex;

    #[test]
    fn test_dark_souls_remastered() {
        let file = File::open("./tests/DSR.bnd4").unwrap();
        let mut archive = BND4File::from_file(&file).unwrap();

        let key = hex::decode("0123456789ABCDEFFEDCBA9876543210").unwrap();
        let encrypted = archive.entries[0].data.clone();

        assert_eq!(archive.header.bnd_version, 0x3444_4E42); // "BND4"
        assert_eq!(archive.header.file_count, archive.entries.len() as u32);

        for (i, entry) in archive.entries.iter().enumerate() {
            assert_eq!(entry.name, format!("USER_DATA{:0>3}", i));
        }

        // TODO: this test should probably go somewhere else
        archive.entries[0].decrypt(CipherMode::CBC, &key).unwrap();

        assert_ne!(encrypted, archive.entries[0].data);
        println!("{:?}", archive.entries[0].data);
    }

    #[test]
    fn test_elden_ring() {
        let file = File::open("./tests/ER.bnd4").unwrap();
        let archive = BND4File::from_file(&file).unwrap();
        println!("{:?}", archive.header);
    }
}
