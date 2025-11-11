use java_compiler::ast::class::{AstClass, AstMethod, AstParameter, AstScope};
use java_compiler::ast::expression::Expression;
use java_compiler::ast::statement::Statement;
use java_compiler::test_support::{build_class_from_source_file_and_compare, build_method_only_and_compare};

#[test]
fn should_build_simple_ast() {
    build_class_from_source_file_and_compare(
        "samples/Simple.java",
        AstClass::new(
            "Simple",
            AstScope::Public,
            false,
            false,
            vec![AstMethod::new(
                "main",
                AstScope::Public,
                false,
                true,
                "void",
                vec![AstParameter::new("args", "String", false)],
                vec![Statement::new_expression_statement(Expression::new_call(
                    Expression::new_child_identifier(Expression::new_variable("System", None), "out"),
                    "println",
                    vec![Expression::new_string_literal("Hello World")],
                ))],
            )],
        ),
    );
}

#[test]
fn should_build_method_with_string_variable_assignment() {
    build_method_only_and_compare(
        r#"
        public static void main(String[] args) {
            String message = "hello";
            System.out.println(message);
        }
        "#,
        AstMethod::new(
            "main",
            AstScope::Public,
            false,
            true,
            "void",
            vec![AstParameter::new("args", "String", false)],
            vec![
                Statement::new_expression_statement(Expression::new_assignment("message", Some("String"), Expression::new_string_literal("hello"))),
                Statement::new_expression_statement(Expression::new_call(
                    Expression::new_child_identifier(Expression::new_variable("System", None), "out"),
                    "println",
                    vec![Expression::new_variable("message", None)],
                )),
            ],
        ),
    );
}
