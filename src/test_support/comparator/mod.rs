use std::fmt::{Debug, Display, Formatter};

#[derive(PartialEq, Debug)]
pub enum ComparisonResult {
    Match,
    NoMatch { differences: Vec<String> },
}
impl Display for ComparisonResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ComparisonResult::Match => write!(f, "Match"),
            ComparisonResult::NoMatch { differences } => {
                write!(f, "Not a match. There were {:} observed differences: {:}\n",
                differences.len(),
                    differences
                           .iter()
                           .map(|d| format!("\t{:}", d))
                           .collect::<Vec<_>>()
                           .join("\n")
                )
            }
        }
    }
}

pub fn do_comparison<A, F>(expected: &A, actual: &A, top_level_name: &str, comparator: F) -> ComparisonResult
where A: Debug,
    F: Fn(&A, &A, &str, &mut Vec<String>)
{

    let mut differences: Vec<String> = vec![];

    comparator(expected, actual, top_level_name, &mut differences);

    if differences.is_empty() {
        ComparisonResult::Match
    } else {
        ComparisonResult::NoMatch { differences }
    }
}

pub fn check_and_report_difference<A: Sized>(expected: A, actual: A, field_name: &str, differences: &mut Vec<String>)
where A: PartialEq<A> + Debug {
    if expected != actual {
        differences.push(format!("{:} is different. Expected {:?} but was {:?}", field_name, expected, actual).to_string())
    }
}

pub fn check_and_report_difference_nested<A, F>(
    expected_nested: &Vec<A>,
    actual_nested: &Vec<A>,
    name: &str,
    differences: &mut Vec<String>,
    nested_comparator: F,
)
where A: Debug,
      F: Fn(&A, &A, &str, &mut Vec<String>),
{
    if expected_nested.len() != actual_nested.len() {
        differences.push(format!("{:?}: Incorrect number. Expected {:?} but was {:?}. Expected: {:?}; Actual: {:?}", name, expected_nested.len(), actual_nested.len(), expected_nested, actual_nested).to_string());
    } else {
        for (i, (expected, actual)) in expected_nested.iter().zip(actual_nested.iter()).enumerate() {
            let nested_name = format!("{:}[{:}]", name, i);
            nested_comparator(expected, actual, &nested_name, differences)
        }
    }
}

pub fn check_and_report_difference_option<A, F>(
    expected_option: &Option<A>,
    actual_option: &Option<A>,
    name: &str,
    differences: &mut Vec<String>,
    value_comparator: F,
)
where A: Debug + Clone,
      F: Fn(&A, &A, &str, &mut Vec<String>),
{
    if expected_option.is_some() && actual_option.is_none() {
        differences.push(format!("{:?}: Expected {:?} but was missing", name, &expected_option.clone().unwrap()))
    } else if expected_option.is_none() && actual_option.is_some() {
        differences.push(format!("{:?}: Expected empty but was {:?}", name, &actual_option.clone().unwrap()))
    } else if expected_option.is_some() && actual_option.is_some() {
        value_comparator(&expected_option.clone().unwrap(), &actual_option.clone().unwrap(), name, differences);
    }
}
