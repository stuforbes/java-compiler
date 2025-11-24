use crate::ast::expression::Expression;
use crate::compiler::instruction::object_expression::from_object_expression;
use crate::compiler::instruction::static_identifier::from_static_identifier;
use crate::compiler::instruction::string_literal::from_string_literal;
use crate::compiler::{CompilationContext, EmptyCompileResult};
use ristretto_classfile::attributes::Instruction;
use crate::compiler::instruction::call::from_call_expression;

pub fn from_expression(expression: &Expression, compilation_context: &mut CompilationContext, instructions: &mut Vec<Instruction>) -> EmptyCompileResult {
    match expression {
        Expression::Call { method_name, arguments } => from_call_expression(method_name, arguments, compilation_context, instructions),
        Expression::StringLiteral { value } => from_string_literal(value, compilation_context, instructions),
        Expression::StaticIdentifier { name } => from_static_identifier(name, compilation_context, instructions),
        Expression::ChildIdentifier { .. } => todo!("Not supported"),
        Expression::ObjectExpression { parent, child } => from_object_expression(parent, child, compilation_context, instructions),
        Expression::Variable { .. } => todo!("Not supported"),
        Expression::Assignment { .. } => todo!("Not supported"),
    }
}