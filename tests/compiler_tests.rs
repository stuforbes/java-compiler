use java_compiler::test_support::compile_source_and_assert_output_is;

#[test]
fn should_compile_simple_hello_world() {
    compile_source_and_assert_output_is("samples/Simple.java", "Simple", "Hello World");
}