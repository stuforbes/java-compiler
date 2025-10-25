use crate::java::{as_map, JavaClass};
use lazy_static::lazy_static;

lazy_static! {
    static ref CLASSES: Vec<&'static str> = vec!["PrintStream"];
}

pub fn contains_class(name: &str) -> bool {
    CLASSES.contains(&name)
}

pub fn load_class(name: &str) -> JavaClass {
    match name {
        "PrintStream " => JavaClass {
            name: "PrintStream",
            descriptor: "Ljava/lang/PrintStream;",
            fields: as_map(vec![]),
            methods: as_map(vec![]),
        },
        _ => panic!("Class {:} does not exist in java.io", name),
    }
}
