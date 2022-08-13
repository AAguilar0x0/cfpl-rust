use std::any::Any;

use crate::environment::Environment;

use self::{
    block::Block, expression::Expression, if_stmt::If, input::Input, print::Print, var::Var,
    var_dec::VarDec, while_stmt::While,
};

pub mod block;
pub mod expression;
pub mod if_stmt;
pub mod input;
pub mod print;
pub mod var;
pub mod var_dec;
pub mod while_stmt;

pub trait Statement {
    fn visit(&self, environment: &mut Environment) -> Result<(), String>;
    fn as_any(&self) -> &dyn Any;
}

pub fn display_statement(statement: &Box<dyn Statement>) -> String {
    if let Some(statement) = (*statement).as_any().downcast_ref::<Block>() {
        return statement.to_string();
    } else if let Some(statement) = (*statement).as_any().downcast_ref::<Expression>() {
        return statement.to_string();
    } else if let Some(statement) = (*statement).as_any().downcast_ref::<If>() {
        return statement.to_string();
    } else if let Some(statement) = (*statement).as_any().downcast_ref::<Input>() {
        return statement.to_string();
    } else if let Some(statement) = (*statement).as_any().downcast_ref::<Print>() {
        return statement.to_string();
    } else if let Some(statement) = (*statement).as_any().downcast_ref::<VarDec>() {
        return statement.to_string();
    } else if let Some(statement) = (*statement).as_any().downcast_ref::<Var>() {
        return statement.to_string();
    } else if let Some(statement) = (*statement).as_any().downcast_ref::<While>() {
        return statement.to_string();
    }
    return "".to_owned();
}
