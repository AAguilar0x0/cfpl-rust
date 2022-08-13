use std::{any::Any, fmt::Display};

use crate::{
    environment::Environment,
    expression::{display_expression, Expression},
    token::Token,
};

use super::Statement;

pub struct Var {
    pub name: Token,
    pub initializer: Box<dyn Expression>,
}

impl Statement for Var {
    fn visit(&self, environment: &mut Environment) -> Result<(), String> {
        let value = self.initializer.visit(environment)?;
        environment.define(self.name.lexeme.clone(), value)?;
        return Ok(());
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Display for Var {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Var({}, {})",
            self.name,
            display_expression(&self.initializer)
        )
    }
}
