use std::{any::Any, fmt::Display};

use crate::{
    data_type::DataType,
    environment::Environment,
    expression::{display_expression, Expression},
};

use super::Statement;

pub struct While {
    pub condition: Box<dyn Expression>,
    pub body: Box<dyn Statement>,
}

impl Statement for While {
    fn visit(&self, environment: &mut Environment) -> Result<(), String> {
        loop {
            if let Some(value) = DataType::any_to_bool(&self.condition.visit(environment)?) {
                if !value {
                    break;
                }
                self.body.visit(environment)?;
            } else {
                return Err("Operand must be a boolean.".to_owned());
            }
        }
        return Ok(());
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Display for While {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "While({}, Statement)",
            display_expression(&self.condition)
        )
    }
}
