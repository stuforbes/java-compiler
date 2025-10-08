use java_compiler::ast::class::{AstClass, AstMethod, AstParameter, AstScope};
use java_compiler::test_support::build_class_from_source_file_and_compare;

#[test]
fn should_build_simple_ast() {
    build_class_from_source_file_and_compare(
        "samples/Simple.java",
        AstClass::new(
            "Simple",
            AstScope::Public,
            false,
            false,
            vec![
                AstMethod::new(
                    "main",
                    AstScope::Public,
                    false,
                    false,
                    "void",
                    vec![AstParameter::new("args", "String", false)],
                    vec![]
                )
            ]
        )
    );
}