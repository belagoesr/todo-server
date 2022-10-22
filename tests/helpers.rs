use std::fs::File;
use std::io::Read;

pub fn read_json(file: &str) -> String {
    let path = String::from("dev-resources/") + file;
    let mut file = File::open(&path).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    data
}
