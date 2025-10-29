use crate::ast::expression::Expression;
use crate::compiler::result::{CompileError, CompileResult};
use crate::compiler::{wrap, CompilationContext};
use ristretto_classfile::attributes::Instruction;
use crate::java::ClassLoader;

pub fn from_expression(
    expression: &Expression,
    compilation_context: &mut CompilationContext,
) -> CompileResult<Vec<Instruction>> {
    match expression {
        Expression::Call {
            object_path,
            method_name,
            arguments,
        } => from_call_expression(object_path, method_name, arguments, compilation_context),
        Expression::StringLiteral { value } => from_string_literal(value, compilation_context),
    }
}

fn from_call_expression(
    object_path: &[&str],
    method_name: &str,
    arguments: &Vec<Expression>,
    compilation_context: &mut CompilationContext,
) -> CompileResult<Vec<Instruction>> {
    if let Some((class_path, suffix)) =
        parse_object_path(object_path, &mut compilation_context.class_loader)
    {
        let class_descriptor = class_path.replace('.', "/");
        let class_id = wrap(
            compilation_context
                .constant_pool
                .add_class(&class_descriptor),
        )?;

        if suffix.is_empty() {
            todo!("Static methods on classes not yet supported")
        } else if suffix.len() == 1 {
            return from_static_field_on_class(
                class_descriptor.replace('/', ".").as_str(),
                class_id,
                suffix[0],
                method_name,
                arguments,
                compilation_context,
            );
        } else {
            todo!("Multiple nested static fields not yet supported")
        }
    } else {
        Err(CompileError::UnknownClass(object_path.join(".")))
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

    let field_class_name = {
        let field = compilation_context
            .class_loader
            .load(class)
            .and_then(|c| c.field_named(field_name));

        let field = field.ok_or_else(|| CompileError::UnknownField {
            class: class.to_string(),
            field: field_name.to_string(),
        })?;

        field.class().to_string() // clone into an owned String
    };

    let field_class_name_and_descriptor = {
        let field_class_option = compilation_context.class_loader.load(field_class_name.as_str());
        let field_class = field_class_option.ok_or_else(|| CompileError::UnknownClass(field_class_name.clone()) )?;
        (field_class.qualified_name().to_string(), field_class.descriptor().to_string())
    };

    let field_class_id = wrap(
        compilation_context
            .constant_pool
            .add_class(&field_class_name_and_descriptor.0),
    )?;
    let field_ref = add_field_ref(field_name, &field_class_name_and_descriptor.1, class_id, compilation_context)?;
    instructions.push(Instruction::Getstatic(field_ref));

    let method_descriptor = {
        let method = compilation_context.class_loader.load(field_class_name.as_str())
            .and_then(|field_class| field_class.method_named(method_name))
            .ok_or_else(|| CompileError::UnknownMethod { class: field_class_name_and_descriptor.0.to_string(), method: method_name.to_string() })?;

            method.descriptor()
    };

    let method_id = wrap(compilation_context.constant_pool.add_method_ref(
        field_class_id,
        method_name,
        method_descriptor,
    ))?;


    for argument in arguments {
        let argument_instructions = from_expression(argument, compilation_context)?;
        for argument_instruction in argument_instructions {
            instructions.push(argument_instruction);
        }
    }

    instructions.push(Instruction::Invokevirtual(method_id));
    instructions.push(Instruction::Return);

    Ok(instructions)

    // if let Some(field) = compilation_context
    //     .packages
    //     .class_for(class)
    //     .and_then(|c| c.field_named(field_name))
    // {
    //     if let Some(field_class) = compilation_context.packages.class_for(field.class()) {
    //         let field_class_id = wrap(
    //             compilation_context
    //                 .constant_pool
    //                 .add_class(field_class.name()),
    //         )?;
    //         let field_ref = add_field_ref(field, &field_class, class_id, compilation_context)?;
    //         instructions.push(Instruction::Getstatic(field_ref));
    //
    //         if let Some(method) = field_class.method_named(method_name) {
    //             let method_id = wrap(compilation_context.constant_pool.add_method_ref(
    //                 field_class_id,
    //                 method_name,
    //                 method.descriptor(),
    //             ))?;
    //
    //             for argument in arguments {
    //                 let argument_instructions = from_expression(argument, compilation_context)?;
    //                 for argument_instruction in argument_instructions {
    //                     instructions.push(argument_instruction);
    //                 }
    //             }
    //
    //             instructions.push(Instruction::Invokevirtual(method_id));
    //             instructions.push(Instruction::Return);
    //         } else {
    //             return Err(CompileError::UnknownMethod {
    //                 class: field_class.name().to_string(),
    //                 method: method_name.to_string(),
    //             });
    //         }
    //
    //         Ok(instructions)
    //     } else {
    //         Err(CompileError::UnknownClass(field.class().to_string()))
    //     }
    // } else {
    //     Err(CompileError::UnknownField {
    //         class: class.to_string(),
    //         field: field_name.to_string(),
    //     })
    // }
}

fn from_string_literal(
    value: &str,
    compilation_context: &mut CompilationContext,
) -> CompileResult<Vec<Instruction>> {
    let index = wrap(compilation_context.constant_pool.add_string(value))?;

    Ok(vec![Instruction::Ldc_w(index)])
}

fn add_field_ref(
    field_name: &str,
    field_class_descriptor: &str,
    class_ref: u16,
    compilation_context: &mut CompilationContext,
) -> CompileResult<u16> {
    let field_ref = wrap(compilation_context.constant_pool.add_field_ref(
        class_ref,
        field_name,
        field_class_descriptor,
    ))?;

    Ok(field_ref)
}

fn parse_object_path<'a>(
    path: &'a [&'a str],
    class_loader: &mut ClassLoader
) -> Option<(&'a str, &'a [&'a str])> {
    for i in (1..=path.len()).rev() {
        let (prefix, suffix) = path.split_at(i);

        if let Some(class) = class_loader.load(prefix.join(".").as_str()) {
            return Some((class.qualified_name(), suffix));
        }
    }
    None
}
