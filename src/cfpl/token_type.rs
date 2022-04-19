#![allow(unreachable_patterns)]
use std::fmt;

#[derive(Debug)]
pub enum TokenType {
    SymLeftParenthesis,
    SymRightParenthesis,

    SymComma,
    SymAssignment,
    SymColon,
    SymAmpersand,
    SymPlus,
    SymMinus,
    SymStar,
    SymForwardSlash,
    SymPercent,

    SymGreater,
    SymLesser,
    SymGreaterEqual,
    SymLesserEqual,
    SymEqual,
    SymNotEqual,

    Identifier,

    LitChar,
    LitInt,
    LitFloat,
    LitBool,
    LitStr,

    RkwAnd,
    RkwOr,
    RkwNot,
    RkwOutput,
    RkwInput,
    RkwVar,
    RkwAs,
    RkwInt,
    RkwBool,
    RkwFloat,
    RkwChar,
    RkwStart,
    RkwStop,
    RkwIf,
    RkwElse,
    RkwWhile,
    Eol,
    Eof,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub fn get_token_type_from_static(lexeme: String) -> Result<TokenType, String> {
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

pub fn is_single_quote(lexeme: &str) -> bool {
    match lexeme {
        "'" | "`" => true,
        "‘" | "\u{2018}" => true,
        "’" | "\u{2019}" => true,
        "‛" | "\u{201B}" => true,
        _ => false,
    }
}

pub fn is_double_quote(lexeme: &str) -> bool {
    match lexeme {
        "\"" => true,
        "“" | "\u{201C}" => true,
        "”" | "\u{201D}" => true,
        "‟" | "\u{201F}" => true,
        _ => false,
    }
}
