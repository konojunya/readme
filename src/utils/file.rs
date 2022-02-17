use std::fs::File;
use std::io::{Read, Write};

pub fn read_file(path: &str) -> String {
    let mut file = File::open(&path).expect("base markdown not found");
    let mut base_markdown = String::new();
    file.read_to_string(&mut base_markdown)
        .expect("could not read base markdown");

    return base_markdown;
}

pub fn write_file(path: &str, content: &str) {
    let mut file = File::create(path).expect("could not create file");
    file.write_all(content.as_bytes())
        .expect("could not write to file");
}
