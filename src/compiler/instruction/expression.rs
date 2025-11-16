use crate::ast::expression::Expression;
use crate::compiler::instruction::call::from_call_expression;
use crate::compiler::instruction::string_literal::from_string_literal;
use crate::compiler::result::CompileResult;
use crate::compiler::CompilationContext;
use ristretto_classfile::attributes::Instruction;
use crate::compiler::instruction::object_expression::from_object_expression;
use crate::compiler::instruction::static_identifier::from_static_identifier;

pub fn from_expression(
    expression: &Expression,
    compilation_context: &mut CompilationContext,
) -> CompileResult<Vec<Instruction>> {

    // e.g. objectexpression {
    //          parent = staticidentifier { System },
    //          child = objectexpression {
    //              parent = staticidentifier { out },
    //              child = call {
    //                  methodname = "println"
    //                  args = ...
    //

    // Need to maintain current object depth

    match expression {
        Expression::Call {
            method_name,
            arguments,
        } => from_call_expression(method_name, arguments, compilation_context),
        Expression::StringLiteral { value } => from_string_literal(value, compilation_context),
        Expression::StaticIdentifier { name } => from_static_identifier(name, compilation_context),
        Expression::ChildIdentifier { .. } => todo!("Not supported"),
        Expression::ObjectExpression { parent, child } => from_object_expression(parent, child, compilation_context),
        Expression::Variable { .. } => todo!("Not supported"),
        Expression::Assignment { .. } => todo!("Not supported"),
    }
}

fn unbox<T>(value: &Box<T>) -> &T {
    &**value
}

