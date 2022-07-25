use crate::{
    expression::{stringify_primitives, Expression},
    statement::Statement,
};

pub struct Print {
    pub expression: Box<dyn Expression>,
}

impl Statement for Print {
    fn visit(&self) -> Result<(), &str> {
        let output = self.expression.visit();
        let value = stringify_primitives(output)?;
        print!("{value}");
        return Ok(());
    }
}
