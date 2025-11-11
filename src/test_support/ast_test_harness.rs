use std::fmt::format;
use crate::ast::class::{AstClass, AstMethod, AstParameter};
use crate::ast::expression::Expression;
use crate::ast::statement::Statement;
use crate::build_ast;
use crate::io::read_file;
use crate::test_support::{check_and_report_difference, check_and_report_difference_nested, do_comparison, ComparisonResult};

const CLASS_WRAPPER: &str = r"
    public class Simple {
        %%
    }
";

pub fn build_class_from_source_file_and_compare(file_path: &str, expected_class: AstClass) {
    let source = read_file(file_path);
    let actual_class = build_ast(source.as_str());

    let result = do_comparison(&expected_class, &actual_class, "Class", check_and_report_difference_in_class);

    assert_eq!(ComparisonResult::Match, result, "Not a match: {:}", result)
}

pub fn build_method_only_and_compare(content: &str, expected_method: AstMethod) {
    let source = CLASS_WRAPPER.replace("%%", content);

    let actual_class = build_ast(source.as_str());

    assert_eq!(1, actual_class.methods().len());

    let actual_first_method = &actual_class.methods().first().unwrap();
    let result = do_comparison(&expected_method, actual_first_method, "Method", check_and_report_differences_in_methods);

    assert_eq!(ComparisonResult::Match, result, "Not a match {:}", result);
}

fn check_and_report_difference_in_class(expected_class: &AstClass, actual_class: &AstClass, name: &str, differences: &mut Vec<String>) {
    check_and_report_difference(
        expected_class.name(),
        actual_class.name(),
        format!("{:}.name", name).as_str(),
        differences,
    );
    check_and_report_difference(
        expected_class.is_final(),
        actual_class.is_final(),
        format!("{:}.final", name).as_str(),
        differences,
    );
    check_and_report_difference(
        expected_class.is_static(),
        actual_class.is_static(),
        format!("{:}.static", name).as_str(),
        differences,
    );
    check_and_report_difference(
        expected_class.scope(),
        actual_class.scope(),
        format!("{:}.scope", name).as_str(),
        differences,
    );
    check_and_report_difference_nested(
        expected_class.methods(),
        actual_class.methods(),
        format!("{:}.method", name).as_str(),
        differences,
        check_and_report_differences_in_methods,
    );
}

fn check_and_report_differences_in_methods(expected_method: &AstMethod, actual_method: &AstMethod, name: &str, differences: &mut Vec<String>) {
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
            check_and_report_differences_in_statements(expected_statement, actual_statement, name, differences)
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

#[allow(unreachable_patterns)]
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
        (expected, actual) => differences.push(format!("{:} is different. Expected {:?} but was {:?}", name, expected, actual).to_string()),
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
                target: expected_target,
                method_name: expected_method_name,
                arguments: expected_arguments,
            },
            Expression::Call {
                target: actual_target,
                method_name: actual_method_name,
                arguments: actual_arguments,
            },
        ) => {
            check_and_report_differences_in_expressions(expected_target, actual_target, format!("{:}.target", name).as_str(), differences);
            check_and_report_difference(
                &expected_method_name,
                &actual_method_name,
                format!("{:}.method_name", name).as_str(),
                differences
            );
            check_and_report_difference_nested(
                &expected_arguments,
                &actual_arguments,
                format!("{:}.arguments", name).as_str(),
                differences,
                |expected_argument, actual_argument, name, differences| {
                    check_and_report_differences_in_expressions(expected_argument, actual_argument, name, differences)
                },
            );
        }
        (Expression::StringLiteral { value: expected_value }, Expression::StringLiteral { value: actual_value }) => {
            check_and_report_difference(expected_value, actual_value, format!("{:}.value", name).as_str(), differences);
        }
        (
            Expression::Variable {
                name: expected_name,
                type_def: expected_type_def,
            },
            Expression::Variable {
                name: actual_name,
                type_def: actual_type_def,
            },
        ) => {
            check_and_report_difference(expected_name, actual_name, format!("{:}.name", name).as_str(), differences);
            check_and_report_difference(expected_type_def, actual_type_def, format!("{:}.type_def", name).as_str(), differences);
        }
        (
            Expression::ChildIdentifier {
                parent: expected_parent,
                name: expected_name,
            },
            Expression::ChildIdentifier {
                parent: actual_parent,
                name: actual_name,
            },
        ) => {
            check_and_report_differences_in_expressions(expected_parent, actual_parent, format!("{:}.parent", name).as_str(), differences);
            check_and_report_difference(expected_name, actual_name, format!("{:}.name", name).as_str(), differences);
        }
        (
            Expression::Assignment {
                name: expected_name,
                type_def: expected_type_def,
                value: expected_value,
            },
            Expression::Assignment {
                name: actual_name,
                type_def: actual_type_def,
                value: actual_value,
            },
        ) => {
            check_and_report_difference(expected_name, actual_name, format!("{:}.name", name).as_str(), differences);
            check_and_report_difference(expected_type_def, actual_type_def, format!("{:}.type_def", name).as_str(), differences);
            check_and_report_differences_in_expressions(expected_value, actual_value, format!("{:}.value", name).as_str(), differences);
        }
        (unknown_expected, unknown_actual) => {
            differences.push(format!("{:} is different. Expected {:?} but was {:?}", name, unknown_expected, unknown_actual).to_string())
        }
    }
}
