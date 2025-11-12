use crate::ast::expression::Expression;
use std::fmt::Debug;

#[derive(Debug)]
pub enum Statement<'ast> {
    Expression { expression: Expression<'ast> },
    VariableAssignment { name: &'ast str, var_type: Option<&'ast str>, is_final: bool, value: Option<Expression<'ast>> }
}
impl <'ast> Statement<'ast> {
    pub fn new_expression_statement(expression: Expression<'ast>) -> Statement<'ast> {
        Statement::Expression { expression }
    }

    pub fn new_var_assignment(name: &'ast str, var_type: Option<&'ast str>, is_final: bool, value: Option<Expression<'ast>>) -> Statement<'ast> {
        Statement::VariableAssignment { name, var_type, is_final, value }
    }
}