use std::collections::HashMap;

use crate::{
    data_type::DataType,
    expression::{
        assign::Assign, binary::Binary, grouping::Grouping, literal::Literal, logical::Logical,
        unary::Unary, variable::Variable, Expression,
    },
    source_code,
    statement::{
        self, block::Block, if_stmt::If, input::Input, print::Print, var::Var, var_dec::VarDec,
        while_stmt::While, Statement,
    },
    token::{self, Token},
    token_type::TokenType,
};

pub struct Parser<'a> {
    var_declarations: bool,
    declaring: bool,
    in_control_structure: bool,
    in_scope: bool,
    scope_counter: usize,
    current_index: usize,
    variable_type: HashMap<String, DataType>,
    source_code: &'a source_code::SourceCode,
    tokens: &'a Vec<token::Token>,
}

impl Parser<'_> {
    pub fn syntax_analysis(
        source_code: &source_code::SourceCode,
        tokens: &Vec<token::Token>,
    ) -> Result<Vec<Box<dyn statement::Statement>>, String> {
        let mut parser = Parser {
            var_declarations: true,
            declaring: false,
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
            if self.compare_current(*current_type) {
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
            &format!("Missing new line after \'{}\'.", token_type),
        )?;
        return Ok(());
    }

    fn expect_token_and_eol(
        &mut self,
        token_type: &TokenType,
        message: &str,
    ) -> Result<(), String> {
        let temp_current_index = self.current_index;
        self.expect_token_and_eol_next(token_type, message)?;
        self.current_index = temp_current_index;
        return Ok(());
    }

    fn expect_logical_expressions(&self, expression: &dyn Expression) -> Result<(), String> {
        if let Some(expression) = expression.as_any().downcast_ref::<Grouping>() {
            return self.expect_logical_expressions(&*expression.expression);
        }
        if let Some(expression) = expression.as_any().downcast_ref::<Unary>() {
            if expression.operator.token_type == TokenType::RkwNot {
                return self.expect_logical_expressions(&*expression.right);
            }
        }
        if expression.as_any().downcast_ref::<Logical>().is_some() {
            return Ok(());
        }
        let erroneous = self.get_previous().unwrap();
        let result = if let Some(expression) = expression.as_any().downcast_ref::<Binary>() {
            match expression.operator.token_type {
                TokenType::SymLesser
                | TokenType::SymLesserEqual
                | TokenType::SymGreater
                | TokenType::SymGreaterEqual
                | TokenType::SymEqual
                | TokenType::SymNotEqual
                | TokenType::RkwAnd
                | TokenType::RkwOr
                | TokenType::RkwNot => Ok(()),
                _ => Err(&expression.operator),
            }
        } else if let Some(expression) = expression.as_any().downcast_ref::<Literal>() {
            match DataType::box_any_to_data_type(&expression.value) {
                Some(data_type) if data_type == DataType::BOOL => Ok(()),
                _ => Err(erroneous),
            }
        } else if let Some(expression) = expression.as_any().downcast_ref::<Var>() {
            match self.variable_type.get(&expression.name.lexeme) {
                Some(data_type) if *data_type == DataType::BOOL => Ok(()),
                _ => Err(&expression.name),
            }
        } else {
            Err(erroneous)
        };

        return match result {
            Ok(_) => Ok(()),
            Err(operator) => Err(self
                .source_code
                .error_string_token(operator, "Expected BOOL evaluation result.")),
        };
    }

    fn declaration(&mut self) -> Result<Box<dyn Statement>, String> {
        return if self.compare_then_next(&[&TokenType::RkwVar]) {
            self.variable_declaration()
        } else {
            self.statement()
        };
    }

    fn variable_declaration(&mut self) -> Result<Box<dyn Statement>, String> {
        if !self.declaring {
            self.declaring = true;
        }
        if !self.var_declarations {
            return Err(self.source_code.error_string_token(
                self.get_previous().unwrap(),
                "Misplaced variable declaration.",
            ));
        }

        let get_identifier = |parser: &mut Parser| {
            return if parser.compare_then_next(&[&TokenType::Identifier]) {
                Ok(parser.get_previous().unwrap().clone())
            } else if TokenType::is_reserved_keyword(&parser.get_current().token_type) {
                Err(parser.source_code.error_string_token(
                    parser.get_current(),
                    "Expected valid variable name but got reserved keyword.",
                ))
            } else {
                Err(self
                    .source_code
                    .error_string_token(parser.get_current(), "Expected valid variable name."))
            };
        };
        let name = get_identifier(self)?;

        let temp_current_index = self.current_index;
        while !self.compare_then_next(&[&TokenType::RkwAs, &TokenType::Eol, &TokenType::RkwStart]) {
            self.current_index += 1;
        }
        let token_type = if self.compare_then_next(&[
            &TokenType::RkwBool,
            &TokenType::RkwChar,
            &TokenType::RkwFloat,
            &TokenType::RkwInt,
        ]) {
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
                    DataType::get_token_data_type(&token_type).unwrap(),
                );
            }

            let mut initializer;
            if parser.compare_then_next(&[&TokenType::SymAssignment]) {
                initializer = parser.expression()?;
                if let Some(literal) = (*initializer).as_any().downcast_ref::<Literal>() {
                    let value_data_type = DataType::box_any_to_data_type(&literal.value).unwrap();
                    if token_type.token_type == TokenType::RkwFloat
                        && value_data_type == DataType::INT
                    {
                        let value = *DataType::downcast_box_any::<i32>(&literal.value).unwrap();
                        initializer = Box::new(Literal {
                            value: Box::new(value),
                        });
                    } else if value_data_type != DataType::get_token_data_type(&token_type).unwrap()
                    {
                        return Err(parser.source_code.error_string_token(
                            &name,
                            &format!("Expected {} type.", token_type.lexeme),
                        ));
                    }
                }
            } else {
                initializer = Box::new(Literal {
                    value: Box::new(DataType::get_default_of_type(&token_type.token_type).unwrap()),
                });
            }

            variable_declarations.push(Var { name, initializer });
            return Ok(());
        };

        push_declaration(self, name)?;

        while self.compare_then_next(&[&TokenType::SymComma]) {
            let name = get_identifier(self)?;
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
        self.expect_then_next(&[&TokenType::Eol], "Expected new line after declaration.")?;

        if self.declaring {
            self.declaring = false;
        }

        return Ok(Box::new(VarDec {
            variable_declarations,
        }));
    }

    fn statement(&mut self) -> Result<Box<dyn Statement>, String> {
        if self.compare_then_next(&[&TokenType::RkwStart]) {
            return self.block();
        }
        if !self.in_scope {
            return Err(self
                .source_code
                .error_string_token(self.get_current(), "Statement is out of scope."));
        }
        if self.compare_then_next(&[&TokenType::RkwIf]) {
            return self.if_stmt();
        } else if self.compare_then_next(&[&TokenType::RkwOutput]) {
            return self.output();
        } else if self.compare_then_next(&[&TokenType::RkwInput]) {
            return self.input();
        } else if self.compare_then_next(&[&TokenType::RkwWhile]) {
            return self.while_stmt();
        }

        return self.expression_statement();
    }

    fn expression_statement(&mut self) -> Result<Box<dyn Statement>, String> {
        if !self.in_scope && !self.declaring {
            return Err(self.source_code.error_string_token(
                self.get_current(),
                "Out of scope expression is only allowed in variable declaration.",
            ));
        }
        let expression = self.expression()?;
        self.expect_then_next(&[&TokenType::Eol], "Expected new line after expression.")?;

        return Ok(Box::new(statement::expression::Expression {
            statement: expression,
        }));
    }

    fn expression(&mut self) -> Result<Box<dyn Expression>, String> {
        return self.assignment();
    }

    fn assignment(&mut self) -> Result<Box<dyn Expression>, String> {
        let expression = self.concatenation()?;
        return if self.compare_then_next(&[&TokenType::SymAssignment]) {
            let equals = self.get_previous().unwrap().clone();
            let value = self.assignment()?;
            if let Some(expression) = (*expression).as_any().downcast_ref::<Variable>() {
                let name = expression.name.to_owned();
                let data_type = self.variable_type.get(&name.lexeme).unwrap().clone();
                if let Some(value) = (*value).as_any().downcast_ref::<Literal>() {
                    if DataType::box_any_to_data_type(&value.value).unwrap() != data_type {
                        return Err(self.source_code.error_string_token(
                            &name,
                            &format!("Expected {:?} type.", data_type),
                        ));
                    }
                }
                return Ok(Box::new(Assign {
                    name,
                    value,
                    data_type,
                }));
            }
            Err(self
                .source_code
                .error_string_token(&equals, "Invalid assignment target."))
        } else if self.compare_then_next(&[
            &TokenType::LitBool,
            &TokenType::LitChar,
            &TokenType::LitFloat,
            &TokenType::LitInt,
            &TokenType::LitStr,
            &TokenType::Identifier,
        ]) {
            Err(self
                .source_code
                .error_string_token(self.get_previous().unwrap(), "Missing expression operator."))
        } else {
            Ok(expression)
        };
    }

    fn concatenation(&mut self) -> Result<Box<dyn Expression>, String> {
        let mut expression = self.logical_or()?;
        while self.compare_then_next(&[&TokenType::SymAmpersand]) {
            let operator = self.get_previous().unwrap().clone();
            let right = self.logical_or()?;
            expression = Box::new(Binary {
                operator,
                right,
                left: expression,
            });
        }

        return Ok(expression);
    }

    fn logical_or(&mut self) -> Result<Box<dyn Expression>, String> {
        let mut expression = self.logical_and()?;
        while self.compare_then_next(&[&TokenType::RkwOr]) {
            let operator = self.get_previous().unwrap().clone();
            let right = self.logical_and()?;
            self.expect_logical_expressions(&*right)?;
            expression = Box::new(Logical {
                operator,
                right,
                left: expression,
            });
        }

        return Ok(expression);
    }

    fn logical_and(&mut self) -> Result<Box<dyn Expression>, String> {
        let mut expression = self.equality()?;
        while self.compare_then_next(&[&TokenType::RkwAnd]) {
            let operator = self.get_previous().unwrap().clone();
            let right = self.equality()?;
            self.expect_logical_expressions(&*right)?;
            expression = Box::new(Logical {
                operator,
                right,
                left: expression,
            });
        }

        return Ok(expression);
    }

    fn equality(&mut self) -> Result<Box<dyn Expression>, String> {
        let mut expression = self.comparison()?;
        while self.compare_then_next(&[&TokenType::SymEqual, &TokenType::SymNotEqual]) {
            let operator = self.get_previous().unwrap().clone();
            let right = self.comparison()?;
            expression = Box::new(Binary {
                operator,
                right,
                left: expression,
            });
        }

        return Ok(expression);
    }

    fn comparison(&mut self) -> Result<Box<dyn Expression>, String> {
        let mut expression = self.term()?;
        while self.compare_then_next(&[
            &TokenType::SymLesser,
            &TokenType::SymLesserEqual,
            &TokenType::SymGreater,
            &TokenType::SymGreaterEqual,
        ]) {
            let operator = self.get_previous().unwrap().clone();
            let right = self.term()?;
            expression = Box::new(Binary {
                operator,
                right,
                left: expression,
            });
        }

        return Ok(expression);
    }

    fn term(&mut self) -> Result<Box<dyn Expression>, String> {
        let mut expression = self.factor()?;
        while self.compare_then_next(&[&TokenType::SymPlus, &TokenType::SymMinus]) {
            let operator = self.get_previous().unwrap().clone();
            let right = self.factor()?;
            expression = Box::new(Binary {
                operator,
                right,
                left: expression,
            });
        }

        return Ok(expression);
    }

    fn factor(&mut self) -> Result<Box<dyn Expression>, String> {
        let mut expression = self.unary()?;
        while self.compare_then_next(&[
            &TokenType::SymStar,
            &TokenType::SymForwardSlash,
            &TokenType::SymPercent,
        ]) {
            let operator = self.get_previous().unwrap().clone();
            let right = self.unary()?;
            expression = Box::new(Binary {
                operator,
                right,
                left: expression,
            });
        }

        return Ok(expression);
    }

    fn unary(&mut self) -> Result<Box<dyn Expression>, String> {
        if self.compare_then_next(&[
            &TokenType::SymPlus,
            &TokenType::SymMinus,
            &TokenType::RkwNot,
        ]) {
            let operator = self.get_previous().unwrap().clone();
            let right = self.unary()?;
            if operator.token_type == TokenType::RkwNot {
                self.expect_logical_expressions(&*right)?;
            }
            return Ok(Box::new(Unary { operator, right }));
        }

        return self.primary();
    }

    fn primary(&mut self) -> Result<Box<dyn Expression>, String> {
        self.next();
        let previous_token = self.get_previous().unwrap();
        return match previous_token.token_type {
            TokenType::LitBool
            | TokenType::LitChar
            | TokenType::LitFloat
            | TokenType::LitInt
            | TokenType::LitStr => {
                let value =
                    DataType::str_to_data_type(&previous_token.lexeme, &previous_token.token_type);
                if let Some(value) = value {
                    Ok(Box::new(Literal { value }))
                } else {
                    Err(self
                        .source_code
                        .error_string_token(previous_token, "Expected a literal value."))
                }
            }
            TokenType::Identifier => {
                if !self.var_declarations
                    && !self.variable_type.contains_key(&previous_token.lexeme)
                {
                    Err(self.source_code.error_string_token(
                        previous_token,
                        &format!("Undefined variable {}.", previous_token.lexeme),
                    ))
                } else {
                    Ok(Box::new(Variable {
                        name: previous_token.clone(),
                    }))
                }
            }
            TokenType::SymLeftParenthesis => {
                let expression = self.expression()?;
                self.expect_then_next(
                    &[&TokenType::SymRightParenthesis],
                    "Expected ')' after expression.",
                )?;
                Ok(Box::new(Grouping { expression }))
            }
            _ => {
                let error_string = self
                    .source_code
                    .error_string_token(self.get_current(), "Expected expression.");
                Err(error_string)
            }
        };
    }

    fn if_stmt(&mut self) -> Result<Box<dyn Statement>, String> {
        let if_token = self.get_previous().unwrap().clone();
        self.expect_then_next(
            &[&TokenType::SymLeftParenthesis],
            "Expected '(' after 'if'.",
        )?;
        let condition = self.expression()?;
        self.expect_token_and_eol_next(
            &TokenType::SymRightParenthesis,
            "Expected ')' after condition.",
        )?;
        self.expect_token_and_eol(&TokenType::RkwStart, "Expected 'START' before code block.")?;
        self.in_control_structure = true;
        let then_branch = self.statement()?;
        let else_branch = if self.compare_then_next(&[&TokenType::RkwElse]) {
            self.expect_then_next(&[&TokenType::Eol], "Expected new line after 'ELSE'.")?;
            self.expect_token_and_eol(&TokenType::RkwStart, "Expected 'START' before code block.")?;
            self.in_control_structure = true;
            Some(self.statement()?)
        } else {
            None
        };

        return Ok(Box::new(If {
            token: if_token,
            condition,
            then_branch,
            else_branch,
        }));
    }

    fn output(&mut self) -> Result<Box<dyn Statement>, String> {
        self.expect_then_next(&[&TokenType::SymColon], "Expected ':' after 'OUTPUT'.")?;
        let expression = self.expression()?;
        self.expect_then_next(&[&TokenType::Eol], "Expected new line after expression.")?;

        return Ok(Box::new(Print { expression }));
    }

    fn input(&mut self) -> Result<Box<dyn Statement>, String> {
        self.expect_then_next(&[&TokenType::SymColon], "Expected ':' after 'INPUT'.")?;
        let name = self
            .expect_then_next(
                &[&TokenType::Identifier],
                "Expected a variable name to receive the input.",
            )?
            .clone();
        self.expect_then_next(
            &[&TokenType::Eol],
            "Expected only one variable to receive the input.",
        )?;

        let variable = Variable { name };

        return Ok(Box::new(Input { variable }));
    }

    fn while_stmt(&mut self) -> Result<Box<dyn Statement>, String> {
        self.expect_then_next(
            &[&TokenType::SymLeftParenthesis],
            "Expected '(' after 'WHILE.",
        )?;
        let condition = self.expression()?;
        self.expect_token_and_eol_next(
            &TokenType::SymRightParenthesis,
            "Expected ')' after condition.",
        )?;
        self.expect_token_and_eol(&TokenType::RkwStart, "Expected 'START' before code block.")?;
        self.in_control_structure = true;
        let body = self.statement()?;

        return Ok(Box::new(While { condition, body }));
    }

    fn block(&mut self) -> Result<Box<dyn Statement>, String> {
        if self.in_scope && !self.in_control_structure {
            return Err(self
                .source_code
                .error_string_token(self.get_previous().unwrap(), "Nested scope in invalid."));
        }
        if !self.in_scope && self.scope_counter > 0 {
            return Err(self
                .source_code
                .error_string_token(self.get_previous().unwrap(), "Multiple scope in invalid."));
        }
        let mut is_scope = false;
        if self.var_declarations && !self.in_scope {
            is_scope = true;
            self.in_scope = true;
            self.scope_counter += 1;
            self.var_declarations = false;
        }

        let mut statements = vec![];
        self.expect_then_next(&[&TokenType::Eol], "Missing new line after 'START'.")?;
        self.in_control_structure = false;
        while !self.compare_current(&TokenType::RkwStop) && !self.is_at_end() {
            statements.push(self.declaration()?);
        }
        self.expect_then_next(&[&TokenType::RkwStop], "Expected 'STOP' after code block.")?;
        if !self.is_at_end() {
            self.expect_then_next(&[&TokenType::Eol], "Missing new line after 'STOP'.")?;
            if is_scope {
                self.in_scope = false;
            }
        }

        return Ok(Box::new(Block { statements }));
    }
}
