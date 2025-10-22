use ristretto_classfile::attributes::Instruction;
use ristretto_classfile::ConstantPool;
use crate::ast::expression::Expression;
use crate::compiler::result::{CompileError, CompileResult};
use crate::compiler::wrap;
use crate::java::java;

pub fn from_expression(expression: &Expression, constant_pool: &mut ConstantPool) -> CompileResult<Instruction> {
    match expression {
        Expression::Call { object_path, method_name, arguments } => {
            from_call_expression(object_path, method_name, arguments, constant_pool)
        }
        Expression::StringLiteral { value } => {
            from_string_literal(value, constant_pool)
        }
    }
}

fn from_call_expression(object_path: &str, method_name: &str, arguments: &Vec<Expression>, constant_pool: &mut ConstantPool) -> CompileResult<Instruction> {
    if let Some((package, class)) = java().package_and_class_named(object_path) {
        let fully_qualified_class = format!("{:}/{:}", package.name(), class.name());
        wrap(constant_pool.add_class(&fully_qualified_class))?;
        todo!()

        // if let Some(field) = class.field_named(method_name) {
        //     todo!("need to get field descriptor")
        // } else {
        //     Err(CompileError::UnknownMethod { class: fully_qualified_class, method: method_name.to_string() } )
        // }
    } else {
        Err(CompileError::UnknownClass(object_path.to_string()))
    }
}

fn from_string_literal(value: &str, constant_pool: &mut ConstantPool) -> CompileResult<Instruction> {
    todo!()
}
