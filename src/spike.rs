mod compiler;
mod testassertion;

#[allow(dead_code)]
pub fn compile(class_name: &str) {
    compiler::compile(class_name);
}

#[allow(dead_code)]
pub fn assert_output_is(class_name: &str, expected_output: &str) {
    testassertion::assert_output_is(class_name, expected_output);
}