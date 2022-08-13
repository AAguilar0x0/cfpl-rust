use std::{any::Any, fmt::Display};

use crate::{data_type::DataType, environment::Environment, token::Token};

use super::Expression;

pub struct Variable {
    pub name: Token,
}

impl Expression for Variable {
    fn visit(&self, environment: &mut Environment) -> Result<Box<dyn std::any::Any>, String> {
        let value = environment.get(&self.name.lexeme)?;
        return match DataType::clone_ref_any(value) {
            Some(value) => Ok(value),
            None => Err("Cannot clone variable.".to_owned()),
        };
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Display for Variable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Variable({:?})", self.name)
    }
}
