use crate::token::Token;

use super::Expression;

pub struct Variable {
    pub name: Token,
}

impl Expression for Variable {
    fn visit(&self) -> Box<dyn std::any::Any> {
        return Box::new(true);
    }
}
