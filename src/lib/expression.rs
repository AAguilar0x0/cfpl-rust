use std::any::Any;

use crate::environment::Environment;

pub mod assign;
pub mod binary;
pub mod grouping;
pub mod literal;
pub mod logical;
pub mod unary;
pub mod variable;

pub trait Expression {
    fn visit<'a>(&self, environment: &mut Environment) -> Result<Box<dyn Any>, &'a str>;
}
