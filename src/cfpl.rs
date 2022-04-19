pub mod lexer;
pub mod source_code;
pub mod token;
pub mod token_type;
use std::fs;
use std::io::ErrorKind;

pub fn source_code(source_code_string: String) {
    let cfpl_source_code = source_code::SourceCode {
        vec: source_code_string.chars().collect(),
        source_code: source_code_string,
    };
    let tokens = match lexer::lexical_analysis(&cfpl_source_code) {
        Ok(result) => result,
        Err(error) => return print!("[Lexical-Analysis-Error]: {}", error),
    };
    // Debugging purposes
    print!("Tokens:");
    for token in &tokens {
        print!("\n{}", token);
    }
}

pub fn file(file_path: &str) {
    source_code(match fs::read_to_string(file_path) {
        Ok(result) => result,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => return print!("File not found: {file_path}"),
            _ => return print!("Error opening the file: {file_path}"),
        },
    });
}
