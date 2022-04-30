use super::lexeme;
use super::source_code;
use super::token;
use super::token_type;
use std::cell::Cell;

thread_local!(static LINE: Cell<usize> = Cell::new(0));
thread_local!(static COLUMN: Cell<usize> = Cell::new(0));
thread_local!(static FIRST_IN_LINE: Cell<bool> = Cell::new(true));

pub fn lexical_analysis(
    cfpl_source_code: &source_code::SourceCode,
) -> Result<Vec<token::Token>, String> {
    let mut tokens: Vec<token::Token> = Vec::new();
    let source_code = &cfpl_source_code.vec;
    let mut i: usize = 0;
    let _debug_length = source_code.len();
    let mut _debug_current_character = source_code[i].clone();
    let mut _debug_current_column = COLUMN.with(|column| column.get());
    let mut _debug_current_line = LINE.with(|line| line.get());
    let mut _debug_first_in_line = FIRST_IN_LINE.with(|first_in_line| first_in_line.get());
    while i < source_code.len() {
        _debug_current_character = source_code[i].clone();
        _debug_current_column = COLUMN.with(|column| column.get());
        _debug_current_line = LINE.with(|line| line.get());
        _debug_first_in_line = FIRST_IN_LINE.with(|first_in_line| first_in_line.get());
        let (index_result, column_result, is_override_fil): (usize, Result<usize, usize>, bool) =
            match source_code[i] {
                '\n' => {
                    FIRST_IN_LINE.with(|first_in_line| {
                        if !first_in_line.get() {
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
                    (0, Err(0), false)
                }
                '(' => {
                    single_symbol(&cfpl_source_code, &mut tokens, i)?;
                    (0, Ok(0), false)
                }
                ')' => {
                    single_symbol(&cfpl_source_code, &mut tokens, i)?;
                    (0, Ok(0), false)
                }
                ',' => {
                    single_symbol(&cfpl_source_code, &mut tokens, i)?;
                    (0, Ok(0), false)
                }
                ':' => {
                    single_symbol(&cfpl_source_code, &mut tokens, i)?;
                    (0, Ok(0), false)
                }
                '+' => {
                    single_symbol(&cfpl_source_code, &mut tokens, i)?;
                    (0, Ok(0), false)
                }
                '-' => {
                    single_symbol(&cfpl_source_code, &mut tokens, i)?;
                    (0, Ok(0), false)
                }
                '/' => {
                    single_symbol(&cfpl_source_code, &mut tokens, i)?;
                    (0, Ok(0), false)
                }
                '%' => {
                    single_symbol(&cfpl_source_code, &mut tokens, i)?;
                    (0, Ok(0), false)
                }
                '*' => {
                    if FIRST_IN_LINE.with(|first_in_line| first_in_line.get()) {
                        let index = comment_line(&source_code, i);
                        LINE.with(|line| line.set(line.get() + 1));
                        FIRST_IN_LINE.with(|first_in_line| first_in_line.set(true));
                        (index - i, Err(0), true)
                    } else {
                        single_symbol(&cfpl_source_code, &mut tokens, i)?;
                        (0, Ok(0), false)
                    }
                }
                '=' => {
                    let index = single_double_symbol(
                        &source_code,
                        &mut tokens,
                        i,
                        lexeme::possibly_equal_assignment,
                    );
                    (index - i, Ok(index - i), false)
                }
                '<' => {
                    let index = single_double_symbol(
                        &source_code,
                        &mut tokens,
                        i,
                        lexeme::possibly_lesser_lesser_equal_notequal,
                    );
                    (index - i, Ok(index - i), false)
                }
                '>' => {
                    let index = single_double_symbol(
                        &source_code,
                        &mut tokens,
                        i,
                        lexeme::possibly_greater_greater_equal,
                    );
                    (index - i, Ok(index - i), false)
                }
                other => {
                    if lexeme::is_single_quote(other) {
                        let index = character_literal(&cfpl_source_code, &mut tokens, i)?;
                        (index - i, Ok(index - i), false)
                    } else if lexeme::is_double_quote(other) {
                        (0, Ok(0), false)
                    } else {
                        let token_line = LINE.with(|line| line.get());
                        let token_column = COLUMN.with(|column| column.get());
                        Err(cfpl_source_code.error_string_manual(
                            token_line,
                            token_column,
                            String::from(other),
                            "Invalid character token.".to_string(),
                        ))?;
                        (0, Ok(0), false)
                    }
                }
            };
        if !is_override_fil && !source_code[i].is_whitespace() {
            FIRST_IN_LINE.with(|first_in_line| first_in_line.set(false));
        }
        i += index_result + 1;
        match column_result {
            Ok(increment_value) => {
                COLUMN.with(|column| column.set(column.get() + increment_value + 1))
            }
            Err(no_increment_value) => COLUMN.with(|column| column.set(no_increment_value)),
        }
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
    cfpl_source_code: &source_code::SourceCode,
    tokens: &mut Vec<token::Token>,
    index: usize,
) -> Result<(), String> {
    let token_line = LINE.with(|line| line.get());
    let token_column = COLUMN.with(|column| column.get());
    let token_type_here =
        match lexeme::static_lexeme_to_token_type(String::from(cfpl_source_code.vec[index])) {
            Ok(result) => result,
            Err(err) => {
                return Err(cfpl_source_code.error_string_manual(
                    token_line,
                    token_column,
                    cfpl_source_code.vec[index].to_string(),
                    err,
                ))
            }
        };
    tokens.push(token::Token::new(
        token_type_here,
        String::from(cfpl_source_code.vec[index]),
        token_line,
        token_column,
    ));
    Ok(())
}

fn comment_line(source_code: &Vec<char>, mut index: usize) -> usize {
    while index < source_code.len() && source_code[index] != '\n' {
        index += 1;
    }
    // LINE.with(|line| line.set(line.get() + 1));
    // COLUMN.with(|column| column.set(0));
    // FIRST_IN_LINE.with(|first_in_line| first_in_line.set(true));
    return index;
}

fn single_double_symbol(
    source_code: &Vec<char>,
    tokens: &mut Vec<token::Token>,
    index: usize,
    get_some_token_value: fn(&Vec<char>, usize) -> (token_type::TokenType, String, usize),
) -> usize {
    let (token_type, lexeme, index) = get_some_token_value(source_code, index);
    let token_line = LINE.with(|line| line.get());
    let token_column = COLUMN.with(|column| column.get());
    tokens.push(token::Token::new(
        token_type,
        lexeme,
        token_line,
        token_column,
    ));
    index
}

fn character_literal(
    cfpl_source_code: &source_code::SourceCode,
    tokens: &mut Vec<token::Token>,
    mut index: usize,
) -> Result<usize, String> {
    let unmod_index = index;
    index += 1;
    let source_code = &cfpl_source_code.vec;
    let token_line = LINE.with(|line| line.get());
    let token_column = COLUMN.with(|column| column.get());
    let result = if lexeme::is_single_quote(source_code[index]) {
        Ok(token::Token::new(
            token_type::TokenType::LitChar,
            '\0'.to_string(),
            token_line,
            token_column,
        ))
    } else if let Ok((result_lexeme, result_index)) =
        lexeme::special_characters(&cfpl_source_code.vec, index)
    {
        index = result_index + 1;
        match lexeme::is_single_quote(source_code[result_index + 1]) {
            true => Ok(token::Token::new(
                token_type::TokenType::LitChar,
                result_lexeme,
                token_line,
                token_column,
            )),
            false => Err(index),
        }
    } else if lexeme::is_single_quote(source_code[index + 1]) {
        index += 1;
        Ok(token::Token::new(
            token_type::TokenType::LitChar,
            source_code[index - 1].to_string(),
            token_line,
            token_column,
        ))
    } else {
        Err(index)
    };
    let get_char_lit_error = |error_index: usize| -> String {
        if let Some(character_literal_closing) = source_code[(unmod_index + 1)..]
            .iter()
            .position(|&elem| elem == '\'')
        {
            let index_wtr_global = unmod_index + character_literal_closing + 1;
            cfpl_source_code.error_string_manual(
                token_line,
                token_column + (error_index - unmod_index),
                if index_wtr_global - error_index <= 10 {
                    source_code[unmod_index..=index_wtr_global]
                        .iter()
                        .collect::<String>()
                } else {
                    let mut ellipse_character_literal = source_code[unmod_index..=error_index]
                        .iter()
                        .collect::<String>();
                    ellipse_character_literal.push_str("...");
                    ellipse_character_literal.push_str(
                        &source_code[(index_wtr_global - 3)..=index_wtr_global]
                            .iter()
                            .collect::<String>(),
                    );
                    ellipse_character_literal
                },
                "Invalid character literal".to_string(),
            )
        } else {
            cfpl_source_code.error_string_manual(
                token_line,
                token_column,
                {
                    let mut ellipse_character_literal =
                        cfpl_source_code.get_code_at_line(token_line);
                    ellipse_character_literal.push_str("...");
                    ellipse_character_literal
                },
                "Unclosed character literal".to_string(),
            )
        }
    };
    match result {
        Ok(result) => {
            tokens.push(result);
            Ok(index)
        }
        Err(error_index) => Err(get_char_lit_error(error_index)),
    }
}
