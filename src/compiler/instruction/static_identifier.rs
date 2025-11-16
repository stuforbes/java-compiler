use ristretto_classfile::attributes::Instruction;
use crate::compiler::{wrap, CompilationContext, CompileError, CompileResult};

pub fn from_static_identifier(name: &str, compilation_context: &mut CompilationContext) -> CompileResult<Vec<Instruction>> {
    
    if compilation_context.scoped_object.is_none() {
        if let Some(class) = compilation_context.class_loader.load(name) {
            let class_descriptor = class.path().replace('.', "/");
            let class_id = wrap(compilation_context.constant_pool.add_class(&class_descriptor))?;
            
            compilation_context.push_scoped_object(&class, class_id);
        }
    }
    Err(CompileError::UnknownClass(name.to_string()))
}