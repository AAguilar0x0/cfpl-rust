use std::collections::HashMap;

use crate::{environment::Environment, statement::Statement};

pub fn interpreter(statements: Vec<Box<dyn Statement>>) -> Result<(), String> {
    let mut environment = Environment {
        variables: HashMap::new(),
    };
    for statement in statements {
        statement.visit(&mut environment)?;
    }
    return Ok(());
}
