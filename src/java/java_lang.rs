use crate::java::{as_map, JavaClass, JavaField, Package};

pub fn build() -> Package {
    let system = JavaClass {
        name: "System",
        fields: as_map(vec![
            JavaField {
                name: "out",
                class: "java.io.PrintStream",
            },
            JavaField {
                name: "err",
                class: "java.io.PrintStream",
            },
        ]),
        methods: as_map(vec![]),
    };

    Package {
        name: "java.lang",
        classes: as_map(vec![system]),
    }
}
