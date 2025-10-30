use crate::java::{new_class_loader, ClassLoader};
use crate::java::class::JavaClass;

pub struct JavaApiHarness {
    packages: ClassLoader,
}
impl JavaApiHarness {
    pub fn new() -> Self {
        Self {
            packages: new_class_loader(),
        }
    }

    pub fn load_class(&mut self, fq_class_name: &str) -> &JavaClass {
        self.packages.load(fq_class_name).unwrap()
    }
}
