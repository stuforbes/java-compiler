use std::collections::HashMap;
pub(crate) use crate::java::class_loader::ClassLoader;

mod class_loader;
mod java_io;
mod java_lang;
mod package;
pub mod class;
pub mod method;
pub mod field;

pub fn new_class_loader() -> ClassLoader {
    ClassLoader::new()
}

pub trait Named {
    fn name(&self) -> &'static str;
}

pub fn as_map<T: Named>(items: Vec<T>) -> HashMap<&'static str, T> {
    let mut map: HashMap<&'static str, T> = HashMap::new();
    for item in items {
        map.insert(&item.name(), item);
    }
    map
}