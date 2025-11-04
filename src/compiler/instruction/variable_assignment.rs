use ristretto_classfile::attributes::Instruction;
use crate::ast::expression::Expression;
use crate::compiler::CompileResult;

pub fn from_variable_assignment(name: &str, var_type: &str, is_final: bool, value: &Option<Expression>) -> CompileResult<Vec<Instruction>> {
    panic!("Not implemented")
}