use crate::ast::class::{AstClass, AstMethod, AstParameter, AstScope, AstStatement};
use crate::ast::class_builder::AstScope::Default;

pub trait Build<T> {
    fn build(self) -> T;
}

pub struct ClassBuilder<'a> {
    name: Option<&'a str>,
    scope: AstScope,
    is_static: bool,
    is_final: bool,
    methods: Vec<MethodBuilder<'a>>,
}

impl<'a> ClassBuilder<'a> {
    pub fn new() -> Self {
        Self {
            name: None,
            scope: Default,
            is_static: false,
            is_final: false,
            methods: vec![],
        }
    }

    pub fn with_scope(&mut self, scope: AstScope) {
        self.scope = scope
    }

    #[allow(dead_code)]
    pub fn as_static(&mut self) {
        self.is_static = true
    }

    #[allow(dead_code)]
    pub fn as_final(&mut self) {
        self.is_final = true
    }

    pub fn with_new_method(&mut self) {
        self.methods.push(MethodBuilder::new())
    }

    pub fn named(&mut self, name: &'a str) {
        self.name = Some(name)
    }

    pub fn latest_method(&mut self) -> &mut MethodBuilder<'a> {
        match self.methods.last_mut() {
            Some(m) => m,
            None => panic!("Expected method to exist"),
        }
    }
}

impl <'a> Build<AstClass<'a>> for ClassBuilder<'a> {
    fn build(self) -> AstClass<'a> {
        let Some(name) = self.name else {
            panic!("Name was not set")
        };

        let mut ast_methods: Vec<AstMethod> = vec![];
        for method_builder in self.methods {
            ast_methods.push(method_builder.build());
        }

        AstClass::new(
            name,
            self.scope,
            self.is_static,
            self.is_final,
            ast_methods,
        )
    }
}

#[derive(Clone)]
#[allow(dead_code)]
pub struct MethodBuilder<'a> {
    name: Option<&'a str>,
    scope: Option<AstScope>,
    is_static: bool,
    is_final: bool,
    return_type: Option<&'a str>,
    parameters: Vec<ParameterBuilder<'a>>,
    statements: Vec<StatementBuilder<'a>>,
}

impl<'a> MethodBuilder<'a> {
    fn new() -> Self {
        Self {
            name: None,
            scope: None,
            is_static: false,
            is_final: false,
            return_type: None,
            parameters: vec![],
            statements: vec![],
        }
    }

    pub fn with_name(&mut self, name: &'a str) {
        self.name = Some(name)
    }

    pub fn with_scope(&mut self, scope: AstScope) {
        self.scope = Some(scope)
    }

    pub fn as_static(&mut self) {
        self.is_static = true;
    }

    pub fn as_final(&mut self) {
        self.is_final = true;
    }

    pub fn with_return_type(&mut self, return_type: &'a str) {
        self.return_type = Some(return_type)
    }

    pub fn with_new_parameter(&mut self) {
        self.parameters.push(ParameterBuilder::new());
    }

    pub fn latest_parameter(&mut self) -> &mut ParameterBuilder<'a> {
        match self.parameters.last_mut() {
            Some(m) => m,
            None => panic!("Expected parameter to exist"),
        }
    }
    
    pub fn with_new_statement(&mut self) {
        self.statements.push(StatementBuilder::new());
    }
    
    pub fn latest_statement(&mut self) -> &mut StatementBuilder<'a> {
        match self.statements.last_mut() {
            Some(s) => s,
            None => panic!("Expected statement to exist"),
        }
    }
}

impl <'a> Build<AstMethod<'a>> for MethodBuilder<'a> {
    fn build(self) -> AstMethod<'a> {
        let Some(name) = self.name else {
            panic!("Name was not set")
        };
        let Some(return_type) = self.return_type else {
            panic!("Return type was not set")
        };

        let mut ast_parameters: Vec<AstParameter> = vec![];
        for parameter in self.parameters {
            ast_parameters.push(parameter.build());
        }

        let scope = self.scope.unwrap_or(AstScope::Public);

        AstMethod::new(
            name,
            scope,
            self.is_final,
            self.is_static,
            return_type,
            ast_parameters,
            vec![]
        )
    }
}

#[derive(Clone)]
pub struct ParameterBuilder<'a> {
    param_name: Option<&'a str>,
    param_type: Option<&'a str>,
    is_array: bool,
}

impl <'a> ParameterBuilder<'a> {
    fn new() -> Self {
        Self {
            param_name: None,
            param_type: None,
            is_array: false,
        }
    }

    pub fn with_name(&mut self, name: &'a str) {
        self.param_name = Some(name)
    }

    pub fn with_type(&mut self, param_type: &'a str) {
        self.param_type = Some(param_type)
    }

    pub fn as_array(&mut self) {
        self.is_array = true;
    }
}

impl <'a> Build<AstParameter<'a>> for ParameterBuilder<'a> {
    fn build(self) -> AstParameter<'a> {

        let Some(param_name) = self.param_name else {
            panic!("Param name was not set")
        };

        let Some(param_type) = self.param_type else {
            panic!("Param type was not set")
        };

        AstParameter::new(param_name, param_type, self.is_array)
    }
}

#[derive(Clone)]
pub struct StatementBuilder<'a> {
    line: &'a str,
}

impl <'a> StatementBuilder<'a> {
    pub fn new() -> Self {
        Self {
            line: "todo"
        }
    }
}

impl <'a> Build<AstStatement<'a>> for StatementBuilder<'a> {
    fn build(self) -> AstStatement<'a> {
        AstStatement::new(self.line)
    }
}