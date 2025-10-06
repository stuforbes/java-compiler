pub use comparator::{ComparisonResult, do_comparison, check_and_report_difference, check_and_report_difference_nested};

pub use ast_test_harness::build_class_from_source_file_and_compare;

mod ast_test_harness;
mod comparator;

