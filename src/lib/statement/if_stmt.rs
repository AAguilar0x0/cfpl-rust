use crate::{environment::Environment, expression::Expression, statement::Statement, token::Token};

struct If {
    token: Token,
    condition: Box<dyn Expression>,
    then_branch: Box<dyn Statement>,
    else_branch: Box<dyn Statement>,
}

impl Statement for If {
    fn visit<'a>(&self, environment: &mut Environment) -> Result<(), &'a str> {
        let any_value = self.condition.visit(environment)?;
        if let Some(condition) = any_value.downcast_ref::<bool>() {
            match condition {
                true => self.then_branch.visit(environment),
                false => self.else_branch.visit(environment),
            }?;
        } else {
            return Err("Invalid data type.");
        }
        return Ok(());
    }
}
