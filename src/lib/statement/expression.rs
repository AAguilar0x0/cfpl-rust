use crate::{environment::Environment, expression};

use super::Statement;

pub struct Expression {
    pub statement: Box<dyn expression::Expression>,
}

impl Statement for Expression {
    fn visit<'a>(&self, environment: &mut Environment) -> Result<(), &'a str> {
        self.statement.visit(environment)?;
        return Ok(());
    }
}
