use std::fmt::Debug;

#[derive(PartialEq, Debug)]
pub enum Expression<'ast> {
    Call {
        object_path: Vec<&'ast str>,
        method_name: &'ast str,
        arguments: Vec<Expression<'ast>>,
    },
    StringLiteral {
        value: &'ast str,
    },
}
impl<'ast> Expression<'ast> {
    pub fn new_call(
        object_path: Vec<&'ast str>,
        method_name: &'ast str,
        arguments: Vec<Expression<'ast>>,
    ) -> Self {
        Self::Call {
            object_path,
            method_name,
            arguments
        }
    }

    pub fn new_string_literal(value: &'ast str) -> Self {
        Self::StringLiteral { value }
    }
}