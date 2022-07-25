use crate::{expression::Expression, statement::Statement, token::Token};

struct If {
    token: Token,
    condition: Box<dyn Expression>,
    then_branch: Box<dyn Statement>,
    else_branch: Box<dyn Statement>,
}

impl Statement for If {
    fn visit(&self) -> Result<(), &str> {
        let any_value = self.condition.visit();
        if let Some(condition) = any_value.downcast_ref::<bool>() {
            match condition {
                true => self.then_branch.visit(),
                false => self.else_branch.visit(),
            }?;
        } else {
            return Err("Invalid data type.");
        }
        return Ok(());
    }
}
