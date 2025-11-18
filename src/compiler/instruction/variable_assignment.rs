use ristretto_classfile::attributes::Instruction;
use crate::ast::expression::Expression;
use crate::compiler::{CompilationContext, CompileResult};
use crate::compiler::instruction::expression::from_expression;

pub fn from_variable_assignment(_name: &str, _var_type: Option<&str>, _is_final: bool, value: &Option<Expression>, compilation_context: &mut CompilationContext) -> CompileResult<Vec<Instruction>> {

    if value.is_none() {
        panic!("Unable to handle uninitialised variables");
    }

    let mut result= from_expression(&value.as_ref().unwrap(), compilation_context)?;
    result.push(Instruction::Astore(1));

    Ok(result)
}