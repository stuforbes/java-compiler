use std::collections::HashMap;

mod java_io;
mod java_lang;

trait Named {
    fn name(&self) -> &'static str;
}

pub struct ClassLoader {
    packages: HashMap<&'static str, Package>,
}
impl ClassLoader {
    pub(crate) fn new() -> Self {
        let mut packages = HashMap::new();
        // java.lang is available as an implicit import to every class
        packages.insert("java.lang", java_lang::build());
        Self { packages }
    }

    pub fn load(&mut self, fully_qualified_class_name: &str) -> Option<&JavaClass> {
        let (package_name, class_name) = self.packagify(fully_qualified_class_name);

        if package_name == "java.io" {
            if !self.packages.contains_key("java.io") {
                self.packages.insert("java.io", Package::new("java.io"));
            }

            let package_rc = self.packages.get_mut(package_name).unwrap();

            if package_rc.class_named(class_name).is_none() && java_io::contains_class(class_name) {
                package_rc.add_class(java_io::load_class(class_name));
            }
        }

        if self.packages.contains_key(package_name) {
            return self
                .packages
                .get(package_name)
                .and_then(|p| p.class_named(class_name));
        }
        None
    }

    fn packagify<'a>(&self, name: &'a str) -> (&'a str, &'a str) {
        if let Some(last_dot) = name.rfind('.') {
            return (&name[0..last_dot], &name[last_dot + 1..name.len()]);
        }
        if let Some(java_lang) = self.packages.get("java.lang") {
            if java_lang.class_named(name).is_some() {
                return ("java.lang", name);
            }
        }
        ("", name)
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
    fn new(name: &'static str) -> Self {
        Self {
            name,
            classes: HashMap::new(),
        }
    }

    fn add_class(&mut self, class: JavaClass) {
        self.classes.insert(class.name(), class);
    }

    pub fn class_named(&self, class_name: &str) -> Option<&JavaClass> {
        self.classes.get(class_name)
    }

    pub fn name(&self) -> &'static str {
        self.name
    }
}

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

pub fn as_map<T: Named>(items: Vec<T>) -> HashMap<&'static str, T> {
    let mut map: HashMap<&'static str, T> = HashMap::new();
    for item in items {
        map.insert(&item.name(), item);
    }
    map
}
