use bnd4::BND4File;
use std::fs::File;

fn main() {
    let mut sl2_file = File::open("tests/NR.bnd4").unwrap();
    let bnd4data = BND4File::from_file(&mut sl2_file).unwrap();

    for entry in bnd4data.entries.iter() {
        println!("{:?}", entry.name);
    }
}
