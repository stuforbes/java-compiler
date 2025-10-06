use java_compiler::ast::class::{Class, Method, Parameter, Scope};
use java_compiler::test_support::build_class_from_source_file_and_compare;

#[test]
fn should_build_simple_ast() {
    build_class_from_source_file_and_compare(
        "samples/Simple.java",
        Class::new(
            "Simple",
            Scope::Public,
            false,
            false,
            vec![
                Method::new(
                    "main",
                    "void",
                    vec![Parameter::new("args", "String")],
                    vec![]
                )
            ]
        )
    );
}