mod lexeme;
mod lexer;
mod parser;
mod parsing_statements;
mod source_code;
mod token;
mod token_type;
use std::fs;
use std::io::ErrorKind;

fn execute(source_code_string: String) {
    let cfpl_source_code = source_code::SourceCode {
        vec: source_code_string.chars().collect(),
        source_code: source_code_string,
    };
    let tokens = match lexer::lexical_analysis(&cfpl_source_code) {
        Ok(result) => result,
        Err(error) => return print!("[Lexical-Analysis-Error]: {}", error),
    };

    // Debugging purposes

    // print!("Tokens:");
    // for token in &tokens {
    //     print!("\n{token} {} {}", token.line + 1, token.column + 1);
    // }
    // for token in &tokens {
    //     print!(
    //         "{}{}",
    //         if matches!(token.token_type, token_type::TokenType::Eol) {
    //             "\n"
    //         } else {
    //             token.lexeme.as_str()
    //         },
    //         if matches!(token.token_type, token_type::TokenType::Eol) {
    //             ""
    //         } else {
    //             " "
    //         }
    //     );
    // }

    let statements = match parser::Parser::syntax_analysis(&cfpl_source_code, &tokens) {
        Ok(result) => result,
        Err(error) => return print!("[Syntax-Analysis-Error]: {}", error),
    };
}

pub fn file(file_path: &str) {
    execute(match fs::read_to_string(file_path) {
        Ok(result) => result,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => return print!("File not found: {file_path}"),
            _ => return print!("Error opening the file: {file_path}"),
        },
    });
}
