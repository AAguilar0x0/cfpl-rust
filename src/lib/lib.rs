pub mod data_type;
pub mod environment;
pub mod expression;
pub mod interpreter;
pub mod lexeme;
pub mod lexer;
pub mod parser;
pub mod source_code;
pub mod statement;
pub mod token;
pub mod token_type;
use interpreter::interpreter;
use std::fs;
use std::io::ErrorKind;

pub fn execute(source_code_string: String) -> bool {
    let cfpl_source_code = source_code::SourceCode {
        vec: source_code_string.chars().collect(),
        source_code: source_code_string,
    };
    let tokens = match lexer::lexical_analysis(&cfpl_source_code) {
        Ok(result) => result,
        Err(error) => {
            eprint!("[Lexical-Analysis-Error]: {}", error);
            return false;
        }
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
        Err(error) => {
            eprint!("[Syntax-Analysis-Error]: {}", error);
            return false;
        }
    };

    // Debugging purposes

    // for statement in &statements {
    //     println!("{}", crate::statement::display_statement(statement));
    // }

    match interpreter(statements) {
        Ok(_) => (),
        Err(error) => {
            eprint!("[Interpreter-Error]: {}", error);
            return false;
        }
    };

    return true;
}

pub fn file(file_path: &str) -> bool {
    return execute(match fs::read_to_string(file_path) {
        Ok(result) => result,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                eprint!("File not found: {file_path}");
                return false;
            }
            _ => {
                eprint!("Error opening the file: {file_path}");
                return false;
            }
        },
    });
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs;

    fn integration_no_input(path: &str, should_true: bool) {
        let paths = fs::read_dir(path);
        assert!(paths.is_ok());
        let paths = paths.unwrap();
        for path in paths {
            assert!(path.is_ok());
            let path = path.unwrap();
            let path = path.path().to_str().unwrap().to_owned();
            println!("Test file full path: {}", &path);
            let result = file(&path);
            assert!(if should_true { result } else { !result });
            println!();
        }
    }

    #[test]
    fn integration_no_input_no_error() {
        integration_no_input("./test_source_codes/no_input/no_error/", true);
    }

    #[test]
    fn integration_no_input_with_error() {
        integration_no_input("./test_source_codes/no_input/with_error/", false);
    }
}
