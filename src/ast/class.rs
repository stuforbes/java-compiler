use crate::ast::statement::Statement;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum AstScope {
    Public,
    Protected,
    Private,
    Default,
}

#[derive(Debug)]
pub struct AstClass<'a> {
    name: &'a str,
    scope: AstScope,
    is_static: bool,
    is_final: bool,
    methods: Vec<AstMethod<'a>>,
}

impl<'a> AstClass<'a> {
    pub fn new(
        name: &'a str,
        scope: AstScope,
        is_static: bool,
        is_final: bool,
        methods: Vec<AstMethod<'a>>,
    ) -> Self {
        Self {
            name,
            scope,
            is_static,
            is_final,
            methods,
        }
    }

    pub fn name(&self) -> &str {
        self.name
    }
    pub fn scope(&self) -> AstScope {
        self.scope
    }
    pub fn is_static(&self) -> bool {
        self.is_static
    }
    pub fn is_final(&self) -> bool {
        self.is_final
    }
    pub fn methods(&self) -> &Vec<AstMethod<'a>> {
        &self.methods
    }
}

#[derive(Debug)]
pub struct AstMethod<'a> {
    name: &'a str,
    scope: AstScope,
    is_final: bool,
    is_static: bool,
    return_type: &'a str,
    return_type_is_array: bool,
    parameters: Vec<AstParameter<'a>>,
    statements: Vec<Statement<'a>>,
}

impl<'a> AstMethod<'a> {
    pub fn new(
        name: &'a str,
        scope: AstScope,
        is_final: bool,
        is_static: bool,
        return_type: &'a str,
        return_type_is_array: bool,
        parameters: Vec<AstParameter<'a>>,
        statements: Vec<Statement<'a>>,
    ) -> Self {
        Self {
            name,
            scope,
            is_final,
            is_static,
            return_type,
            return_type_is_array,
            parameters,
            statements,
        }
    }

    pub fn name(&self) -> &'a str {
        self.name
    }

    pub fn scope(&self) -> AstScope {
        self.scope
    }

    pub fn is_final(&self) -> bool {
        self.is_final
    }

    pub fn is_static(&self) -> bool {
        self.is_static
    }

    pub fn return_type(&self) -> &'a str {
        self.return_type
    }
    pub fn return_type_is_array(&self) -> bool {
        self.return_type_is_array
    }
    pub fn parameters(&self) -> &Vec<AstParameter<'a>> {
        &self.parameters
    }
    pub fn statements(&self) -> &Vec<Statement<'a>> {
        &self.statements
    }
}

#[derive(Debug)]
pub struct AstParameter<'a> {
    param_name: &'a str,
    param_type: &'a str,
    is_array: bool,
}

impl <'a> AstParameter<'a> {
    pub fn new(param_name: &'a str, param_type: &'a str, is_array: bool) -> Self {
        Self {
            param_name,
            param_type,
            is_array,
        }
    }

    pub fn param_name(&self) -> &'a str {
        self.param_name
    }
    pub fn param_type(&self) -> &'a str {
        self.param_type
    }

    pub fn is_array(&self) -> bool {
        self.is_array
    }
}
