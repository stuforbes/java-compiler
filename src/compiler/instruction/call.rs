use crate::ast::expression::Expression;
use crate::compiler::instruction::expression::from_expression;
use crate::compiler::{wrap, CompilationContext, CompileError, CompileResult};
use crate::java::class::JavaClass;
use crate::java::ClassLoader;
use ristretto_classfile::attributes::Instruction;

pub fn from_call_expression(
    method_name: &str,
    arguments: &Vec<Expression>,
    compilation_context: &mut CompilationContext,
) -> CompileResult<Vec<Instruction>> {
    todo!("Needs re-implementing now that call doesn't take object path")
    // let mut object_path = String::new();
    // extract_object_path(target, &mut object_path);
    //
    // if let Some((class_path, suffix)) = parse_object_path(object_path.as_str(), &mut compilation_context.class_loader) {
    //     let class_descriptor = class_path.replace('.', "/");
    //     let class_id = wrap(compilation_context.constant_pool.add_class(&class_descriptor))?;
    //
    //     if suffix.is_empty() {
    //         todo!("Static methods on classes not yet supported")
    //     } else if suffix.len() == 1 {
    //         return from_static_field_on_class(class_path, class_id, suffix.first().unwrap(), method_name, arguments, compilation_context);
    //     } else {
    //         todo!("Multiple nested static fields not yet supported")
    //     }
    // } else {
    //     Err(CompileError::UnknownClass(object_path))
    // }
}

// fn extract_object_path(expression: &Expression, result: &mut String) {
//     match expression {
//         Expression::Call { .. } => todo!(),
//         Expression::StringLiteral { .. } => todo!("Not yet supported"),
//         Expression::Variable { name, type_def: _type_def } => result.push_str(name),
//         Expression::ChildIdentifier { parent, name } => {
//             extract_object_path(parent, result);
//             result.push('.');
//             result.push_str(name);
//         }
//         Expression::Assignment { .. } => panic!("Cannot convert assignment to path. Perhaps assignment should be a statement?")
//     }
// }

fn extract_method_name<'a>(expression: &'a Expression) -> &'a str {
    match expression {
        Expression::ChildIdentifier { name, parent: _parent } => name,
        _expr => panic!("Unsupported")
    }
}

fn from_static_field_on_class(
    class: &str,
    class_id: u16,
    field_name: &str,
    method_name: &str,
    arguments: &Vec<Expression>,
    compilation_context: &mut CompilationContext,
) -> CompileResult<Vec<Instruction>> {
    let mut instructions: Vec<Instruction> = vec![];

    let (field_class_path, field_class_name, field_class_descriptor) = lookup_field_on_class(class, field_name, compilation_context)?;

    let field_class_id = wrap(compilation_context.constant_pool.add_class(&field_class_name))?;
    let field_ref = add_field_ref(field_name, &field_class_descriptor, class_id, compilation_context)?;
    instructions.push(Instruction::Getstatic(field_ref));

    let method_descriptor = lookup_method_descriptor(field_class_path.as_str(), method_name, compilation_context)?;
    let method_ref = wrap(
        compilation_context
            .constant_pool
            .add_method_ref(field_class_id, method_name, method_descriptor.as_str()),
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

fn add_field_ref(field_name: &str, field_class_descriptor: &str, class_ref: u16, compilation_context: &mut CompilationContext) -> CompileResult<u16> {
    let field_ref = wrap(
        compilation_context
            .constant_pool
            .add_field_ref(class_ref, field_name, field_class_descriptor),
    )?;

    Ok(field_ref)
}

fn parse_object_path<'a>(path: &'a str, class_loader: &mut ClassLoader) -> Option<(&'a str, Vec<&'a str>)> {
    let mut end = path.len();

    let mut split_idx = path[0..end].rfind('.');

    while split_idx.is_some() {

        let (prefix, suffix) = path.split_at(split_idx?);

        if let Some(class) = class_loader.load(prefix) {
            let suffix_parts: Vec<&str> = suffix[1..].split('.').collect();
            return Some((class.path(), suffix_parts));
        }

        end = split_idx?;
        split_idx = path[0..end].rfind('.');
    }

    None
}

fn lookup_field_on_class(
    class_name: &str,
    field_name: &str,
    compilation_context: &mut CompilationContext,
) -> CompileResult<(String, String, String)> {
    let field_class_path = {
        let class = lookup_class(class_name, compilation_context)?;
        let field = class.field_named(field_name).ok_or_else(|| CompileError::UnknownField {
            class: class_name.to_string(),
            field: field_name.to_string(),
        })?;
        field.class().to_string()
    };

    let field_class = lookup_class(field_class_path.as_str(), compilation_context)?;

    Ok((
        field_class.path().to_string(),
        field_class.full_name(),
        field_class.descriptor().to_string(),
    ))
}

fn lookup_class<'a>(class_name: &str, compilation_context: &'a mut CompilationContext) -> CompileResult<&'a JavaClass> {
    compilation_context
        .class_loader
        .load(class_name)
        .ok_or_else(|| CompileError::UnknownClass(class_name.to_string()))
}
