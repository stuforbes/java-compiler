use std::collections::HashMap;
use crate::java::Named;
use crate::java::class::JavaClass;

pub(crate) struct Package {
    name: &'static str,
    classes: HashMap<&'static str, JavaClass>,
}
impl Named for Package {
    fn name(&self) -> &'static str {
        self.name
    }
}

impl Package {
    pub fn new(name: &'static str) -> Self {
        Self {
            name,
            classes: HashMap::new(),
        }
    }

    pub(crate) fn add_class(&mut self, class: JavaClass) {
        self.classes.insert(class.name(), class);
    }

    pub fn class_named(&self, class_name: &str) -> Option<&JavaClass> {
        self.classes.get(class_name)
    }
}
