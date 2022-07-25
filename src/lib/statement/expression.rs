use super::Statement;

struct Expression {
    statement: dyn Statement,
}

impl Statement for Expression {
    fn visit(&self) -> Result<(), &str> {
        self.statement.visit()?;
        return Ok(());
    }
}
