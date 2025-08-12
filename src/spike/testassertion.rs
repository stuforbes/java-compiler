use std::process::Command;

pub fn assert_output_is(class_name: &str, expected_output: &str) {
    let output = Command::new("java")
        .arg(class_name)
        .output()
        .expect("Failed to run Java {class_name}".replace("{class_name}", class_name).as_str());

    let stdout = output.stdout;

    assert_eq!(expected_output, str::from_utf8(&stdout).unwrap());
}