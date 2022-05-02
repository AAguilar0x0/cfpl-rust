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
    let mut _debug_current_character = source_code[i];
    let mut _debug_current_column = COLUMN.with(|column| column.get());
    let mut _debug_current_line = LINE.with(|line| line.get());
    let mut _debug_first_in_line = FIRST_IN_LINE.with(|first_in_line| first_in_line.get());
    while i < source_code.len() {
        _debug_current_character = source_code[i];
        _debug_current_column = COLUMN.with(|column| column.get());
        _debug_current_line = LINE.with(|line| line.get());
        _debug_first_in_line = FIRST_IN_LINE.with(|first_in_line| first_in_line.get());
        let (index_result, column_result, is_override_fil): (usize, Result<usize, usize>, bool) =
            match source_code[i] {
                '\n' => {
                    FIRST_IN_LINE.with(|first_in_line| {
                        if !first_in_line.get() {
                            first_in_line.set(true);
                            let token_line = LINE.with(|line| line.get());
                            tokens.push(token::Token::new(
                                token_type::TokenType::Eol,
                                String::from("EOL"),
                                token_line + 1,
                                0,
                            ))
                        }
                    });
                    LINE.with(|line| line.set(line.get() + 1));
                    (0, Err(0), false)
                }
                '(' => {
                    single_symbol(cfpl_source_code, &mut tokens, i)?;
                    (0, Ok(0), false)
                }
                ')' => {
                    single_symbol(cfpl_source_code, &mut tokens, i)?;
                    (0, Ok(0), false)
                }
                ',' => {
                    single_symbol(cfpl_source_code, &mut tokens, i)?;
                    (0, Ok(0), false)
                }
                ':' => {
                    single_symbol(cfpl_source_code, &mut tokens, i)?;
                    (0, Ok(0), false)
                }
                '&' => {
                    single_symbol(cfpl_source_code, &mut tokens, i)?;
                    (0, Ok(0), false)
                }
                '+' => {
                    single_symbol(cfpl_source_code, &mut tokens, i)?;
                    (0, Ok(0), false)
                }
                '-' => {
                    single_symbol(cfpl_source_code, &mut tokens, i)?;
                    (0, Ok(0), false)
                }
                '/' => {
                    single_symbol(cfpl_source_code, &mut tokens, i)?;
                    (0, Ok(0), false)
                }
                '%' => {
                    single_symbol(cfpl_source_code, &mut tokens, i)?;
                    (0, Ok(0), false)
                }
                '*' => {
                    if FIRST_IN_LINE.with(|first_in_line| first_in_line.get()) {
                        let index = comment_line(source_code, i);
                        LINE.with(|line| line.set(line.get() + 1));
                        FIRST_IN_LINE.with(|first_in_line| first_in_line.set(true));
                        (index - i, Err(0), true)
                    } else {
                        single_symbol(cfpl_source_code, &mut tokens, i)?;
                        (0, Ok(0), false)
                    }
                }
                '=' => {
                    let index = single_double_symbol(
                        source_code,
                        &mut tokens,
                        i,
                        lexeme::possibly_equal_assignment,
                    );
                    (index - i, Ok(index - i), false)
                }
                '<' => {
                    let index = single_double_symbol(
                        source_code,
                        &mut tokens,
                        i,
                        lexeme::possibly_lesser_lesser_equal_notequal,
                    );
                    (index - i, Ok(index - i), false)
                }
                '>' => {
                    let index = single_double_symbol(
                        source_code,
                        &mut tokens,
                        i,
                        lexeme::possibly_greater_greater_equal,
                    );
                    (index - i, Ok(index - i), false)
                }
                other => {
                    if lexeme::is_single_quote(other) {
                        let index = character_literal(cfpl_source_code, &mut tokens, i)?;
                        (index - i, Ok(index - i), false)
                    } else if lexeme::is_double_quote(other) {
                        if let Some(index_result) = bool_literal(cfpl_source_code, &mut tokens, i) {
                            (index_result - i, Ok(index_result - i), false)
                        } else {
                            let (index_result, line_result, column_result) =
                                string_literal(cfpl_source_code, &mut tokens, i)?;
                            LINE.with(|line| line.set(line_result));
                            (index_result - i, Err(column_result + 1), false)
                        }
                    } else if source_code[i] == '.' || source_code[i].is_digit(10) {
                        let index = number_literal(cfpl_source_code, &mut tokens, i)?;
                        (index - i, Ok(index - i), false)
                    } else if source_code[i] == '_'
                        || source_code[i] == '$'
                        || source_code[i].is_ascii_alphabetic()
                    {
                        let index = words(cfpl_source_code, &mut tokens, i)?;
                        (index - i, Ok(index - i), false)
                    } else if source_code[i].is_whitespace() {
                        (0, Ok(0), false)
                    } else {
                        let token_line = LINE.with(|line| line.get());
                        let token_column = COLUMN.with(|column| column.get());
                        return Err(cfpl_source_code.error_string_manual(
                            token_line,
                            token_column,
                            String::from(other),
                            "Invalid character token.".to_string(),
                        ));
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
    let token_type_here = match lexeme::static_lexeme_to_token_type(
        String::from(cfpl_source_code.vec[index]).as_str(),
    ) {
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

fn comment_line(source_code: &[char], mut index: usize) -> usize {
    while index < source_code.len() && source_code[index] != '\n' {
        index += 1;
    }
    index
}

fn single_double_symbol(
    source_code: &[char],
    tokens: &mut Vec<token::Token>,
    index: usize,
    get_some_token_value: fn(&[char], usize) -> (token_type::TokenType, String, usize),
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

fn bool_literal(
    cfpl_source_code: &source_code::SourceCode,
    tokens: &mut Vec<token::Token>,
    index: usize,
) -> Option<usize> {
    let token_line = LINE.with(|line| line.get());
    let token_column = COLUMN.with(|column| column.get());
    match lexeme::bool_dfa(&cfpl_source_code.vec, index) {
        Ok((lexeme_result, index_result)) => match index_result > index {
            true => {
                tokens.push(token::Token::new(
                    token_type::TokenType::LitBool,
                    lexeme_result,
                    token_line,
                    token_column,
                ));
                Some(index_result)
            }
            false => None,
        },
        Err(_) => None,
    }
}

fn string_literal(
    cfpl_source_code: &source_code::SourceCode,
    tokens: &mut Vec<token::Token>,
    mut index: usize,
) -> Result<(usize, usize, usize), String> {
    let token_line = LINE.with(|line| line.get());
    let token_column = COLUMN.with(|column| column.get());
    let source_code = &cfpl_source_code.vec;
    let mut literal_value = String::new();
    let start_index = index;
    let mut line = token_line;
    let mut column = token_column + 1;
    index += 1;
    while index < source_code.len() {
        if source_code[index] == '\n' {
            line += 1;
            column = 0;
        }
        if lexeme::is_double_quote(source_code[index]) {
            break;
        }
        literal_value.push_str(
            match lexeme::special_characters(source_code, index) {
                Ok((lexeme_result, index_result)) => {
                    column += index_result - index;
                    index = index_result;
                    lexeme_result
                }
                Err((error_message, error_index, error_type)) => {
                    match matches!(error_type, lexeme::SpecialCharError::InvalidSpecialChar) {
                        true => source_code[index].to_string(),
                        false => {
                            return Err(cfpl_source_code.error_string_manual(
                                line,
                                column + error_index - start_index,
                                {
                                    let mut error_code = source_code[start_index..=error_index]
                                        .iter()
                                        .collect::<String>();
                                    error_code.push_str("...");
                                    error_code
                                },
                                error_message,
                            ))
                        }
                    }
                }
            }
            .as_str(),
        );
        index += 1;
        column += 1;
    }
    tokens.push(token::Token::new(
        token_type::TokenType::LitStr,
        literal_value,
        token_line,
        token_column,
    ));
    Ok((index, line, column))
}

fn number_literal(
    cfpl_source_code: &source_code::SourceCode,
    tokens: &mut Vec<token::Token>,
    index: usize,
) -> Result<usize, String> {
    let token_line = LINE.with(|line| line.get());
    let token_column = COLUMN.with(|column| column.get());
    match lexeme::number_dfa(&cfpl_source_code.vec, index) {
        Ok((lexeme_result, index_result, type_result)) => {
            tokens.push(token::Token::new(
                type_result,
                lexeme_result,
                token_line,
                token_column,
            ));
            Ok(index_result)
        }
        Err((error_message, _)) => Err(error_message),
    }
}

fn words(
    cfpl_source_code: &source_code::SourceCode,
    tokens: &mut Vec<token::Token>,
    index: usize,
) -> Result<usize, String> {
    let token_line = LINE.with(|line| line.get());
    let token_column = COLUMN.with(|column| column.get());
    match lexeme::words_dfa(&cfpl_source_code.vec, index) {
        Ok((lexeme_result, index_result, type_result)) => {
            tokens.push(token::Token::new(
                type_result,
                lexeme_result,
                token_line,
                token_column,
            ));
            Ok(index_result)
        }
        Err((error_message, _)) => Err(error_message),
    }
}
