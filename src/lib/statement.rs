pub mod block;
pub mod expression;
pub mod ifs;
pub mod print;

pub trait Statement {
    fn visit(&self) -> Result<(), &str>;
}
