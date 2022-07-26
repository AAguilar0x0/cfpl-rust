use crate::environment::Environment;

use super::Statement;

struct Block {
    statements: Vec<Box<dyn Statement>>,
}

impl Statement for Block {
    fn visit<'a>(&self, environment: &mut Environment) -> Result<(), &'a str> {
        for statement in &self.statements {
            statement.visit(environment)?;
        }
        return Ok(());
    }
}
