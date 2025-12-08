use crate::ast::expression::Expression;
use crate::compiler::instruction::expression::from_expression;
use crate::compiler::{wrap, CompilationContext, CompileError, EmptyCompileResult};
use ristretto_classfile::attributes::Instruction;
use crate::compiler::result::EMPTY_OK;

pub fn from_call_expression(
    method_name: &str,
    arguments: &Vec<Expression>,
    compilation_context: &mut CompilationContext,
    instructions: &mut Vec<Instruction>
) -> EmptyCompileResult {
    if compilation_context.scoped_object.is_some() {
        from_call_expression_on_scoped_object(method_name, arguments, compilation_context, instructions)?
    } else {
        from_call_expression_on_current_class(method_name, arguments, compilation_context, instructions)?
    }
}

fn from_call_expression_on_scoped_object(method_name: &str, arguments: &Vec<Expression>, compilation_context: &mut CompilationContext, instructions: &mut Vec<Instruction>) -> Result<EmptyCompileResult, CompileError> {
    let class_path = compilation_context.scoped_class_path().unwrap();
    let class_id = compilation_context.scoped_class_id().unwrap();

    let method_descriptor = lookup_method_descriptor(class_path.as_str(), method_name, compilation_context)?;
    let method_ref = wrap(
        compilation_context
            .constant_pool
            .add_method_ref(class_id, method_name, method_descriptor.as_str()),
    )?;

    compilation_context.clear_scoped_object();
    for argument in arguments {
        from_expression(argument, compilation_context, instructions)?;
    }
    compilation_context.push_scoped_object(class_path, class_id);

    instructions.push(Instruction::Invokevirtual(method_ref));

    Ok(EMPTY_OK)
}

fn from_call_expression_on_current_class(method_name: &str, arguments: &Vec<Expression>, compilation_context: &mut CompilationContext, instructions: &mut Vec<Instruction>) -> Result<EmptyCompileResult, CompileError> {
    let class_path = compilation_context.this_class_path();
    let class_id = compilation_context.this_class_id();

    if !compilation_context.symbol_table.contains_method(&method_name.to_string()) {
        return Err(CompileError::UnknownMethod { class: class_path, method: method_name.to_string() })
    }

    let method = compilation_context.symbol_table.method_named(&method_name.to_string());

    let method_ref = wrap(
        compilation_context
            .constant_pool
            .add_method_ref(class_id, method_name, method.descriptor().as_str()),
    )?;

    for argument in arguments {
        from_expression(argument, compilation_context, instructions)?;
    }

    instructions.push(Instruction::Invokestatic(method_ref));

    Ok(EMPTY_OK)
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