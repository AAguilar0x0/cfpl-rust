use crate::{data_type::DataType, expression::Expression};

use super::Statement;

pub struct While {
    condition: Box<dyn Expression>,
    body: Box<dyn Statement>,
}

impl Statement for While {
    fn visit<'a>(&self, environment: &mut crate::environment::Environment) -> Result<(), &'a str> {
        loop {
            if let Some(value) = DataType::any_to_bool(&self.condition.visit(environment)?) {
                if !value {
                    break;
                }
                self.body.visit(environment)?;
            } else {
                return Err("Operand must be a boolean.");
            }
        }
        return Ok(());
    }
}
