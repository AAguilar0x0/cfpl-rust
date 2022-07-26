use crate::environment::Environment;

use super::Statement;

struct Expression {
    statement: dyn Statement,
}

impl Statement for Expression {
    fn visit<'a>(&self, environment: &mut Environment) -> Result<(), &'a str> {
        self.statement.visit(environment)?;
        return Ok(());
    }
}
