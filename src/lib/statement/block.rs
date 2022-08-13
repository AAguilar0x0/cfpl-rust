use std::{any::Any, fmt::Display};

use crate::environment::Environment;

use super::{display_statement, Statement};

pub struct Block {
    pub statements: Vec<Box<dyn Statement>>,
}

impl Statement for Block {
    fn visit(&self, environment: &mut Environment) -> Result<(), String> {
        for statement in &self.statements {
            statement.visit(environment)?;
        }
        return Ok(());
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = "Block(\n".to_owned();
        for statement in &self.statements {
            result.push('\t');
            result.push_str(&display_statement(statement));
            result.push('\n');
        }
        result.push(')');
        write!(f, "{}", result)
    }
}
