use ristretto_classfile::attributes::Instruction;
use crate::ast::expression::Expression;
use crate::compiler::CompileResult;

pub fn from_variable_assignment(_name: &str, _var_type: Option<&str>, _is_final: bool, _value: &Option<Expression>) -> CompileResult<Vec<Instruction>> {
    panic!("Not implemented")
}