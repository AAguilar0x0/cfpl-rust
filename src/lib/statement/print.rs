use crate::{
    data_type::DataType, environment::Environment, expression::Expression, statement::Statement,
};

pub struct Print {
    pub expression: Box<dyn Expression>,
}

impl Statement for Print {
    fn visit<'a>(&self, environment: &mut Environment) -> Result<(), &'a str> {
        let output = self.expression.visit(environment)?;
        let value = DataType::stringify_primitives(&output)?;
        print!("{value}");
        return Ok(());
    }
}
