use std::{any::Any, fmt::Display};

use crate::{data_type::DataType, environment::Environment, token::Token};

use super::Expression;

pub struct Assign {
    pub name: Token,
    pub value: Box<dyn Expression>,
    pub data_type: DataType,
}

impl Expression for Assign {
    fn visit(&self, environment: &mut Environment) -> Result<Box<dyn std::any::Any>, String> {
        let mut value = self.value.visit(environment)?;
        let data_type = DataType::box_any_to_data_type(&value);
        if data_type.is_none() {
            return Err("Expected expression value as '{token_type}'.".to_owned());
        }
        if self.data_type == DataType::FLOAT && data_type.unwrap() == DataType::INT {
            value = Box::new(f64::from(*value.downcast_ref::<i32>().unwrap()));
        }
        let return_value = DataType::clone_ref_any(&value);
        environment.assign(self.name.lexeme.clone(), value)?;
        return Ok(return_value.unwrap());
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Display for Assign {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Assign({:?}, {:?}, Expression)",
            self.name, self.data_type
        )
    }
}
