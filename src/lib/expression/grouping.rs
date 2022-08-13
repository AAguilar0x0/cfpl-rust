use std::{any::Any, fmt::Display};

use crate::environment::Environment;

use super::Expression;

pub struct Grouping {
    pub expression: Box<dyn Expression>,
}

impl Expression for Grouping {
    fn visit(&self, environment: &mut Environment) -> Result<Box<dyn std::any::Any>, String> {
        return self.expression.visit(environment);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Display for Grouping {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Grouping(Expression)")
    }
}
