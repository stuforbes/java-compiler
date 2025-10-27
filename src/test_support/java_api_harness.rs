use crate::java::{JavaClass, Packages};

pub struct JavaApiHarness {
    packages: Packages,
}
impl JavaApiHarness {
    pub fn new() -> Self {
        Self {
            packages: Packages::new(),
        }
    }

    pub fn split_path_into_components<'a>(
        &'a self,
        path: &'a [&'a str],
    ) -> Option<(String, String, Vec<&str>)> {
        self.packages.parse_object_path(path)
            .map(|(package, class, suffix)| (package.to_string(), class.to_string(), suffix.to_vec()))
    }

    pub fn load_class(&mut self, fq_class_name: &str) -> &JavaClass {
        self.packages.class_for(fq_class_name).unwrap()
    }
}
