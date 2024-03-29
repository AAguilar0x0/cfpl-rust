#![allow(unreachable_patterns)]
use super::token_type::TokenType;
use std::collections::HashSet;

pub fn static_lexeme_to_token_type(lexeme: &str) -> Result<TokenType, String> {
    match lexeme {
        "(" => Ok(TokenType::SymLeftParenthesis),
        ")" => Ok(TokenType::SymRightParenthesis),
        "," => Ok(TokenType::SymComma),
        "=" => Ok(TokenType::SymAssignment),
        ":" => Ok(TokenType::SymColon),
        "&" => Ok(TokenType::SymAmpersand),
        "+" => Ok(TokenType::SymPlus),
        "-" => Ok(TokenType::SymMinus),
        "*" => Ok(TokenType::SymStar),
        "/" => Ok(TokenType::SymForwardSlash),
        "%" => Ok(TokenType::SymPercent),
        ">" => Ok(TokenType::SymGreater),
        "<" => Ok(TokenType::SymLesser),
        ">=" => Ok(TokenType::SymGreaterEqual),
        "<=" => Ok(TokenType::SymLesserEqual),
        "==" => Ok(TokenType::SymEqual),
        "<>" => Ok(TokenType::SymNotEqual),
        "AND" => Ok(TokenType::RkwAnd),
        "OR" => Ok(TokenType::RkwOr),
        "NOT" => Ok(TokenType::RkwNot),
        "OUTPUT" => Ok(TokenType::RkwOutput),
        "INPUT" => Ok(TokenType::RkwInput),
        "VAR" => Ok(TokenType::RkwVar),
        "AS" => Ok(TokenType::RkwAs),
        "INT" => Ok(TokenType::RkwInt),
        "BOOL" => Ok(TokenType::RkwBool),
        "FLOAT" => Ok(TokenType::RkwFloat),
        "CHAR" => Ok(TokenType::RkwChar),
        "START" => Ok(TokenType::RkwStart),
        "STOP" => Ok(TokenType::RkwStop),
        "IF" => Ok(TokenType::RkwIf),
        "ELSE" => Ok(TokenType::RkwElse),
        "WHILE" => Ok(TokenType::RkwWhile),
        _ => Err(format!("Invalid lexeme {}.", lexeme.escape_debug())),
    }
}

pub fn possibly_equal_assignment(
    source_code_vec: &[char],
    index: usize,
) -> (TokenType, String, usize) {
    if index + 1 < source_code_vec.len() && source_code_vec[index + 1] == '=' {
        (TokenType::SymEqual, "==".to_string(), index + 1)
    } else {
        (TokenType::SymAssignment, "=".to_string(), index)
    }
}

pub fn possibly_lesser_lesser_equal_notequal(
    source_code_vec: &[char],
    index: usize,
) -> (TokenType, String, usize) {
    let is_not_eof = index + 1 < source_code_vec.len();
    if is_not_eof && source_code_vec[index + 1] == '=' {
        (TokenType::SymLesserEqual, "<=".to_string(), index + 1)
    } else if is_not_eof && source_code_vec[index + 1] == '>' {
        (TokenType::SymNotEqual, "<>".to_string(), index + 1)
    } else {
        (TokenType::SymLesser, "<".to_string(), index)
    }
}

pub fn possibly_greater_greater_equal(
    source_code_vec: &[char],
    index: usize,
) -> (TokenType, String, usize) {
    if index + 1 < source_code_vec.len() && source_code_vec[index + 1] == '=' {
        (TokenType::SymGreaterEqual, ">=".to_string(), index + 1)
    } else {
        (TokenType::SymGreater, ">".to_string(), index)
    }
}

pub fn is_single_quote(lexeme: char) -> bool {
    matches!(
        lexeme,
        '\'' | '`' | '‘' | '\u{2018}' | '’' | '\u{2019}' | '‛' | '\u{201B}'
    )
}

pub fn is_double_quote(lexeme: char) -> bool {
    matches!(
        lexeme,
        '"' | '“' | '\u{201C}' | '”' | '\u{201D}' | '‟' | '\u{201F}'
    )
}

fn evaluate_dfa(
    source_code_vec: &[char],
    mut index: usize,
    initial_state: usize,
    transition_table: &[&[usize]],
    final_state: &HashSet<usize>,
    dead_state: &HashSet<usize>,
    map_char_to_int: fn(char) -> isize,
    terminate_if_any: bool,
) -> Result<(usize, usize), (usize, usize)> {
    let mut current_state = initial_state;
    while index < source_code_vec.len()
        && !dead_state.contains(&current_state)
        && (terminate_if_any || !final_state.contains(&current_state))
    {
        let mapped_result = map_char_to_int(source_code_vec[index]);
        if terminate_if_any && mapped_result == -1 {
            break;
        }
        current_state = transition_table[current_state][mapped_result as usize];
        index += 1;
    }
    let result = (current_state, index - 1);
    if !terminate_if_any && index == source_code_vec.len() {
        return Err(result);
    }
    Ok(result)
}

pub enum SpecialCharError {
    InvalidEscape,
    UnclosedEscape,
    InvalidSpecialChar,
}

fn escape_character_dfa(
    source_code_vec: &[char],
    index: usize,
) -> Result<(String, usize), (String, usize, SpecialCharError)> {
    let transition_table: &[&[usize]] = &[
        //[, *, ]
        &[1, 4, 4], // 0
        &[2, 2, 2], // 1
        &[5, 5, 3], // 2
        &[3, 3, 3], // 3
        &[4, 4, 4], // 4
        &[5, 5, 5], // 5
    ];
    let final_state: HashSet<usize> = HashSet::from([3]);
    let dead_state: HashSet<usize> = HashSet::from([4, 5]);
    let map_char_to_int = |alphabet: char| match alphabet {
        '[' => 0,
        ']' => 2,
        _ => 1,
    };
    match evaluate_dfa(
        source_code_vec,
        index,
        0,
        transition_table,
        &final_state,
        &dead_state,
        map_char_to_int,
        false,
    ) {
        Ok((result_state, result_index)) => match final_state.contains(&result_state) {
            true => Ok((source_code_vec[result_index - 1].to_string(), result_index)),
            false => Err((
                "Invalid escape.".to_string(),
                result_index,
                SpecialCharError::InvalidEscape,
            )),
        },
        Err((_, error_index)) => Err((
            "Unclosed escape.".to_string(),
            error_index,
            SpecialCharError::UnclosedEscape,
        )),
    }
}

pub fn special_characters(
    source_code_vec: &[char],
    index: usize,
) -> Result<(String, usize), (String, usize, SpecialCharError)> {
    match source_code_vec[index] {
        '[' | ']' => escape_character_dfa(source_code_vec, index),
        '#' => Ok(("\n".to_string(), index)),
        _ => Err((
            "Invalid special character.".to_string(),
            index,
            SpecialCharError::InvalidSpecialChar,
        )),
    }
}

pub fn bool_dfa(
    source_code_vec: &[char],
    index: usize,
) -> Result<(String, usize), (String, usize)> {
    let transition_table: &[&[usize]] = &[
        //F, A, L, S, E, T, R, U, "
        &[1, 9, 9, 9, 9, 7, 9, 9, 9], // 0
        &[9, 2, 9, 9, 9, 9, 9, 9, 9], // 1
        &[9, 9, 3, 9, 9, 9, 9, 9, 9], // 2
        &[9, 9, 9, 4, 9, 9, 9, 9, 9], // 3
        &[9, 9, 9, 9, 5, 9, 9, 9, 9], // 4
        &[9, 9, 9, 9, 9, 9, 9, 9, 6], // 5
        &[6, 6, 6, 6, 6, 6, 6, 6, 6], // 6
        &[9, 9, 9, 9, 9, 9, 8, 9, 9], // 7
        &[9, 9, 9, 9, 9, 9, 9, 4, 9], // 8
        &[9, 9, 9, 9, 9, 9, 9, 9, 9], // 9
    ];
    let final_state: HashSet<usize> = HashSet::from([6]);
    let dead_state: HashSet<usize> = HashSet::from([9]);
    let map_char_to_int = |alphabet: char| match alphabet {
        'F' => 0,
        'A' => 1,
        'L' => 2,
        'S' => 3,
        'E' => 4,
        'T' => 5,
        'R' => 6,
        'U' => 7,
        other => {
            if is_double_quote(other) {
                8
            } else {
                -1
            }
        }
    };
    match evaluate_dfa(
        source_code_vec,
        index + 1,
        0,
        transition_table,
        &final_state,
        &dead_state,
        map_char_to_int,
        true,
    ) {
        Ok((result_state, result_index)) => match final_state.contains(&result_state) {
            true => Ok((
                source_code_vec[(index + 1)..result_index]
                    .iter()
                    .collect::<String>(),
                result_index,
            )),
            false => Ok(("".to_string(), index)),
        },
        Err((_, error_index)) => Err(("Unclosed bool literal.".to_string(), error_index)),
    }
}

pub fn number_dfa(
    source_code_vec: &[char],
    index: usize,
) -> Result<(String, usize, TokenType), (String, usize)> {
    let transition_table: &[&[usize]] = &[
        //D, .
        &[1, 3], // 0
        &[1, 2], // 1
        &[4, 5], // 2
        &[4, 5], // 3
        &[4, 5], // 4
        &[5, 5], // 5
    ];
    let final_state: HashSet<usize> = HashSet::from([1, 2, 4]);
    let dead_state: HashSet<usize> = HashSet::from([5]);
    let map_char_to_int = |alphabet: char| match alphabet {
        '.' => 1,
        other => {
            if other.is_ascii_digit() {
                0
            } else {
                -1
            }
        }
    };
    match evaluate_dfa(
        source_code_vec,
        index,
        0,
        transition_table,
        &final_state,
        &dead_state,
        map_char_to_int,
        true,
    ) {
        Ok((result_state, result_index)) => match final_state.contains(&result_state) {
            true => Ok((
                source_code_vec[index..=result_index]
                    .iter()
                    .collect::<String>(),
                result_index,
                match result_state {
                    1 => TokenType::LitInt,
                    _ => TokenType::LitFloat,
                },
            )),
            false => Err(("Invalid number literal.".to_string(), result_index)),
        },
        Err((_, error_index)) => Err(("Unclosed code block.".to_string(), error_index)),
    }
}

pub fn words_dfa(
    source_code_vec: &[char],
    index: usize,
) -> Result<(String, usize, TokenType), (String, usize)> {
    let transition_table: &[&[usize]] = &[
        //_, $, A, D
        &[1, 1, 1, 2], // 0
        &[1, 1, 1, 1], // 1
        &[2, 2, 2, 2], // 2
    ];
    let final_state: HashSet<usize> = HashSet::from([1]);
    let dead_state: HashSet<usize> = HashSet::from([2]);
    let map_char_to_int = |alphabet: char| match alphabet {
        '_' => 0,
        '$' => 1,
        other => {
            if other.is_ascii_alphabetic() {
                2
            } else if other.is_ascii_digit() {
                3
            } else {
                -1
            }
        }
    };
    match evaluate_dfa(
        source_code_vec,
        index,
        0,
        transition_table,
        &final_state,
        &dead_state,
        map_char_to_int,
        true,
    ) {
        Ok((result_state, result_index)) => match final_state.contains(&result_state) {
            true => {
                let lexeme = source_code_vec[index..=result_index]
                    .iter()
                    .collect::<String>();
                match static_lexeme_to_token_type(lexeme.as_str()) {
                    Ok(token_type) => Ok((lexeme, result_index, token_type)),
                    Err(_) => Ok((lexeme, result_index, TokenType::Identifier)),
                }
            }
            false => Err(("Invalid syntax.".to_string(), result_index)),
        },
        Err((_, error_index)) => Err(("Invalid syntax.".to_string(), error_index)),
    }
}
