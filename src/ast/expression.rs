use std::fmt::Debug;

#[derive(PartialEq, Debug)]
pub enum Expression<'ast> {
    Call {
        target: Box<Expression<'ast>>,
        method_name: &'ast str,
        arguments: Vec<Expression<'ast>>,
    },
    StringLiteral {
        value: &'ast str,
    },
    Variable {
        name: &'ast str,
        type_def: Option<&'ast str>,
    },
    ChildIdentifier {
        parent: Box<Expression<'ast>>,
        name: &'ast str,
    },
    Assignment {
        name: &'ast str,
        type_def: Option<&'ast str>,
        value: Box<Expression<'ast>>,
    }
}

impl<'ast> Expression<'ast> {
    pub fn new_call(
        target: Expression<'ast>,
        method_name: &'ast str,
        arguments: Vec<Expression<'ast>>,
    ) -> Self {
        Self::Call {
            target: Box::new(target),
            method_name,
            arguments
        }
    }

    pub fn new_string_literal(value: &'ast str) -> Self {
        Self::StringLiteral { value }
    }

    pub fn new_variable(name: &'ast str, type_def: Option<&'ast str>) -> Self {
        Self::Variable { name, type_def }
    }

    pub fn new_child_identifier(parent: Expression<'ast>, name: &'ast str) -> Self {
        Self::ChildIdentifier { parent: Box::new(parent), name }
    }

    pub fn new_assignment(name: &'ast str, type_def: Option<&'ast str>, value: Expression<'ast>) -> Self {
        Self::Assignment { name, type_def, value: Box::new(value) }
    }
}