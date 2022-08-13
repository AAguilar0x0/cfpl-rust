use std::{any::Any, fmt::Display};

use crate::{data_type::DataType, environment::Environment, token::Token, token_type::TokenType};

use super::Expression;

pub struct Logical {
    pub operator: Token,
    pub left: Box<dyn Expression>,
    pub right: Box<dyn Expression>,
}

impl Expression for Logical {
    fn visit(&self, environment: &mut Environment) -> Result<Box<dyn std::any::Any>, String> {
        if self.operator.token_type != TokenType::RkwOr
            && self.operator.token_type != TokenType::RkwAnd
        {
            return Err("Operator must be logical.".to_owned());
        }
        let left_value = self.left.visit(environment)?;
        let is_left = DataType::any_to_bool(&left_value);
        if is_left.is_none() {
            return Err("Operand must be a boolean.".to_owned());
        }
        let is_left = is_left.unwrap();
        if (self.operator.token_type == TokenType::RkwOr && *is_left)
            || (self.operator.token_type == TokenType::RkwAnd && !*is_left)
        {
            return Ok(left_value);
        } else {
            return self.right.visit(environment);
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Display for Logical {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Logical(Expression, {:?}, Expression)", self.operator)
    }
}
