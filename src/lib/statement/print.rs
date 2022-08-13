use std::{
    any::Any,
    fmt::Display,
    io::{self, Write},
};

use crate::{
    data_type::DataType,
    environment::Environment,
    expression::{display_expression, Expression},
    statement::Statement,
};

pub struct Print {
    pub expression: Box<dyn Expression>,
}

impl Statement for Print {
    fn visit(&self, environment: &mut Environment) -> Result<(), String> {
        let output = self.expression.visit(environment)?;
        let value = DataType::stringify_primitives(&output)?;
        print!("{value}");
        let _ = io::stdout().flush();
        return Ok(());
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Display for Print {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Print({})", display_expression(&self.expression))
    }
}
