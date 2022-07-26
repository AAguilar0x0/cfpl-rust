use crate::environment::Environment;

pub mod block;
pub mod expression;
pub mod ifs;
pub mod input;
pub mod print;

pub trait Statement {
    fn visit<'a>(&self, environment: &mut Environment) -> Result<(), &'a str>;
}
