use lazy_static::lazy_static;
use std::collections::HashMap;
use crate::java::{build_java, Package};

pub struct ResolvedSymbol<'symbol> {
    object_class: &'symbol String,
    field_class: &'symbol String,
    descriptor: &'symbol String,
}