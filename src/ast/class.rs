
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Scope {
    Public,
    Protected,
    Private,
    Default,
}

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

    pub fn name(&self) -> &str {
        self.name
    }
    pub fn scope(&self) -> Scope {
        self.scope
    }
    pub fn is_static(&self) -> bool {
        self.is_static
    }
    pub fn is_final(&self) -> bool {
        self.is_final
    }
    pub fn methods(&self) -> &Vec<Method<'a>> {
        &self.methods
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

    pub fn name(&self) -> &'a str {
        self.name
    }
    pub fn return_type(&self) -> &'a str {
        self.return_type
    }
    pub fn parameters(&self) -> &Vec<Parameter<'a>> {
        &self.parameters
    }
    pub fn statements(&self) -> &Vec<&'a str> {
        &self.statements
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

    pub fn param_name(&self) -> &'a str {
        self.param_name
    }
    pub fn param_type(&self) -> &'a str {
        self.param_type
    }
}
