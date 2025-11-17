use crate::ast::expression::Expression;
use crate::compiler::instruction::expression::from_expression;
use crate::compiler::{wrap, CompilationContext, CompileError, CompileResult};
use ristretto_classfile::attributes::Instruction;

pub fn from_call_expression(
    method_name: &str,
    arguments: &Vec<Expression>,
    compilation_context: &mut CompilationContext,
) -> CompileResult<Vec<Instruction>> {

    let mut instructions: Vec<Instruction> = vec![];

    let (class_path, class_id) = if compilation_context.scoped_object.is_some() {
        (compilation_context.scoped_class_path().unwrap(), compilation_context.scoped_class_id().unwrap())
    } else {
        panic!("Cannot do local method refs yet");
    };


    let method_descriptor = lookup_method_descriptor(class_path.as_str(), method_name, compilation_context)?;
    let method_ref = wrap(
        compilation_context
            .constant_pool
            .add_method_ref(class_id, method_name, method_descriptor.as_str()),
    )?;

    for argument in arguments {
        let argument_instructions = from_expression(argument, compilation_context)?;
        for argument_instruction in argument_instructions {
            instructions.push(argument_instruction);
        }
    }

    instructions.push(Instruction::Invokevirtual(method_ref));
    instructions.push(Instruction::Return);

    Ok(instructions)
}

fn lookup_method_descriptor(class_path: &str, method_name: &str, compilation_context: &mut CompilationContext) -> Result<String, CompileError> {
    let method = compilation_context
        .class_loader
        .load(class_path)
        .and_then(|field_class| field_class.method_named(method_name))
        .ok_or_else(|| CompileError::UnknownMethod {
            class: class_path.to_string(),
            method: method_name.to_string(),
        })?;

    Ok(method.descriptor().to_string())
}