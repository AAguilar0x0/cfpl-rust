#![allow(dead_code)]
use std::fmt;

#[derive(Debug)]
pub enum TokenType {
    SymLeftParenthesis,
    SymRightParenthesis,
    SymLeftBrace,
    SymRightBrace,

    SymComma,
    SymAssignment,
    SymColon,
    SymOctothorpe,
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
