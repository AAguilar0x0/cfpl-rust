use std::{any::Any, io::stdin, str::FromStr};

use crate::{data_type::DataType, environment::Environment, expression};

use super::Statement;

pub struct Input {
    variable: expression::variable::Variable,
}

impl Statement for Input {
    fn visit<'a>(&self, environment: &mut Environment) -> Result<(), &'a str> {
        let mut buf = String::new();
        if stdin().read_line(&mut buf).is_err() {
            return Err("Something went wrong while reading from stdin.");
        }
        let data_type = environment.data_type(&self.variable.name.lexeme)?;
        let value = match data_type {
            DataType::INT => handle_parse::<i32>(&buf),
            DataType::FLOAT => handle_parse::<f64>(&buf),
            DataType::CHAR => handle_parse::<char>(&buf),
            DataType::BOOL => handle_parse::<bool>(&buf),
        }?;
        environment.assign(self.variable.name.lexeme.clone(), value)?;
        return Ok(());
    }
}

fn handle_parse<T: FromStr + 'static>(buf: &str) -> Result<Box<dyn Any>, &'static str> {
    match buf.trim().parse::<T>() {
        Ok(value) => Ok(Box::new(value)),
        Err(_) => Err("Something went wrong while parsing from stdin."),
    }
}
