use std::{any::Any, fmt::Display};

use crate::{
    environment::Environment,
    expression::{display_expression, Expression},
    statement::Statement,
    token::Token,
};

pub struct If {
    pub token: Token,
    pub condition: Box<dyn Expression>,
    pub then_branch: Box<dyn Statement>,
    pub else_branch: Option<Box<dyn Statement>>,
}

impl Statement for If {
    fn visit(&self, environment: &mut Environment) -> Result<(), String> {
        let any_value = self.condition.visit(environment)?;
        if let Some(condition) = any_value.downcast_ref::<bool>() {
            if *condition {
                self.then_branch.visit(environment)?;
            } else if let Some(else_branch) = &self.else_branch {
                else_branch.visit(environment)?;
            }
        } else {
            return Err("Invalid data type.".to_owned());
        }
        return Ok(());
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Display for If {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "If({})", display_expression(&self.condition))
    }
}
