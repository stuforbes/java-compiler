use crate::java::as_map;
use crate::java::class::JavaClass;
use crate::java::field::JavaField;
use crate::java::package::Package;

pub fn build() -> Package {
    let system = JavaClass::new(
        "System",
        "java.lang.System",
        "Ljava/lang/System;",
        as_map(vec![]),
        as_map(vec![
            JavaField::new("out", "java.io.PrintStream"),
            JavaField::new("err", "java.io.PrintStream"),
        ]),
    );

    let mut package = Package::new("java/lang");
    package.add_class(system);
    package
}
