mod expression;

use ristretto_classfile::attributes::Instruction;
use crate::ast::statement::Statement;
use crate::compiler::{CompilationContext, CompileResult};
use crate::compiler::instruction::expression::from_expression;

pub fn from(statement: &Statement, compilation_context: &mut CompilationContext) -> CompileResult<Vec<Instruction>> {
    match statement { Statement::Expression { expression } => from_expression(expression, compilation_context) }
}
