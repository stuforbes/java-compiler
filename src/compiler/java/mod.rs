use std::collections::HashMap;
use lazy_static::lazy_static;
use crate::compiler::java::java_lang::build;

mod java_lang;

lazy_static! {
    pub static ref java_packages: Packages = build_java();
}

pub fn build_java() -> Packages {
    Packages {
        packages: as_map(vec![build()])
    }
}

trait Named {
    fn name(&self) -> &'static str;
}

pub struct Packages {
    packages: HashMap<&'static str, Package>
}
impl Packages {
    pub(crate) fn package_and_class_named(&self, name: &str) -> Option<(&Package, &JavaClass)> {
        if let Some(last_dot) = name.rfind('.') {
            let package_name = &name[0..last_dot];
            if let Some(package) = self.package_named(package_name) {
                return package.class_named(&name[last_dot+1..name.len()])
                    .map(|class| (package, class))
            }
        }
        None
    }

    fn package_named(&self, package_name: &str) -> Option<&Package> {
        self.packages.get(package_name)
    }
}

pub struct Package {
    name: &'static str,
    classes: HashMap<&'static str, JavaClass>,
}
impl Named for Package {
    fn name(&self) -> &'static str {
        self.name
    }
}
impl Package {
    pub fn class_named(&self, class_name: &str) -> Option<&JavaClass> {
        self.classes.get(class_name)
    }

    pub fn name(&self) -> &'static str {
        self.name
    }
}

pub struct JavaClass {
    name: &'static str,
    methods: HashMap<&'static str, JavaMethod>,
    fields: HashMap<&'static str, JavaField>,
}
impl Named for JavaClass {
    fn name(&self) -> &'static str {
        self.name
    }
}
impl JavaClass {
    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn method_named(&self, name: &'static str) -> Option<&JavaMethod> {
        self.methods.get(name)
    }

    pub fn field_named(&self, name: &'static str) -> Option<&JavaField> {
        self.fields.get(name)
    }
}

pub struct JavaMethod {
    name: &'static str,
    return_type: &'static str,
    descriptor: &'static str,
}
impl Named for JavaMethod {
    fn name(&self) -> &'static str {
        self.name
    }
}
impl JavaMethod {
    pub fn name(&self) -> &str {
        self.name
    }

    pub fn return_type(&self) -> &str {
        self.return_type
    }

    pub fn descriptor(&self) -> &str {
        self.descriptor
    }
}

pub struct JavaField {
    name: &'static str,
    class: &'static str,
}
impl Named for JavaField {
    fn name(&self) -> &'static str {
        self.name
    }
}
impl JavaField {
    pub fn name(&self) -> &str {
        self.name
    }

    pub fn class(&self) -> &str {
        self.class
    }
}

pub fn as_map<T : Named>(
    items: Vec<T>,
) -> HashMap<&'static str, T> {
    let mut map: HashMap<&'static str, T> = HashMap::new();
    for item in items {
        map.insert(&item.name(), item);
    }
    map
}
