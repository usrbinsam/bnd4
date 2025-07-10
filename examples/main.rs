use bnd4::{encodings, BND4File, CipherMode};
use std::fs::File;
use std::io::{Cursor, Seek};

fn main() {
    let mut sl2_file = File::open("tests/DSR.bnd4").unwrap();
    let mut bnd4data = BND4File::from_file(&mut sl2_file).unwrap();

    for entry in bnd4data.entries.iter_mut() {
        println!("{:?}", entry.name);
        entry
            .decrypt(
                CipherMode::CBC,
                &hex::decode("0123456789ABCDEFFEDCBA9876543210").unwrap(),
            )
            .unwrap();
        let mut cursor = Cursor::new(&entry.data);
        cursor.seek(std::io::SeekFrom::Start(0x00000118)).unwrap();
        println!("{:?}", encodings::read_utf16(&mut cursor).unwrap());
        println!("data offset: {}", entry.header.data_offset);
    }
}
