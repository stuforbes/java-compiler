use crate::ast::class::{AstClass, AstMethod, AstParameter, AstScope};
use crate::ast::class_builder::AstScope::Default;
use crate::ast::statement::Statement;

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

pub struct MethodBuilder<'a> {
    name: Option<&'a str>,
    scope: Option<AstScope>,
    is_static: bool,
    is_final: bool,
    return_type: Option<&'a str>,
    return_type_is_array: bool,
    parameters: Vec<ParameterBuilder<'a>>,
    statements: Vec<Statement<'a>>,
}

impl<'a> MethodBuilder<'a> {
    fn new() -> Self {
        Self {
            name: None,
            scope: None,
            is_static: false,
            is_final: false,
            return_type: None,
            return_type_is_array: false,
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

    #[allow(dead_code)]
    pub fn as_final(&mut self) {
        self.is_final = true;
    }

    pub fn with_return_type(&mut self, return_type: &'a str) {
        self.return_type = Some(return_type)
    }

    pub fn return_type_is_array(&mut self, return_type_is_array: bool) {
        self.return_type_is_array = return_type_is_array
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
    
    pub fn with_statements(&mut self, statements: Vec<Statement<'a>>) {
        for statement in statements {
            self.statements.push(statement);
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
            self.return_type_is_array,
            ast_parameters,
            self.statements
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