use std::collections::HashMap;
use crate::java::{java_io, java_lang};
use crate::java::class::JavaClass;
use crate::java::package::Package;

pub struct ClassLoader {
    packages: HashMap<&'static str, Package>,
}
impl ClassLoader {
    pub(super) fn new() -> Self {
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

            let package = self.packages.get_mut(package_name).unwrap();

            if package.class_named(class_name).is_none() && java_io::contains_class(class_name) {
                package.add_class(java_io::load_class(class_name));
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
