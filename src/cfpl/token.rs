use super::token_type::*;
use std::fmt;

pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: u128,
    pub column: u128,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} '{}'", self.token_type, self.lexeme)
    }
}
