use std::any::Any;

use crate::{data_type::DataType, environment, token::Token};

use super::Expression;

pub struct Variable {
    pub name: Token,
}

impl Expression for Variable {
    fn visit<'a>(
        &self,
        environment: &mut environment::Environment,
    ) -> Result<Box<dyn std::any::Any>, &'a str> {
        let value = environment.get(&self.name.lexeme)?;
        return match DataType::clone_ref_any(value) {
            Some(value) => Ok(value),
            None => Err("asdf"),
        };
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
