use crate::java::as_map;
use lazy_static::lazy_static;
use crate::java::class::JavaClass;
use crate::java::method::JavaMethod;

lazy_static! {
    static ref CLASSES: Vec<&'static str> = vec!["PrintStream"];
}

pub fn contains_class(name: &str) -> bool {
    CLASSES.contains(&name)
}

pub fn load_class(name: &str) -> JavaClass {
    match name {
        "PrintStream" => JavaClass::new(
            "PrintStream",
            "java.io.PrintStream",
            "Ljava/io/PrintStream;",
            as_map(vec![
                JavaMethod::new("println", "void", "(Ljava/lang/String;)V")
            ]),
            as_map(vec![]),
        ),
        _ => panic!("Class {:} does not exist in java.io", name),
    }
}
