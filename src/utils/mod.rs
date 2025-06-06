use std::path;

pub fn read_file(path: &str) -> String {
    let path = path::Path::new(path);
    std::fs::read_to_string(path).expect("Failed to read file")
}
