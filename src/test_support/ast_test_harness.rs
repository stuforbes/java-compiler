use std::fmt::Display;
use crate::ast::class::{Class, Method, Parameter};
use crate::build;
use crate::io::read_file;
use crate::test_support::{check_and_report_difference, check_and_report_difference_nested, do_comparison, ComparisonResult};

pub fn build_class_from_source_file_and_compare<'a>(file_path: &str, expected_class: Class) {
    let source = read_file(file_path);
    let actual_class = build(source.as_str());

    let result = do_comparison(&expected_class, &actual_class, compare_classes);
    
    assert_eq!(ComparisonResult::Match, result, "Not a match: {:}", result)
}

fn compare_classes(expected_class: &Class, actual_class: &Class, differences: &mut Vec<String>) {

    check_and_report_difference(expected_class.name(), actual_class.name(), "Name", differences);
    check_and_report_difference(expected_class.is_final(), actual_class.is_final(), "Final", differences);
    check_and_report_difference(expected_class.is_static(), actual_class.is_static(), "Static", differences);
    check_and_report_difference(expected_class.scope(), actual_class.scope(), "Scope", differences);
    check_and_report_difference_nested(expected_class.methods(), actual_class.methods(), "Method", differences, check_and_report_differences_in_methods);
}

fn check_and_report_differences_in_methods(expected_method: &Method, actual_method: &Method, name: &str, differences: &mut Vec<String>) {
    check_and_report_difference(expected_method.name(), actual_method.name(), format!("{:}.name", name).as_str(), differences);
    check_and_report_difference(expected_method.return_type(), actual_method.return_type(), format!("{:}.return_type", name).as_str(), differences);
    check_and_report_difference_nested(expected_method.parameters(), actual_method.parameters(), format!("{:}.parameters", name).as_str(), differences, check_and_report_differences_in_parameters);
    check_and_report_difference_nested(expected_method.statements(), actual_method.statements(), format!("{:}.statements", name).as_str(), differences, check_and_report_differences_in_statements);
}

fn check_and_report_differences_in_parameters(expected_parameter: &Parameter, actual_parameter: &Parameter, name: &str, differences: &mut Vec<String>) {
    check_and_report_difference(expected_parameter.param_name(), actual_parameter.param_name(), format!("{:}.param_name", name).as_str(), differences);
    check_and_report_difference(expected_parameter.param_type(), actual_parameter.param_type(), format!("{:}.param_type", name).as_str(), differences);
}

fn check_and_report_differences_in_statements(expected_statement: &&str, actual_statement: &&str, name: &str, differences: &mut Vec<String>) {
    check_and_report_difference(expected_statement, actual_statement, format!("{:}.statement", name).as_str(), differences);
}