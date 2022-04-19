use super::source_code;
use super::token;
use super::token_type;
use std::cell::Cell;

thread_local!(static LINE: Cell<usize> = Cell::new(0));
thread_local!(static COLUMN: Cell<usize> = Cell::new(0));
thread_local!(static FIRST_IN_LINE: Cell<bool> = Cell::new(false));

pub fn lexical_analysis(
    cfpl_source_code: &source_code::SourceCode,
) -> Result<Vec<token::Token>, String> {
    let mut tokens: Vec<token::Token> = Vec::new();
    let source_code = &cfpl_source_code.vec;
    let mut i: usize = 0;
    while i < source_code.len() {
        match source_code[i] {
            '\n' => {
                FIRST_IN_LINE.with(|first_in_line| {
                    let first_in_line_borrowed = first_in_line.get();
                    if !first_in_line_borrowed {
                        first_in_line.set(true);
                        LINE.with(|line| {
                            tokens.push(token::Token::new(
                                token_type::TokenType::Eol,
                                String::from("EOL"),
                                line.get(),
                                0,
                            ))
                        });
                    }
                });
                LINE.with(|line| line.set(line.get() + 1));
                COLUMN.with(|column| column.set(0));
            }
            '(' => {
                single_symbol(&source_code, &mut tokens, i)?;
            }
            ')' => {
                single_symbol(&source_code, &mut tokens, i)?;
            }
            ',' => {
                single_symbol(&source_code, &mut tokens, i)?;
            }
            ':' => {
                single_symbol(&source_code, &mut tokens, i)?;
            }
            '+' => {
                single_symbol(&source_code, &mut tokens, i)?;
            }
            '-' => {
                single_symbol(&source_code, &mut tokens, i)?;
            }
            '*' => {
                if FIRST_IN_LINE.with(|first_in_line| first_in_line.get()) {
                    let index = comment_line(&source_code, i);
                    // COLUMN.with(|column| column.set(column.get() + index - 1));
                    i = index;
                } else {
                    single_symbol(&source_code, &mut tokens, i)?;
                }
            }
            '/' => {
                single_symbol(&source_code, &mut tokens, i)?;
            }
            '%' => {
                single_symbol(&source_code, &mut tokens, i)?;
            }
            other => {
                if other.is_whitespace() {
                    ()
                }
            }
        }
        i += 1;
        COLUMN.with(|column| column.set(column.get() + i));
    }
    LINE.with(|line| {
        tokens.push(token::Token::new(
            token_type::TokenType::Eof,
            String::from("EOF"),
            line.get(),
            0,
        ))
    });
    Ok(tokens)
}

fn single_symbol(
    source_code: &Vec<char>,
    tokens: &mut Vec<token::Token>,
    index: usize,
) -> Result<(), String> {
    let mut error_line = 0;
    let mut error_column = 0;
    LINE.with(|line| error_line = line.get());
    COLUMN.with(|column| error_column = column.get());
    let token_type_here =
        match token_type::get_token_type_from_static(String::from(source_code[index])) {
            Ok(result) => result,
            Err(err) => return Err(err),
        };
    LINE.with(|line| {
        tokens.push(token::Token::new(
            token_type_here,
            String::from(source_code[index]),
            line.get(),
            0,
        ))
    });
    Ok(())
}

fn comment_line(source_code: &Vec<char>, mut index: usize) -> usize {
    while source_code[index] != '\n' {
        index += 1;
    }
    LINE.with(|line| line.set(line.get() + 1));
    COLUMN.with(|column| column.set(0));
    FIRST_IN_LINE.with(|first_in_line| first_in_line.set(true));
    return index;
}
