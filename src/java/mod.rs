use std::collections::HashMap;

mod java_io;
mod java_lang;

trait Named {
    fn name(&self) -> &'static str;
}

pub struct Packages {
    packages: HashMap<&'static str, Package>,
}
impl Packages {
    pub(crate) fn new() -> Self {
        let mut packages = HashMap::new();
        // java.lang is available as an implicit import to every class
        packages.insert("java.lang", java_lang::build());
        Self { packages }
    }

    pub fn parse_object_path<'a>(
        &self,
        path: &'a [&'a str],
    ) -> Option<(&str, &str, &'a [&'a str])> {
        for i in (1..=path.len()).rev() {
            let (prefix, suffix) = path.split_at(i);

            if let Some((package, class)) = self.find_package_and_class_named(prefix) {
                return Some((package.name(), class.name(), suffix));
            }
        }
        None
    }

    pub fn class_for(&mut self, fully_qualified_class_name: &str) -> Option<&JavaClass> {
        let (package_name, class_name) = Packages::packagify(fully_qualified_class_name);

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

    fn find_package_and_class_named(&self, path: &[&str]) -> Option<(&Package, &JavaClass)> {
        let path_str = path.join(".");
        let (package, name) = Packages::packagify(path_str.as_str());
        if package.is_empty() {
            self.package_and_class_for_relative_name(name)
        } else {
            self.package_and_class_for(package, name)
        }
    }

    fn packagify(name: &str) -> (&str, &str) {
        if let Some(last_dot) = name.rfind('.') {
            (&name[0..last_dot], &name[last_dot + 1..name.len()])
        } else {
            ("", name)
        }
    }

    fn package_and_class_for_relative_name(&self, name: &str) -> Option<(&Package, &JavaClass)> {
        // TODO: Classes that are local to the source class will need to be resolved here
        let java_lang = self.packages.get("java.lang").unwrap();

        let class_exists = java_lang.class_named(name).is_some();
        if !class_exists {
            None
        } else {
            Some((java_lang, java_lang.class_named(name).unwrap()))
        }
    }

    fn package_and_class_for(
        &self,
        package_name: &str,
        class_name: &str,
    ) -> Option<(&Package, &JavaClass)> {
        if let Some(package) = self.package_named(package_name) {
            package
                .class_named(class_name)
                .map(|class| (package, class))
        } else {
            None
        }
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
