use ristretto_classfile::ConstantPool;
use crate::compiler::resolved_class::ResolvedClass;
use crate::compiler::resolved_class::ResolvedClass::FullyQualified;



fn add_classes_in_object_path(object_path: &str, constant_pool: &mut ConstantPool) -> ristretto_classfile::Result<()>{
    if let Some(FullyQualified(name)) = object_path.find('.')
        .map(|idx| &object_path[0..idx])
        .iter().find_map(|object_name| ResolvedClass::from_name(object_name)) {

        constant_pool.add_class(name)?;

        Ok(())
    } else {
        panic!("Malformed object: {:}", object_path);
    }
}