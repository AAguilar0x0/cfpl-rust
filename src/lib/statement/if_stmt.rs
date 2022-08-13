use crate::{environment::Environment, expression::Expression, statement::Statement, token::Token};

pub struct If {
    pub token: Token,
    pub condition: Box<dyn Expression>,
    pub then_branch: Box<dyn Statement>,
    pub else_branch: Option<Box<dyn Statement>>,
}

impl Statement for If {
    fn visit<'a>(&self, environment: &mut Environment) -> Result<(), &'a str> {
        let any_value = self.condition.visit(environment)?;
        if let Some(condition) = any_value.downcast_ref::<bool>() {
            if *condition {
                self.then_branch.visit(environment)?;
            } else if let Some(else_branch) = &self.else_branch {
                else_branch.visit(environment)?;
            }
        } else {
            return Err("Invalid data type.");
        }
        return Ok(());
    }
}
