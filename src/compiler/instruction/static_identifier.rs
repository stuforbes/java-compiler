use ristretto_classfile::attributes::Instruction;
use crate::compiler::{wrap, CompilationContext, CompileError, CompileResult};

pub fn from_static_identifier(name: &str, compilation_context: &mut CompilationContext) -> CompileResult<Vec<Instruction>> {
    
    if compilation_context.scoped_object.is_none() {
        let class_path = if let Some(class) = compilation_context.class_loader.load(name) {
            let class_descriptor = class.path().replace('.', "/");
            let class_id = wrap(compilation_context.constant_pool.add_class(&class_descriptor))?;
            Some((class.path().to_string(), class_id))
        } else {
            None
        };

        if let Some((class_path, class_id)) = class_path {
            compilation_context.push_scoped_object(class_path, class_id);
        }
    }
    Err(CompileError::UnknownClass(name.to_string()))
}