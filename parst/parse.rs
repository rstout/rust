extern mod std;
use std::io::{File, Open, ReadWrite};
use std::char::from_u32;

pub fn print_file_bytes(file_name: ~str) {
    let path = Path::new(file_name);

    let mut file = match File::open_mode(&path, Open, ReadWrite) {
        Some(s) => s,
        None => fail!("whoops! I'm sure this raised, anyways..")
    };

    for b in file.bytes() {
        let c = match from_u32(b as u32) {
            Some(c1) => c1,
            None => '?'
        };
        println!("{}", c);
    }
}
