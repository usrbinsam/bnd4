use binrw::BinRead;

#[derive(Debug, BinRead, Eq, PartialEq, Clone)]
#[brw(little)]
pub struct BND4EntryHeader {
    pub padding: u64,
    /// The length of the entry.
    pub size: u64,
    /// The byte offset where the data begins.
    pub data_offset: u32,
    /// The byte offset where the null-terminated name begins.
    pub name_offset: u32,
    pub unused: u64,
}
