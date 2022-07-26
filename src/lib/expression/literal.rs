use std::any::Any;

use crate::data_type::DataType;

use super::Expression;

pub struct Literal {
    value: Box<dyn Any>,
}

impl Expression for Literal {
    fn visit<'a>(&self, _: &mut crate::environment::Environment) -> Result<Box<dyn Any>, &'a str> {
        return match DataType::clone_ref_any(&self.value) {
            Some(value) => Ok(value),
            None => Err("Cannot clone value"),
        };
    }
}
