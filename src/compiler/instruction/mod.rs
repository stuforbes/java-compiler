mod expression;

use ristretto_classfile::attributes::Instruction;
use ristretto_classfile::ConstantPool;
use crate::ast::statement::Statement;
use crate::compiler::CompileResult;
use crate::compiler::instruction::expression::from_expression;

pub fn from(statement: &Statement, constant_pool: &mut ConstantPool) -> CompileResult<Instruction> {
    match statement { Statement::Expression { expression } => from_expression(expression, constant_pool) }
}
