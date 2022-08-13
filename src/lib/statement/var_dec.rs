use std::{any::Any, fmt::Display};

use crate::environment::Environment;

use super::{var::Var, Statement};

pub struct VarDec {
    pub variable_declarations: Vec<Var>,
}

impl Statement for VarDec {
    fn visit(&self, environment: &mut Environment) -> Result<(), String> {
        for variable_declaration in &self.variable_declarations {
            variable_declaration.visit(environment)?;
        }
        return Ok(());
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Display for VarDec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = "VarDec(\n".to_owned();
        for variable in &self.variable_declarations {
            result.push('\t');
            result.push_str(&variable.to_string());
            result.push('\n');
        }
        result.push(')');
        write!(f, "{}", result)
    }
}
