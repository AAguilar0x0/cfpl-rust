use std::any::Any;

use crate::{data_type::DataType, token::Token, token_type::TokenType};

use super::Expression;

pub struct Assign {
    pub name: Token,
    pub value: Box<dyn Expression>,
    pub token_type: TokenType,
}

impl Expression for Assign {
    fn visit<'a>(
        &self,
        environment: &mut crate::environment::Environment,
    ) -> Result<Box<dyn std::any::Any>, &'a str> {
        let mut value = self.value.visit(environment)?;
        let data_type = DataType::box_any_to_data_type(&value);
        if data_type.is_none() {
            return Err("Expected expression value as '{token_type}'");
        }
        if self.token_type == TokenType::RkwFloat && data_type.unwrap() == DataType::INT {
            value = Box::new(f64::from(*value.downcast_ref::<i32>().unwrap()));
        }
        let return_value = DataType::clone_ref_any(&value);
        environment.assign(self.name.lexeme.clone(), value)?;
        return Ok(return_value.unwrap());
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
