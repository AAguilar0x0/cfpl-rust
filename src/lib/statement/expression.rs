use std::{any::Any, fmt::Display};

use crate::{
    environment::Environment,
    expression::{self, display_expression},
};

use super::Statement;

pub struct Expression {
    pub statement: Box<dyn expression::Expression>,
}

impl Statement for Expression {
    fn visit(&self, environment: &mut Environment) -> Result<(), String> {
        self.statement.visit(environment)?;
        return Ok(());
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Expression({})", display_expression(&self.statement))
    }
}
