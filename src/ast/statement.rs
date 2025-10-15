use crate::ast::expression::Expression;
use std::fmt::Debug;

#[derive(Debug)]
pub enum Statement<'ast> {
    Expression { expression: Expression<'ast> },
}
impl <'ast> Statement<'ast> {
    pub fn new_expression_statement(expression: Expression) -> Statement {
        Statement::Expression { expression }
    }
}