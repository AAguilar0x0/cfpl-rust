use std::fs::File;
use std::io::ErrorKind;
use std::io::Read;
pub mod lexer;
pub mod source_code;
pub mod token;
pub mod token_type;

pub fn source_code(source_code_string: String) {
    let cfpl_source_code = source_code::SourceCode {
        source_code: source_code_string,
    };
    if let Err(error) = lexer::lexical_analysis(&cfpl_source_code) {
        print!("[Lexical-Analysis-Error]: {}", error)
    }
}

pub fn file(file_path: &str) {
    let _file = File::open(file_path);
    let mut _file = match _file {
        Ok(file_result) => file_result,
        Err(error) => {
            return match error.kind() {
                ErrorKind::NotFound => print!("File not found: {file_path}"),
                _ => print!("Error opening the file: {file_path}"),
            }
        }
    };
    let mut source = String::new();
    match _file.read_to_string(&mut source) {
        Ok(_) => (),
        Err(_) => return print!("Error reading the file: {file_path}"),
    };
    source_code(source);
}
