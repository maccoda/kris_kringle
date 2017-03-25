use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

pub fn write_to_file<P: AsRef<Path>>(file_name: P, content_to_write: String) {
    let mut file = File::create(file_name).unwrap();
    let content: &[u8] = &(content_to_write.into_bytes())[..];
    file.write_all(content).unwrap();
}

pub fn read_from_file<P: AsRef<Path>>(file_name: P) -> String {
    let mut content = String::new();
    let mut file = File::open(file_name).unwrap();
    file.read_to_string(&mut content).unwrap();
    content
}