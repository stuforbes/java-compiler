use java_compiler::test_support::load_class;

#[test]
fn should_load_java_lang_class_using_fully_qualified_name() {
    let system = load_class("java.lang.System").unwrap();

    assert_eq!("System", system.name());

    let sys_out = system.field_named("out").unwrap();
    assert_eq!("out", sys_out.name());
    assert_eq!("java.io.PrintStream", sys_out.class());
}

#[test]
fn should_load_java_lang_class_using_relative_name() {
    let system = load_class("System").unwrap();

    assert_eq!("System", system.name());
}

#[test]
fn should_fail_to_load_class_that_doesnt_exist() {
    assert!(load_class("java.lang.Unknown").is_none());
}