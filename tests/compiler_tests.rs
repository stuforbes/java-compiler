use java_compiler::test_support::compile_source_and_assert_output_is;

#[test]
fn should_compile_simple_hello_world() {
    compile_source_and_assert_output_is("samples/Simple.java", "Simple", "Hello World");
}

#[test]
fn should_compile_string_variable_assignment() {
    compile_source_and_assert_output_is("samples/StringVariableAssignment.java", "StringVariableAssignment", "a string variable");
}

#[test]
fn should_compile_local_method_invocation_returning_a_string() {
    compile_source_and_assert_output_is("samples/MethodInvocationReturningString.java", "MethodInvocationReturningString", "a string returned from a method"); 
}

#[test]
#[ignore]
fn should_compile_local_method_invocation_returning_a_string_array() {
    compile_source_and_assert_output_is("samples/MethodInvocationReturningStringArray.java", "MethodInvocationReturningStringArray", "a string array returned from a method");
}