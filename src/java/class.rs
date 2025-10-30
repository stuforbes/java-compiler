use std::collections::HashMap;
use crate::java::Named;
use crate::java::field::JavaField;
use crate::java::method::JavaMethod;

pub struct JavaClass {
    name: &'static str,
    full_name: &'static str,
    descriptor: &'static str,
    methods: HashMap<&'static str, JavaMethod>,
    fields: HashMap<&'static str, JavaField>,
}
impl Named for JavaClass {
    fn name(&self) -> &'static str {
        self.name
    }
}
impl JavaClass {
    pub fn new(
        name: &'static str,
        full_name: &'static str,
        descriptor: &'static str,
        methods: HashMap<&'static str, JavaMethod>,
        fields: HashMap<&'static str, JavaField>,
    ) -> Self {
        Self {
            name,
            full_name,
            descriptor,
            methods,
            fields,
        }
    }
    
    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn qualified_name(&self) -> &'static str {
        self.full_name
    }

    pub fn descriptor(&self) -> &str {
        self.descriptor
    }

    pub fn method_named(&self, name: &str) -> Option<&JavaMethod> {
        self.methods.get(name)
    }

    pub fn field_named(&self, name: &str) -> Option<&JavaField> {
        self.fields.get(name)
    }
}
