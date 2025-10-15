use crate::ast::class::{AstClass, AstMethod, AstParameter};
use crate::ast::expression::Expression;
use crate::ast::statement::Statement;
use crate::build_ast;
use crate::io::read_file;
use crate::test_support::{
    check_and_report_difference, check_and_report_difference_nested, do_comparison,
    ComparisonResult,
};

pub fn build_class_from_source_file_and_compare<'a>(file_path: &str, expected_class: AstClass) {
    let source = read_file(file_path);
    let actual_class = build_ast(source.as_str());

    let result = do_comparison(&expected_class, &actual_class, compare_classes);

    assert_eq!(ComparisonResult::Match, result, "Not a match: {:}", result)
}

fn compare_classes(
    expected_class: &AstClass,
    actual_class: &AstClass,
    differences: &mut Vec<String>,
) {
    check_and_report_difference(
        expected_class.name(),
        actual_class.name(),
        "Name",
        differences,
    );
    check_and_report_difference(
        expected_class.is_final(),
        actual_class.is_final(),
        "Final",
        differences,
    );
    check_and_report_difference(
        expected_class.is_static(),
        actual_class.is_static(),
        "Static",
        differences,
    );
    check_and_report_difference(
        expected_class.scope(),
        actual_class.scope(),
        "Scope",
        differences,
    );
    check_and_report_difference_nested(
        expected_class.methods(),
        actual_class.methods(),
        "Method",
        differences,
        check_and_report_differences_in_methods,
    );
}

fn check_and_report_differences_in_methods(
    expected_method: &AstMethod,
    actual_method: &AstMethod,
    name: &str,
    differences: &mut Vec<String>,
) {
    check_and_report_difference(
        expected_method.name(),
        actual_method.name(),
        format!("{:}.name", name).as_str(),
        differences,
    );
    check_and_report_difference(
        expected_method.is_static(),
        actual_method.is_static(),
        format!("{:}.static", name).as_str(),
        differences,
    );
    check_and_report_difference(
        expected_method.is_final(),
        actual_method.is_final(),
        format!("{:}.final", name).as_str(),
        differences,
    );
    check_and_report_difference(
        expected_method.return_type(),
        actual_method.return_type(),
        format!("{:}.return_type", name).as_str(),
        differences,
    );
    check_and_report_difference_nested(
        expected_method.parameters(),
        actual_method.parameters(),
        format!("{:}.parameters", name).as_str(),
        differences,
        check_and_report_differences_in_parameters,
    );
    check_and_report_difference_nested(
        expected_method.statements(),
        actual_method.statements(),
        format!("{:}.statements", name).as_str(),
        differences,
        |expected_statement, actual_statement, name, differences| {
            check_and_report_differences_in_statements(
                expected_statement,
                actual_statement,
                name,
                differences,
            )
        },

    );
}

fn check_and_report_differences_in_parameters(
    expected_parameter: &AstParameter,
    actual_parameter: &AstParameter,
    name: &str,
    differences: &mut Vec<String>,
) {
    check_and_report_difference(
        expected_parameter.param_name(),
        actual_parameter.param_name(),
        format!("{:}.param_name", name).as_str(),
        differences,
    );
    check_and_report_difference(
        expected_parameter.param_type(),
        actual_parameter.param_type(),
        format!("{:}.param_type", name).as_str(),
        differences,
    );
}

fn check_and_report_differences_in_statements(
    expected_statement: &Statement,
    actual_statement: &Statement,
    name: &str,
    differences: &mut Vec<String>,
) {
    match (expected_statement, actual_statement) {
        (
            Statement::Expression {
                expression: expected_expression,
            },
            Statement::Expression {
                expression: actual_expression,
            },
        ) => check_and_report_differences_in_expressions(
            expected_expression,
            actual_expression,
            format!("{:}.expression", name).as_str(),
            differences,
        ),
        (expected, actual) => differences.push(
            format!(
                "{:} is different. Expected {:?} but was {:?}",
                name, expected, actual
            )
            .to_string(),
        ),
    }
}

fn check_and_report_differences_in_expressions(
    expected_expression: &Expression,
    actual_expression: &Expression,
    name: &str,
    differences: &mut Vec<String>,
) {
    match (expected_expression, actual_expression) {
        (
            Expression::Call {
                object_path: expected_object_path,
                method_name: expected_method_name,
                arguments: expected_arguments,
            },
            Expression::Call {
                object_path: actual_object_path,
                method_name: actual_method_name,
                arguments: actual_arguments,
            },
        ) => {
            check_and_report_difference(
                expected_object_path,
                actual_object_path,
                format!("{:}.object_path", name).as_str(),
                differences,
            );
            check_and_report_difference(
                expected_method_name,
                actual_method_name,
                format!("{:}.method_name", name).as_str(),
                differences,
            );
            check_and_report_difference_nested(
                &expected_arguments,
                &actual_arguments,
                format!("{:}.arguments", name).as_str(),
                differences,
                |expected_argument, actual_argument, name, differences| {
                    check_and_report_differences_in_expressions(
                        expected_argument,
                        actual_argument,
                        name,
                        differences,
                    )
                },
            );
        },
        (Expression::StringLiteral { value: expected_value }, Expression::StringLiteral { value: actual_value}) => {
              check_and_report_difference(expected_value, actual_value, format!("{:}.value", name).as_str(), differences);
        },
        (unknown_expected, unknown_actual) => differences.push(
            format!(
                "{:} is different. Expected {:?} but was {:?}",
                name, unknown_expected, unknown_actual
            )
            .to_string(),
        ),
    }
}
