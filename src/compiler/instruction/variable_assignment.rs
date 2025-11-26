use crate::ast::expression::Expression;
use crate::compiler::instruction::expression::from_expression;
use crate::compiler::{CompilationContext, EmptyCompileResult};
use ristretto_classfile::attributes::Instruction;
use crate::compiler::result::EMPTY_OK;

pub fn from_variable_assignment(
    name: &str,
    _var_type: Option<&str>,
    _is_final: bool,
    value: &Option<Expression>,
    compilation_context: &mut CompilationContext,
    instructions: &mut Vec<Instruction>,
) -> EmptyCompileResult {
    if value.is_none() {
        panic!("Unable to handle uninitialised variables");
    }

    from_expression(&value.as_ref().unwrap(), compilation_context, instructions)?;

    let id = compilation_context.stack.push(name);
    instructions.push(Instruction::Astore(id));

    EMPTY_OK
}
