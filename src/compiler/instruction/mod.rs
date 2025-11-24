mod expression;
mod call;
mod string_literal;
mod variable_assignment;
mod object_expression;
mod static_identifier;

use ristretto_classfile::attributes::Instruction;
use crate::ast::statement::Statement;
use crate::compiler::{CompilationContext, EmptyCompileResult};
use crate::compiler::instruction::expression::from_expression;
use crate::compiler::instruction::variable_assignment::from_variable_assignment;

pub fn from(statement: &Statement, compilation_context: &mut CompilationContext, instructions: &mut Vec<Instruction>) -> EmptyCompileResult {
    match statement {
        Statement::Expression { expression } => from_expression(expression, compilation_context, instructions),
        Statement::VariableAssignment { name, var_type, is_final, value } =>
            from_variable_assignment(name, *var_type, *is_final, value, compilation_context, instructions)
    }
}
