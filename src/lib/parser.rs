use std::collections::HashMap;

use crate::{statement::Statement, token::Token, token_type::TokenType};

use super::{source_code, statement, token, token_type};

pub struct Parser<'a> {
    var_declarations: bool,
    is_declaring: bool,
    in_control_structure: bool,
    in_scope: bool,
    scope_counter: usize,
    current_index: usize,
    variable_type: HashMap<String, token_type::TokenType>,
    source_code: &'a source_code::SourceCode,
    tokens: &'a Vec<token::Token>,
}

impl<'a> Parser<'a> {
    pub fn syntax_analysis(
        source_code: &'a source_code::SourceCode,
        tokens: &'a Vec<token::Token>,
    ) -> Result<Vec<Box<dyn statement::Statement>>, String> {
        let mut parser = Parser {
            var_declarations: false,
            is_declaring: false,
            in_control_structure: false,
            in_scope: false,
            scope_counter: 0,
            current_index: 0,
            variable_type: HashMap::new(),
            source_code,
            tokens,
        };
        let mut statements = Vec::new();

        while !parser.is_at_end() {
            statements.push(parser.declaration()?);
        }

        return Ok(statements);
    }

    fn next(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current_index += 1;
        }
        return self.get_previous().unwrap();
    }

    fn get_previous(&self) -> Option<&Token> {
        if self.current_index == 0 {
            return None;
        } else {
            return Some(&self.tokens[self.current_index - 1]);
        }
    }

    fn get_current(&self) -> &Token {
        return &self.tokens[self.current_index];
    }

    fn is_at_end(&self) -> bool {
        return self.get_current().token_type == TokenType::Eof;
    }

    fn compare_current(&self, current_type: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        return self.get_current().token_type == *current_type;
    }

    fn compare_multiple_then_next(&mut self, types: &[TokenType]) -> bool {
        let mut result = false;
        for current_type in types {
            if (self.compare_current(current_type)) {
                self.next();
                result = true;
                break;
            }
        }
        return result;
    }

    fn expect_then_next(
        &mut self,
        token_type: &TokenType,
        message: &str,
    ) -> Result<&Token, String> {
        if self.compare_current(token_type) {
            return Ok(self.next());
        }
        return Err(self
            .source_code
            .error_string_token(self.get_current(), message));
    }

    fn expect_token_and_eol_next(
        &mut self,
        token_type: &TokenType,
        message: &str,
    ) -> Result<(), String> {
        self.expect_then_next(token_type, message)?;
        self.expect_then_next(
            &TokenType::Eol,
            &format!("Missing new line after \'{}\'", token_type),
        )?;
        return Ok(());
    }

    fn expect_token_and_eol(&mut self, token_type: TokenType, message: &str) -> Result<(), String> {
        let temp_current_index = self.current_index;
        self.expect_token_and_eol_next(&token_type, message)?;
        self.current_index = temp_current_index;
        return Ok(());
    }

    fn declaration(&mut self) -> Result<Box<dyn Statement>, String> {
        return if self.compare_multiple_then_next(&[TokenType::RkwVar]) {
            self.variable_declaration()
        } else {
            todo!();
        };
    }

    fn variable_declaration(&self) -> Result<Box<dyn Statement>, String> {
        todo!();
    }
}
