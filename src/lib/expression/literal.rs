use std::{any::Any, fmt::Display};

use crate::data_type::DataType;

use super::Expression;

pub struct Literal {
    pub value: Box<dyn Any>,
}

impl Expression for Literal {
    fn visit(&self, _: &mut crate::environment::Environment) -> Result<Box<dyn Any>, String> {
        return match DataType::clone_ref_any(&(self.value)) {
            Some(value) => Ok(value),
            None => Err("Cannot clone literal.".to_owned()),
        };
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Literal({:?})", self.value)
    }
}
