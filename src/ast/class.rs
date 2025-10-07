
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

impl <'a> AstClass<'a> {
    pub fn new(name: &'a str,
               scope: AstScope,
               is_static: bool,
               is_final: bool,
               methods: Vec<AstMethod<'a>>
    ) -> Self {
        Self {
            name,
            scope,
            is_static,
            is_final,
            methods
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
    return_type: &'a str,
    parameters: Vec<AstParameter<'a>>,
    statements: Vec<&'a str>,
}

impl <'a> AstMethod<'a> {
    pub fn new(name: &'a str,
               return_type: &'a str,
               parameters: Vec<AstParameter<'a>>,
               statements: Vec<&'a str>
    ) -> Self {
        Self {
            name,
            return_type,
            parameters,
            statements
        }
    }

    pub fn name(&self) -> &'a str {
        self.name
    }
    pub fn return_type(&self) -> &'a str {
        self.return_type
    }
    pub fn parameters(&self) -> &Vec<AstParameter<'a>> {
        &self.parameters
    }
    pub fn statements(&self) -> &Vec<&'a str> {
        &self.statements
    }
}

#[derive(Debug)]
pub struct AstParameter<'a> {
    param_name: &'a str,
    param_type: &'a str,
}

impl <'a> AstParameter<'a> {
    pub fn new(param_name: &'a str, param_type: &'a str) -> Self {
        Self {
            param_name,
            param_type
        }
    }

    pub fn param_name(&self) -> &'a str {
        self.param_name
    }
    pub fn param_type(&self) -> &'a str {
        self.param_type
    }
}
