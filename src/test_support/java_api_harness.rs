use crate::java::{java, JavaClass};

pub fn load_class(name: &str) -> Option<&JavaClass> {
    java()
        .package_and_class_named(name)
        .map(|(_, class)| class)
}