use crate::environment::Environment;

pub mod block;
pub mod expression;
pub mod if_stmt;
pub mod input;
pub mod print;
pub mod var;
pub mod while_stmt;

pub trait Statement {
    fn visit<'a>(&self, environment: &mut Environment) -> Result<(), &'a str>;
}
