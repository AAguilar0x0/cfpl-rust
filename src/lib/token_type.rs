use std::fmt;

#[derive(Clone, PartialEq, Debug)]
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

impl TokenType {
    pub fn is_reserved_keyword(token_type: &TokenType) -> bool {
        return matches!(
            *token_type,
            TokenType::RkwAnd
                | TokenType::RkwOr
                | TokenType::RkwNot
                | TokenType::RkwOutput
                | TokenType::RkwInput
                | TokenType::RkwVar
                | TokenType::RkwAs
                | TokenType::RkwInt
                | TokenType::RkwBool
                | TokenType::RkwFloat
                | TokenType::RkwChar
                | TokenType::RkwStart
                | TokenType::RkwStop
                | TokenType::RkwIf
                | TokenType::RkwElse
                | TokenType::RkwWhile
        );
    }
}
