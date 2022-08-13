use std::{any::Any, fmt::Display};

use crate::{data_type::DataType, environment::Environment, token::Token, token_type::TokenType};

use super::Expression;

pub struct Unary {
    pub operator: Token,
    pub right: Box<dyn Expression>,
}

impl Expression for Unary {
    fn visit(&self, environment: &mut Environment) -> Result<Box<dyn std::any::Any>, String> {
        match self.operator.token_type {
            TokenType::RkwNot => {
                let expression = self.right.visit(environment)?;
                let is_bool = DataType::any_to_bool(&expression);
                if is_bool.is_none() {
                    return Err("Operand must be a boolean.".to_owned());
                }
                return Ok(Box::new(!is_bool.unwrap()));
            }
            TokenType::SymPlus => {
                let expression = self.right.visit(environment)?;
                let data_type = DataType::box_any_to_data_type(&expression);
                if data_type.is_none() {
                    return Err("Invalid operand data type.".to_owned());
                }
                return match data_type.unwrap() {
                    DataType::INT | DataType::FLOAT => Ok(expression),
                    _ => Err("Operand must be a number.".to_owned()),
                };
            }
            TokenType::SymMinus => {
                let expression = self.right.visit(environment)?;
                let data_type = DataType::box_any_to_data_type(&expression);
                if data_type.is_none() {
                    return Err("Invalid operand data type.".to_owned());
                }
                return match data_type.unwrap() {
                    DataType::INT => Ok(Box::new(-expression.downcast_ref::<i32>().unwrap())),
                    DataType::FLOAT => Ok(Box::new(-expression.downcast_ref::<f64>().unwrap())),
                    _ => Err("Operand must be a number.".to_owned()),
                };
            }
            _ => return Err("Invalid unary operator.".to_owned()),
        };
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Display for Unary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unary({:?}, Expression)", self.operator)
    }
}
