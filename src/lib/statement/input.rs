use std::{any::Any, fmt::Display, io::stdin, str::FromStr};

use crate::{data_type::DataType, environment::Environment, expression};

use super::Statement;

pub struct Input {
    pub variable: expression::variable::Variable,
}

impl Statement for Input {
    fn visit(&self, environment: &mut Environment) -> Result<(), String> {
        let mut buf = String::new();
        if stdin().read_line(&mut buf).is_err() {
            return Err("Something went wrong while reading from stdin.".to_owned());
        }
        let data_type = environment.data_type(&self.variable.name.lexeme)?;
        let value = match data_type {
            DataType::INT => handle_parse::<i32>(&buf),
            DataType::FLOAT => handle_parse::<f64>(&buf),
            DataType::CHAR => handle_parse::<char>(&buf),
            DataType::BOOL => handle_parse::<bool>(&buf),
            DataType::STR => return Err("Invalid STR data type.".to_owned()),
        }?;
        environment.assign(self.variable.name.lexeme.clone(), value)?;
        return Ok(());
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

fn handle_parse<T: FromStr + 'static>(buf: &str) -> Result<Box<dyn Any>, String> {
    match buf.trim().parse::<T>() {
        Ok(value) => Ok(Box::new(value)),
        Err(_) => Err("Something went wrong while parsing from stdin.".to_owned()),
    }
}

impl Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Input({})", self.variable.to_string())
    }
}
