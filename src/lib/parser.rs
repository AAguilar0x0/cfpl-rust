use std::collections::HashMap;

use super::{source_code, statement, token, token_type};

pub struct Parser<'a> {
    var_declarations: bool,
    is_declaring: bool,
    in_control_structure: bool,
    in_scope: bool,
    scope_counter: usize,
    current: usize,
    statements: Vec<Box<dyn statement::Statement>>,
    variable_type: HashMap<String, token_type::TokenType>,
    source_code: &'a source_code::SourceCode,
    tokens: &'a Vec<token::Token>,
}

impl<'a> Parser<'a> {
    pub fn syntax_analysis(
        source_code: &'a source_code::SourceCode,
        tokens: &'a Vec<token::Token>,
    ) -> Result<Vec<Box<dyn statement::Statement>>, String> {
        let parser = Parser {
            var_declarations: false,
            is_declaring: false,
            in_control_structure: false,
            in_scope: false,
            scope_counter: 0,
            current: 0,
            statements: Vec::new(),
            variable_type: HashMap::new(),
            source_code,
            tokens,
        };

        parser.parse()?;

        return Ok(parser.statements);
    }

    fn parse(&self) -> Result<(), String> {
        todo!();
    }
}
