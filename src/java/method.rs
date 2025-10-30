use crate::java::Named;

pub struct JavaMethod {
    name: &'static str,
    return_type: &'static str,
    descriptor: &'static str,
}
impl Named for JavaMethod {
    fn name(&self) -> &'static str {
        self.name
    }
}
impl JavaMethod {
    pub fn new(
        name: &'static str,
        return_type: &'static str,
        descriptor: &'static str,
    ) -> Self {
        Self {
            name,
            return_type,
            descriptor,
        }
    }
    pub fn name(&self) -> &str {
        self.name
    }

    pub fn return_type(&self) -> &str {
        self.return_type
    }

    pub fn descriptor(&self) -> &str {
        self.descriptor
    }
}
