#![allow(unreachable_patterns)]
use super::token_type::TokenType;
use std::collections::HashSet;

pub fn static_lexeme_to_token_type(lexeme: String) -> Result<TokenType, String> {
    match lexeme.as_str() {
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
    source_code_vec: &Vec<char>,
    index: usize,
) -> (TokenType, String, usize) {
    if index + 1 < source_code_vec.len() && source_code_vec[index + 1] == '=' {
        (TokenType::SymEqual, "==".to_string(), index + 1)
    } else {
        (TokenType::SymAssignment, "=".to_string(), index)
    }
}

pub fn possibly_lesser_lesser_equal_notequal(
    source_code_vec: &Vec<char>,
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
    source_code_vec: &Vec<char>,
    index: usize,
) -> (TokenType, String, usize) {
    if index + 1 < source_code_vec.len() && source_code_vec[index + 1] == '=' {
        (TokenType::SymGreaterEqual, ">=".to_string(), index + 1)
    } else {
        (TokenType::SymGreater, ">".to_string(), index)
    }
}

pub fn is_single_quote(lexeme: char) -> bool {
    match lexeme {
        '\'' | '`' => true,
        '‘' | '\u{2018}' => true,
        '’' | '\u{2019}' => true,
        '‛' | '\u{201B}' => true,
        _ => false,
    }
}

pub fn is_double_quote(lexeme: char) -> bool {
    match lexeme {
        '"' => true,
        '“' | '\u{201C}' => true,
        '”' | '\u{201D}' => true,
        '‟' | '\u{201F}' => true,
        _ => false,
    }
}

fn evaluate_dfa(
    source_code_vec: &Vec<char>,
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
    if index == source_code_vec.len() {
        return Err(result);
    }
    Ok(result)
}

fn escape_character(
    source_code_vec: &Vec<char>,
    index: usize,
) -> Result<(String, usize), (String, usize)> {
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
            false => Err(("Invalid escape.".to_string(), 1)),
        },
        Err(_) => Err(("Unclosed escape.".to_string(), 0)),
    }
}

pub fn special_characters(
    source_code_vec: &Vec<char>,
    index: usize,
) -> Result<(String, usize), (String, usize)> {
    match source_code_vec[index] {
        '[' | ']' => escape_character(source_code_vec, index),
        '#' => Ok(("\n".to_string(), index)),
        _ => Err(("Invalid special character.".to_string(), 2)),
    }
}
