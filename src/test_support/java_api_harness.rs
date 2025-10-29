use crate::java::{JavaClass, ClassLoader};

pub struct JavaApiHarness {
    packages: ClassLoader,
}
impl JavaApiHarness {
    pub fn new() -> Self {
        Self {
            packages: ClassLoader::new(),
        }
    }

    pub fn load_class(&mut self, fq_class_name: &str) -> &JavaClass {
        self.packages.load(fq_class_name).unwrap()
    }
}
