use crate::ast::expression::Expression;
use crate::compiler::instruction::expression::from_expression;
use crate::compiler::{CompilationContext, EmptyCompileResult};
use ristretto_classfile::attributes::Instruction;

pub fn from_object_expression(
    parent: &Expression,
    child: &Expression,
    compilation_context: &mut CompilationContext,
    instructions: &mut Vec<Instruction>
) -> EmptyCompileResult {
    from_expression(parent, compilation_context, instructions)?;

    from_expression(child, compilation_context, instructions)?;

    Ok(())
}