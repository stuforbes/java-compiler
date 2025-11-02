use std::fs;

#[allow(dead_code)]
pub fn read_file(file_path: &str) -> String {
    match fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(e) => panic!("Could not read file {e}"),
    }
}
