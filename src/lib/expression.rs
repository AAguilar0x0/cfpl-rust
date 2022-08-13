use std::any::Any;

use crate::environment::Environment;

use self::{
    assign::Assign, binary::Binary, grouping::Grouping, literal::Literal, logical::Logical,
    unary::Unary, variable::Variable,
};

pub mod assign;
pub mod binary;
pub mod grouping;
pub mod literal;
pub mod logical;
pub mod unary;
pub mod variable;

pub trait Expression {
    fn visit(&self, environment: &mut Environment) -> Result<Box<dyn Any>, String>;
    fn as_any(&self) -> &dyn Any;
}

pub fn display_expression(expression: &Box<dyn Expression>) -> String {
    if let Some(expression) = (*expression).as_any().downcast_ref::<Assign>() {
        return expression.to_string();
    } else if let Some(expression) = (*expression).as_any().downcast_ref::<Binary>() {
        return expression.to_string();
    } else if let Some(expression) = (*expression).as_any().downcast_ref::<Grouping>() {
        return expression.to_string();
    } else if let Some(expression) = (*expression).as_any().downcast_ref::<Literal>() {
        return expression.to_string();
    } else if let Some(expression) = (*expression).as_any().downcast_ref::<Logical>() {
        return expression.to_string();
    } else if let Some(expression) = (*expression).as_any().downcast_ref::<Unary>() {
        return expression.to_string();
    } else if let Some(expression) = (*expression).as_any().downcast_ref::<Variable>() {
        return expression.to_string();
    }
    return "".to_owned();
}
