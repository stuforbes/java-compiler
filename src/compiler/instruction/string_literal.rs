use ristretto_classfile::attributes::Instruction;
use crate::compiler::{wrap, CompilationContext, CompileResult};

pub fn from_string_literal(
    value: &str,
    compilation_context: &mut CompilationContext,
    instructions: &mut Vec<Instruction>
) -> CompileResult<()> {
    let index = wrap(compilation_context.constant_pool.add_string(value))?;

    instructions.push(Instruction::Ldc_w(index));
    Ok(())
}
