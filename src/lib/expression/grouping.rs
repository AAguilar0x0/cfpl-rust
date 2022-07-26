use super::Expression;

pub struct Grouping {
    expression: Box<dyn Expression>,
}

impl Expression for Grouping {
    fn visit<'a>(
        &self,
        environment: &mut crate::environment::Environment,
    ) -> Result<Box<dyn std::any::Any>, &'a str> {
        return self.expression.visit(environment);
    }
}
