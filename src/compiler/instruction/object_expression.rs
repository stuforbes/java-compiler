use crate::ast::expression::Expression;
use crate::compiler::instruction::expression::from_expression;
use crate::compiler::{CompilationContext, CompileResult};
use ristretto_classfile::attributes::Instruction;

pub fn from_object_expression(
    parent: &Expression,
    child: &Expression,
    compilation_context: &mut CompilationContext,
) -> CompileResult<Vec<Instruction>> {
    let mut instructions = vec![];

    let parent_instructions = from_expression(parent, compilation_context)?;
    for instruction in parent_instructions {
        instructions.push(instruction);
    }

    let child_instructions = from_expression(child, compilation_context)?;
    for instruction in child_instructions {
        instructions.push(instruction);
    }

    Ok(instructions)
}