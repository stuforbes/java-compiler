mod compiler;
mod testassertion;

pub fn compile(class_name: &str) {
    compiler::compile(class_name);
}

pub fn assert_output_is(class_name: &str, expected_output: &str) {
    testassertion::assert_output_is(class_name, expected_output);
}