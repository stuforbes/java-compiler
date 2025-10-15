use std::fmt::{Debug, Formatter};

pub trait Expression : std::fmt::Debug {}

pub struct CallExpression<'ast> {
    object_path: &'ast str,
    method_name: &'ast str,
    arguments: Vec<Box<dyn Expression + 'ast>>,
}
impl <'ast> CallExpression<'ast> {
    pub fn new(object_path: &'ast str,
               method_name: &'ast str,
               arguments: Vec<Box<dyn Expression + 'ast>>
    ) -> Self {
        Self {
            object_path,
            method_name,
            arguments
        }
    }
}

impl <'ast> Debug for CallExpression<'ast> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CallExpression")
            .field("object_path", &self.object_path)
            .field("method_name", &self.method_name)
            .field("arguments", &self.arguments)
            .finish()
    }
}

impl<'ast> Expression for CallExpression<'ast> {}

pub struct StringLiteral<'ast> {
    value: &'ast str,
}
impl <'ast> StringLiteral<'ast> {
    pub fn new(value: &'ast str) -> Self {
        Self {
            value
        }
    }
}

impl <'ast> Debug for StringLiteral<'ast> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StringLiteral")
            .field("value", &self.value)
            .finish()
    }
}

impl<'ast> Expression for StringLiteral<'ast> {}
