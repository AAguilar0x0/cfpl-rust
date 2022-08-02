use crate::{expression::Expression, token::Token};

use super::Statement;

pub struct VarDec {
    name: Token,
    initializer: Box<dyn Expression>,
}

pub struct Var {
    variable_declarations: Vec<Box<VarDec>>,
}

impl Statement for Var {
    fn visit<'a>(&self, environment: &mut crate::environment::Environment) -> Result<(), &'a str> {
        let mut value;
        for variable_declaration in &self.variable_declarations {
            value = variable_declaration.initializer.visit(environment)?;
            environment.define(variable_declaration.name.lexeme.clone(), value)?;
        }
        return Ok(());
    }
}
