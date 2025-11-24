use crate::compiler::{wrap, CompilationContext, CompileError, CompileResult};
use crate::java::class::JavaClass;
use ristretto_classfile::attributes::Instruction;

pub fn from_static_identifier(name: &str, compilation_context: &mut CompilationContext, instructions: &mut Vec<Instruction>) -> CompileResult<()> {
    if compilation_context.scoped_object.is_none() {
        if compilation_context.stack.contains(name) {
            instructions.push(Instruction::Aload(compilation_context.stack.get(name)))
        }
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
    } else {
        let scoped_class_id = compilation_context.scoped_class_id();
        let (path, full_name, descriptor) = lookup_field_on_class(name, compilation_context)?;
        let field_class_id = wrap(compilation_context.constant_pool.add_class(&full_name))?;
        let field_ref = add_field_ref(name, &descriptor, scoped_class_id.unwrap(), compilation_context)?;
        instructions.push(Instruction::Getstatic(field_ref));

        compilation_context.push_scoped_object(path, field_class_id);
    }
    Ok(())
}

fn lookup_field_on_class(field_name: &str, compilation_context: &mut CompilationContext) -> CompileResult<(String, String, String)> {
    let scoped_class_path = compilation_context.scoped_class_path().unwrap();
    let field_class_path = {
        let class = lookup_class(scoped_class_path.as_str(), compilation_context)?;
        let field = class.field_named(field_name).ok_or_else(|| CompileError::UnknownField {
            class: scoped_class_path.to_string(),
            field: field_name.to_string(),
        })?;
        field.class().to_string()
    };

    let field_class = lookup_class(field_class_path.as_str(), compilation_context)?;

    Ok((field_class_path, field_class.full_name(), field_class.descriptor().to_string()))
}

fn lookup_class<'a>(class_name: &str, compilation_context: &'a mut CompilationContext) -> CompileResult<&'a JavaClass> {
    compilation_context
        .class_loader
        .load(class_name)
        .ok_or_else(|| CompileError::UnknownClass(class_name.to_string()))
}

fn add_field_ref(field_name: &str, field_class_descriptor: &str, class_ref: u16, compilation_context: &mut CompilationContext) -> CompileResult<u16> {
    let field_ref = wrap(
        compilation_context
            .constant_pool
            .add_field_ref(class_ref, field_name, field_class_descriptor),
    )?;

    Ok(field_ref)
}
