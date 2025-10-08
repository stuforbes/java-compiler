use lazy_static::lazy_static;
use crate::compiler::resolved_class::ResolvedClass::FullyQualified;

const FORWARD_SLASH: &str = "/";

lazy_static! {
    // TODO: This is incomplete
    static ref JAVA_LANG_TYPES: Vec<&'static str> = vec![
        "String",
    ];
}

pub enum ResolvedClass {
    FullyQualified(String),
    Unqualified(String),
}
impl ResolvedClass {
    pub fn from_name(name: &str) -> Option<ResolvedClass> {
        if name.contains('.') {
            return Some(FullyQualified(name.replace('.', FORWARD_SLASH)))
        }

        if JAVA_LANG_TYPES.contains(&name) {
            return Some(FullyQualified(format!("java/lang/{:}", name)))
        }

        None
    }
    
    pub fn fully_qualified(&self) -> &String {
        match self {
            FullyQualified(path) => path,
            ResolvedClass::Unqualified(path) => path,
        }
    }
}
