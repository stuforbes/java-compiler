use crate::ast::class::{Class, Method, Parameter, Scope};
use crate::ast::class_builder::Scope::Default;

pub trait Build<T> {
    fn build(&self) -> T;
}

pub struct ClassBuilder<'a> {
    name: Option<&'a str>,
    scope: Scope,
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

    pub fn with_scope(&mut self, scope: Scope) {
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

impl <'a> Build<Class<'a>> for ClassBuilder<'a> {
    fn build(&self) -> Class<'a> {
        let Some(name) = self.name else {
            panic!("Name was not set")
        };

        Class::new(
            name,
            self.scope,
            self.is_static,
            self.is_final,
            self.methods.iter().map(|m|m.build()).collect()
        )
    }
}

#[derive(Clone)]
#[allow(dead_code)]
pub struct MethodBuilder<'a> {
    name: Option<&'a str>,
    return_type: Option<&'a str>,
    parameters: Vec<ParameterBuilder<'a>>,
    statements: Vec<&'a str>,
}

impl<'a> MethodBuilder<'a> {
    fn new() -> Self {
        Self {
            name: None,
            return_type: None,
            parameters: vec![],
            statements: vec![],
        }
    }

    pub fn with_name(&mut self, name: &'a str) {
        self.name = Some(name)
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
}

impl <'a> Build<Method<'a>> for MethodBuilder<'a> {
    fn build(&self) -> Method<'a> {
        let Some(name) = self.name else {
            panic!("Name was not set")
        };
        let Some(return_type) = self.return_type else {
            panic!("Return type was not set")
        };

        Method::new(
            name,
            return_type,
            self.parameters.iter().map(|p|p.build()).collect(),
            vec![]
        )
    }
}

#[derive(Clone)]
pub struct ParameterBuilder<'a> {
    param_name: Option<&'a str>,
    param_type: Option<&'a str>,
}

impl <'a> ParameterBuilder<'a> {
    fn new() -> Self {
        Self {
            param_name: None,
            param_type: None,
        }
    }

    pub fn with_name(&mut self, name: &'a str) {
        self.param_name = Some(name)
    }

    pub fn with_type(&mut self, param_type: &'a str) {
        self.param_type = Some(param_type)
    }
}

impl <'a> Build<Parameter<'a>> for ParameterBuilder<'a> {
    fn build(&self) -> Parameter<'a> {

        let Some(param_name) = self.param_name else {
            panic!("Param name was not set")
        };

        let Some(param_type) = self.param_type else {
            panic!("Param type was not set")
        };

        Parameter::new(param_name, param_type)
    }
}