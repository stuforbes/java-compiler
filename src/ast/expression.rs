use std::fmt::Debug;

#[derive(PartialEq, Debug, Clone)]
pub enum Expression<'ast> {
    Call {
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
    StaticIdentifier {
        name: &'ast str,
    },
    // TODO: Deprecated
    ChildIdentifier {
        parent: Box<Expression<'ast>>,
        name: &'ast str,
    },
    ObjectExpression {
        parent: Box<Expression<'ast>>,
        child: Box<Expression<'ast>>,
    },
    Assignment {
        name: &'ast str,
        type_def: Option<&'ast str>,
        value: Box<Expression<'ast>>,
    }
}

impl<'ast> Expression<'ast> {
    pub fn new_call(
        method_name: &'ast str,
        arguments: Vec<Expression<'ast>>,
    ) -> Expression<'ast> {
        Self::Call {
            method_name,
            arguments
        }
    }

    pub fn new_string_literal(value: &'ast str) -> Expression<'ast> {
        Self::StringLiteral { value }
    }

    pub fn new_variable(name: &'ast str, type_def: Option<&'ast str>) -> Expression<'ast> {
        Self::Variable { name, type_def }
    }

    pub fn new_static_identifier(name: &'ast str) -> Expression<'ast> {
        Self::StaticIdentifier { name }
    }

    pub fn new_child_identifier(parent: Expression<'ast>, name: &'ast str) -> Expression<'ast> {
        Self::ChildIdentifier { parent: Box::new(parent), name }
    }

    pub fn new_object_expression(parent: Expression<'ast>, child: Expression<'ast>) -> Expression<'ast> {
        Self::ObjectExpression { parent: Box::new(parent), child: Box::new(child) }
    }

    pub fn new_assignment(name: &'ast str, type_def: Option<&'ast str>, value: Expression<'ast>) -> Expression<'ast> {
        Self::Assignment { name, type_def, value: Box::new(value) }
    }
}