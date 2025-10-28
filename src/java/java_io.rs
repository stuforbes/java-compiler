use crate::java::{as_map, JavaClass, JavaMethod};
use lazy_static::lazy_static;

lazy_static! {
    static ref CLASSES: Vec<&'static str> = vec!["PrintStream"];
}

pub fn contains_class(name: &str) -> bool {
    CLASSES.contains(&name)
}

pub fn load_class(name: &str) -> JavaClass {
    match name {
        "PrintStream" => JavaClass {
            name: "PrintStream",
            full_name: "java/io/PrintStream",
            descriptor: "Ljava/io/PrintStream;",
            fields: as_map(vec![]),
            methods: as_map(vec![
                JavaMethod {
                    name: "println",
                    return_type: "void",
                    descriptor: "(Ljava/lang/String;)V",
                }
            ]),
        },
        _ => panic!("Class {:} does not exist in java.io", name),
    }
}
