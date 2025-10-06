use crate::ast::class_builder::Scope;

#[derive(Debug)]
pub struct Class<'a> {
    name: &'a str,
    scope: Scope,
    is_static: bool,
    is_final: bool,
    methods: Vec<Method<'a>>,
}

impl <'a> Class<'a> {
    pub fn new(name: &'a str,
               scope: Scope,
               is_static: bool,
               is_final: bool,
               methods: Vec<Method<'a>>
    ) -> Self {
        Self {
            name,
            scope,
            is_static,
            is_final,
            methods
        }
    }
}

#[derive(Debug)]
pub struct Method<'a> {
    name: &'a str,
    return_type: &'a str,
    parameters: Vec<Parameter<'a>>,
    statements: Vec<&'a str>,
}

impl <'a> Method<'a> {
    pub fn new(name: &'a str,
               return_type: &'a str,
               parameters: Vec<Parameter<'a>>,
               statements: Vec<&'a str>
    ) -> Self {
        Self {
            name,
            return_type,
            parameters,
            statements
        }
    }
}

#[derive(Debug)]
pub struct Parameter<'a> {
    param_name: &'a str,
    param_type: &'a str,
}

impl <'a> Parameter<'a> {
    pub fn new(param_name: &'a str, param_type: &'a str) -> Self {
        Self {
            param_name,
            param_type
        }
    }
}
