use std::any::Any;

use crate::{data_type::DataType, token::Token, token_type::TokenType};

use super::Expression;

pub struct Logical {
    pub operator: Token,
    pub left: Box<dyn Expression>,
    pub right: Box<dyn Expression>,
}

impl Expression for Logical {
    fn visit<'a>(
        &self,
        environment: &mut crate::environment::Environment,
    ) -> Result<Box<dyn std::any::Any>, &'a str> {
        if self.operator.token_type != TokenType::RkwOr
            && self.operator.token_type != TokenType::RkwAnd
        {
            return Err("Operator must be logical.");
        }
        let left_value = self.left.visit(environment)?;
        let is_left = DataType::any_to_bool(&left_value);
        if is_left.is_none() {
            return Err("Operand must be a boolean.");
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
