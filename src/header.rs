use binrw::BinRead;

#[derive(Debug, BinRead, Eq, PartialEq, Clone)]
#[brw(little)]
pub struct BND4Header {
    pub bnd_version: u32,
    pub unknown: u64,
    pub file_count: u32,
    pub unknown2: u64,
    pub signature: u64,
    pub entry_header_size: u64,
    pub data_offset: u64,
    pub is_unicode: i8,
    pub unknown3: [u8; 15]
}