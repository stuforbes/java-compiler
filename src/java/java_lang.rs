use crate::java::as_map;
use crate::java::class::JavaClass;
use crate::java::field::JavaField;
use crate::java::package::Package;

pub fn build() -> Package {
    let mut package = Package::new("java/lang");
    package.add_class(load_system());
    package.add_class(load_string());
    package
}

fn load_system() -> JavaClass {
    JavaClass::new(
        "System",
        "java.lang.System",
        "Ljava/lang/System;",
        as_map(vec![]),
        as_map(vec![
            JavaField::new("out", "java.io.PrintStream"),
            JavaField::new("err", "java.io.PrintStream"),
        ]),
    )
}

fn load_string() -> JavaClass {
    JavaClass::new(
        "String",
        "java.lang.String",
        "Ljava/lang/String;",
        as_map(vec![]),
        as_map(vec![]),
    )
}

