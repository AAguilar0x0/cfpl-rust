use std::fmt;

#[derive(PartialEq, Debug)]
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
