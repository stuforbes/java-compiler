use std::process::Command;
use crate::compile;

pub fn compile_source_and_assert_output_is(source_file_path: &str, class_name: &str, expected_output: &str) {
    let result = compile(source_file_path);

    result.unwrap_or_else(|e| panic!("Error encountered: {:?}", e));

    let output = Command::new("java")
        .arg(class_name)
        .output()
        .expect("failed to execute process");

    let actual_error = str::from_utf8(output.stderr.as_slice()).unwrap().trim();
    assert_eq!("", actual_error, "Expected no error");

    let actual_output = str::from_utf8(output.stdout.as_slice()).unwrap().trim();

    assert_eq!(expected_output, actual_output);
}