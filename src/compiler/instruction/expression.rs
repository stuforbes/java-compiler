use crate::ast::expression::Expression;
use crate::compiler::result::{CompileError, CompileResult};
use crate::compiler::wrap;
use crate::java::{java, JavaClass, JavaField};
use ristretto_classfile::attributes::Instruction;
use ristretto_classfile::ConstantPool;

pub fn from_expression(
    expression: &Expression,
    constant_pool: &mut ConstantPool,
) -> CompileResult<Vec<Instruction>> {
    match expression {
        Expression::Call {
            object_path,
            method_name,
            arguments,
        } => from_call_expression(object_path, method_name, arguments, constant_pool),
        Expression::StringLiteral { value } => from_string_literal(value, constant_pool),
    }
}

fn from_call_expression(
    object_path: &[&str],
    method_name: &str,
    arguments: &Vec<Expression>,
    constant_pool: &mut ConstantPool,
) -> CompileResult<Vec<Instruction>> {
    if let Some((package, class, suffix)) = java().parse_object_path(object_path) {
        let fully_qualified_class = format!("{:}/{:}", package.name(), class.name());
        let class_id = wrap(constant_pool.add_class(&fully_qualified_class))?;

        if suffix.is_empty() {
            todo!("Static methods on classes not yet supported")
        } else if suffix.len() == 1 {
            return from_static_field_on_static_field(object_path, class, class_id, suffix[0], method_name, arguments, constant_pool)
        } else {
            todo!("Multiple nested static fields not yet supported")
        }
    } else {
        Err(CompileError::UnknownClass(object_path.join(".")))
    }
}

fn from_static_field_on_static_field(object_path: &[&str], class: &JavaClass, class_id: u16, field_name: &str, method_name: &str, arguments: &Vec<Expression>, constant_pool: &mut ConstantPool) -> CompileResult<Vec<Instruction>> {
    let mut instructions: Vec<Instruction> = vec![];
    if let Some(field) = class.field_named(field_name) {
        if let Some(field_class) = java().class_for(field.class()) {
            let field_class_id = wrap(constant_pool.add_class(field_class.name()))?;
            let field_ref = add_field_ref(field, &field_class, class_id, constant_pool)?;
            instructions.push(Instruction::Getstatic(field_ref));

            if let Some(method) = field_class.method_named(method_name) {
                let method_id =
                    wrap(constant_pool.add_method_ref(field_class_id, method_name, method.descriptor()))?;

                for argument in arguments {
                    let argument_instructions = from_expression(argument, constant_pool)?;
                    for argument_instruction in argument_instructions {
                        instructions.push(argument_instruction);
                    }
                }

                instructions.push(Instruction::Invokevirtual(method_id));
                instructions.push(Instruction::Return);
            } else {
                return Err(CompileError::UnknownMethod {
                    class: field_class.name().to_string(),
                    method: method_name.to_string(),
                });
            }

            Ok(instructions)
        } else {
            Err(CompileError::UnknownClass(field.class().to_string()))
        }

    } else {
        Err(CompileError::UnknownField {
            class: class.name().to_string(),
            field: field_name.to_string(),
        })
    }
}

fn from_string_literal(
    value: &str,
    constant_pool: &mut ConstantPool,
) -> CompileResult<Vec<Instruction>> {
    todo!()
}

fn add_field_ref(
    field: &JavaField,
    field_class: &JavaClass,
    class_ref: u16,
    constant_pool: &mut ConstantPool,
) -> CompileResult<u16> {
    let field_ref =
        wrap(constant_pool.add_field_ref(class_ref, field.name(), field_class.descriptor()))?;

    Ok(field_ref)
}
