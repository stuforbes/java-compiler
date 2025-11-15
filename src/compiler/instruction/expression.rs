use crate::ast::expression::Expression;
use crate::compiler::instruction::call::from_call_expression;
use crate::compiler::instruction::string_literal::from_string_literal;
use crate::compiler::result::CompileResult;
use crate::compiler::CompilationContext;
use ristretto_classfile::attributes::Instruction;

pub fn from_expression(
    expression: &Expression,
    compilation_context: &mut CompilationContext,
) -> CompileResult<Vec<Instruction>> {
    match expression {
        Expression::Call {
            method_name,
            arguments,
        } => from_call_expression(method_name, arguments, compilation_context),
        Expression::StringLiteral { value } => from_string_literal(value, compilation_context),
        Expression::StaticIdentifier { name } => todo!("Not supported"),
        Expression::ChildIdentifier { .. } => todo!("Not supported"),
        Expression::ObjectExpression { .. } => todo!("Not supported"),
        Expression::Variable { .. } => todo!("Not supported"),
        Expression::Assignment { .. } => todo!("Not supported"),
    }
}

fn unbox<T>(value: &Box<T>) -> &T {
    &**value
}

