use crate::java::{java_packages, JavaClass};

pub fn load_class(name: &str) -> Option<&JavaClass> {
    java_packages.package_and_class_named(name)
        .map(|(_, class)| class)
}