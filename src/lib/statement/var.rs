use crate::{expression::Expression, token::Token};

use super::Statement;

pub struct Var {
    pub name: Token,
    pub initializer: Box<dyn Expression>,
}

impl Statement for Var {
    fn visit<'a>(&self, environment: &mut crate::environment::Environment) -> Result<(), &'a str> {
        let value = self.initializer.visit(environment)?;
        environment.define(self.name.lexeme.clone(), value)?;
        return Ok(());
    }
}
