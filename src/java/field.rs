use crate::java::Named;

pub struct JavaField {
    name: &'static str,
    class: &'static str,
}
impl Named for JavaField {
    fn name(&self) -> &'static str {
        self.name
    }
}
impl JavaField {
    pub fn new(name: &'static str, class: &'static str) -> Self {
        Self {
            name,
            class,
        }
    }

    pub fn name(&self) -> &str {
        self.name
    }

    pub fn class(&self) -> &str {
        self.class
    }
}
