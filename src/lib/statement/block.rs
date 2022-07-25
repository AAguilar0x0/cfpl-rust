use super::Statement;

struct Block {
    statements: Vec<Box<dyn Statement>>,
}

impl Statement for Block {
    fn visit(&self) -> Result<(), &str> {
        for statement in &self.statements {
            statement.visit()?;
        }
        return Ok(());
    }
}
