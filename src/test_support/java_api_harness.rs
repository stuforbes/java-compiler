use crate::java::{java, JavaClass, Package};

pub fn load_class<'a>(path: &'a [&'a str]) -> Option<(&'a Package, &'a JavaClass, &'a [&'a str])> {
    java()
        .parse_object_path(path)
}