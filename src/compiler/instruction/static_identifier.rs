use crate::compiler::{wrap, CompilationContext, CompileResult, EmptyCompileResult};
use ristretto_classfile::attributes::Instruction;
use crate::compiler::resolver::ResolvedEntity::{StaticClass, StaticFieldReference, VariableOnStack};
use crate::compiler::result::EMPTY_OK;

pub fn from_static_identifier(name: &str, compilation_context: &mut CompilationContext, instructions: &mut Vec<Instruction>) -> EmptyCompileResult {
    let resolved = compilation_context.resolve(name)?;

    match resolved {
        VariableOnStack(stack_index) => {
            instructions.push(Instruction::Aload(stack_index))
        },
        StaticClass(class_path) => {
            let class_descriptor = class_path.replace('.', "/");
            let class_id = wrap(compilation_context.constant_pool.add_class(&class_descriptor))?;
            compilation_context.push_scoped_object(class_path, class_id)
        }
        StaticFieldReference { parent_id, path, full_name, descriptor } => {
            let field_class_id = wrap(compilation_context.constant_pool.add_class(&full_name))?;
            let field_ref = add_field_ref(name, &descriptor, parent_id, compilation_context)?;
            instructions.push(Instruction::Getstatic(field_ref));

            compilation_context.push_scoped_object(path, field_class_id);
        }
    }
    EMPTY_OK
}

fn add_field_ref(field_name: &str, field_class_descriptor: &str, class_ref: u16, compilation_context: &mut CompilationContext) -> CompileResult<u16> {
    let field_ref = wrap(
        compilation_context
            .constant_pool
            .add_field_ref(class_ref, field_name, field_class_descriptor),
    )?;

    Ok(field_ref)
}
