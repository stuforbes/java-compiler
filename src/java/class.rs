use std::collections::HashMap;
use crate::java::Named;
use crate::java::field::JavaField;
use crate::java::method::JavaMethod;

pub struct JavaClass {
    name: &'static str,
    path: &'static str,
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
        path: &'static str,
        descriptor: &'static str,
        methods: HashMap<&'static str, JavaMethod>,
        fields: HashMap<&'static str, JavaField>,
    ) -> Self {
        Self {
            name,
            path,
            descriptor,
            methods,
            fields,
        }
    }
    
    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn path(&self) -> &'static str {
        self.path
    }

    pub fn full_name(&self) -> String {
        self.path.replace('.', "/")
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
