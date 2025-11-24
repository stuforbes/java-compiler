use ristretto_classfile::attributes::Instruction;
use crate::ast::expression::Expression;
use crate::compiler::{CompilationContext, CompileResult};
use crate::compiler::instruction::expression::from_expression;

pub fn from_variable_assignment(name: &str, _var_type: Option<&str>, _is_final: bool, value: &Option<Expression>, compilation_context: &mut CompilationContext) -> CompileResult<Vec<Instruction>> {

    if value.is_none() {
        panic!("Unable to handle uninitialised variables");
    }

    let mut result= from_expression(&value.as_ref().unwrap(), compilation_context)?;

    let id = compilation_context.stack.push(name);
    result.push(Instruction::Astore(id));

    Ok(result)
}