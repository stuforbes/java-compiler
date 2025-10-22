use java_compiler::test_support::load_class;

#[test]
fn should_load_java_lang_class_using_fully_qualified_name() {
    let v = vec!["java", "lang", "System"];
    let (java_lang, system, suffix) = load_class(v.as_ref()).unwrap();

    assert_eq!("java/lang", java_lang.name());
    assert_eq!("System", system.name());
    assert!(suffix.is_empty());

    let sys_out = system.field_named("out").unwrap();
    assert_eq!("out", sys_out.name());
    assert_eq!("java.io.PrintStream", sys_out.class());
}

#[test]
fn should_load_java_lang_class_using_relative_name() {
    let v = vec!["System"];
    let (java_lang, system, suffix) = load_class(v.as_ref()).unwrap();

    assert_eq!("java/lang", java_lang.name());
    assert_eq!("System", system.name());
    assert!(suffix.is_empty())
}

#[test]
fn should_fail_to_load_class_that_doesnt_exist() {
    assert!(load_class(vec!["java.lang.Unknown"].as_ref()).is_none());
}

#[test]
fn should_capture_static_field_and_method_names_as_the_suffix() {
    let v = vec!["System", "out", "println"];
    let (java_lang, system, suffix) = load_class(v.as_ref()).unwrap();

    assert_eq!("java/lang", java_lang.name());
    assert_eq!("System", system.name());
    assert_eq!(vec!["out", "println"], suffix)
}