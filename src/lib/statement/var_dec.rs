use super::{var::Var, Statement};

pub struct VarDec {
    pub variable_declarations: Vec<Var>,
}

impl Statement for VarDec {
    fn visit<'a>(&self, environment: &mut crate::environment::Environment) -> Result<(), &'a str> {
        for variable_declaration in &self.variable_declarations {
            variable_declaration.visit(environment)?;
        }
        return Ok(());
    }
}
