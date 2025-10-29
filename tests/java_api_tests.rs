use java_compiler::test_support::java_api_harness::JavaApiHarness;

// #[test]
// fn should_split_java_lang_system_into_correct_components() {
//     let mut harness = JavaApiHarness::new();
//     let v = vec!["java", "lang", "System"];
//     let (java_lang, system, suffix) = harness.split_path_into_components(v.as_ref()).unwrap();
// 
//     assert_eq!("java/lang", java_lang);
//     assert_eq!("System", system);
//     assert!(suffix.is_empty());
// }
// 
// #[test]
// fn should_split_java_lang_system_using_relative_name() {
//     let harness = JavaApiHarness::new();
// 
//     let v = vec!["System"];
//     let (java_lang, system, suffix) = harness.split_path_into_components(v.as_ref()).unwrap();
// 
//     assert_eq!("java/lang", java_lang);
//     assert_eq!("System", system);
//     assert!(suffix.is_empty())
// }
// 
// #[test]
// fn should_fail_to_split_class_that_doesnt_exist() {
//     let harness = JavaApiHarness::new();
// 
//     assert!(
//         harness
//             .split_path_into_components(vec!["java.lang.Unknown"].as_ref())
//             .is_none()
//     );
// }
// 
// #[test]
// fn should_capture_static_field_and_method_names_as_the_suffix() {
//     let harness = JavaApiHarness::new();
// 
//     let v = vec!["System", "out", "println"];
//     let (java_lang, system, suffix) = harness.split_path_into_components(v.as_ref()).unwrap();
// 
//     assert_eq!("java/lang", java_lang);
//     assert_eq!("System", system);
//     assert_eq!(vec!["out", "println"], suffix)
// }
// 
#[test]
fn should_load_system_class() {
    let mut harness = JavaApiHarness::new();

    let system_class = harness.load_class("java.lang.System");
    let sys_out = system_class.field_named("out").unwrap();
    assert_eq!("out", sys_out.name());
    assert_eq!("java.io.PrintStream", sys_out.class());

}