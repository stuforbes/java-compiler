use crate::ast::expression::Expression;
use std::fmt::{Debug, Formatter};

pub trait Statement: Debug {}

// pub enum Statement {
//     Expression { expression: Expression },
// }

pub struct ExpressionStatement<'ast> {
    expression: Box<dyn Expression + 'ast>,
}
impl<'ast> ExpressionStatement<'ast> {
    pub fn new(expression: Box<dyn Expression + 'ast>) -> Self {
        Self { expression }
    }
}

impl<'ast> Debug for ExpressionStatement<'ast> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ExpressionStatement")
            .field("expression", &self.expression)
            .finish()
    }
}

impl<'ast> Statement for ExpressionStatement<'ast> {}
