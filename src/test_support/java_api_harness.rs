use std::sync::OnceLock;
use crate::java::{JavaClass, Package, Packages};

static PACKAGES: OnceLock<Packages> = OnceLock::new();

pub fn load_class<'a>(path: &'a [&'a str]) -> Option<(&'a Package, &'a JavaClass, &'a [&'a str])> {
    PACKAGES.get_or_init(Packages::new).parse_object_path(path)
}