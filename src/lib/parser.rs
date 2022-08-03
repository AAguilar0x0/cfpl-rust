use std::collections::HashMap;

use crate::{
    data_type::DataType,
    expression::{literal::Literal, Expression},
    statement::{
        var::{Var, VarDec},
        Statement,
    },
    token::Token,
    token_type::TokenType,
};

use super::{source_code, statement, token};

pub struct Parser<'a> {
    var_declarations: bool,
    is_declaring: bool,
    in_control_structure: bool,
    in_scope: bool,
    scope_counter: usize,
    current_index: usize,
    variable_type: HashMap<String, DataType>,
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

    fn compare_then_next(&mut self, types: &[&TokenType]) -> bool {
        let mut result = false;
        for current_type in types {
            if self.compare_current(current_type) {
                self.next();
                result = true;
                break;
            }
        }
        return result;
    }

    fn expect_then_next(&mut self, types: &[&TokenType], message: &str) -> Result<&Token, String> {
        if self.compare_then_next(types) {
            return Ok(self.get_previous().unwrap());
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
        self.expect_then_next(&[token_type], message)?;
        self.expect_then_next(
            &[&TokenType::Eol],
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
        return if self.compare_then_next(&[&TokenType::RkwVar]) {
            self.variable_declaration()
        } else {
            todo!();
        };
    }

    fn variable_declaration(&mut self) -> Result<Box<dyn Statement>, String> {
        if !self.is_declaring {
            self.is_declaring = true;
        }
        if !self.var_declarations {
            return Err(self.source_code.error_string_token(
                self.get_previous().unwrap(),
                "Misplaced variable declaration.",
            ));
        }
        let mut name;
        if self.compare_current(&TokenType::Identifier) {
            name = self.get_current().clone();
        } else if TokenType::is_reserved_keyword(&self.get_current().token_type) {
            return Err(self.source_code.error_string_token(
                self.get_current(),
                "Expected valid variable name but got reserved keyword.",
            ));
        } else {
            return Err(self
                .source_code
                .error_string_token(self.get_current(), "Expected valid variable name."));
        }

        let temp_current_index = self.current_index;
        while !self.compare_then_next(&[&TokenType::RkwAs, &TokenType::Eol, &TokenType::RkwStart]) {
            self.current_index += 1;
        }
        let token_data_type = if !self.compare_then_next(&[&TokenType::RkwBool]) {
            self.get_previous().unwrap().clone()
        } else {
            return Err(self
                .source_code
                .error_string_token(&name, "Expected declaration variable data type."));
        };
        self.current_index = temp_current_index;

        let mut variable_declarations = Vec::new();

        let mut push_declaration = |parser: &mut Parser, name: Token| {
            if parser.variable_type.contains_key(&name.lexeme) {
                return Err(parser.source_code.error_string_token(
                    &name,
                    &format!("Variable name '{}' is already declared.", &name.lexeme),
                ));
            } else {
                parser.variable_type.insert(
                    name.lexeme.clone(),
                    DataType::get_token_data_type(&token_data_type).unwrap(),
                );
            }

            let mut initializer;
            if parser.compare_then_next(&[&TokenType::SymAssignment]) {
                initializer = parser.expression()?;
                if let Some(literal) = initializer.as_any().downcast_ref::<Literal>() {
                    if token_data_type.token_type == TokenType::RkwFloat
                        && DataType::any_to_data_type(&literal.value).unwrap() == DataType::INT
                    {
                        let value = *literal.as_any().downcast_ref::<i32>().unwrap();
                        initializer = Box::new(Literal {
                            value: Box::new(value),
                        });
                    } else {
                        return Err(parser.source_code.error_string_token(
                            &name,
                            &format!("Expected {} type.", token_data_type.lexeme),
                        ));
                    }
                }
            } else {
                initializer = Box::new(Literal {
                    value: Box::new(DataType::get_default_of_type(&token_data_type.token_type)),
                });
            }

            variable_declarations.push(VarDec { name, initializer });
            return Ok(());
        };

        push_declaration(self, name)?;

        while self.compare_then_next(&[&TokenType::SymComma]) {
            name = self
                .expect_then_next(&[&TokenType::Identifier], "Expected variable name.")?
                .clone();
            push_declaration(self, name)?;
        }

        let expect_data_type_error = "Expected declaration variable data type.";
        self.expect_then_next(&[&TokenType::RkwAs], expect_data_type_error)?;
        self.expect_then_next(
            &[
                &TokenType::RkwBool,
                &TokenType::RkwChar,
                &TokenType::RkwFloat,
                &TokenType::RkwInt,
            ],
            expect_data_type_error,
        )?;

        return Ok(Box::new(Var {
            variable_declarations,
        }));
    }

    fn expression(&mut self) -> Result<Box<dyn Expression>, String> {
        todo!()
    }
}
