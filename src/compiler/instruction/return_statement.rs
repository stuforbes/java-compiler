use ristretto_classfile::attributes::Instruction;
use crate::ast::expression::Expression;
use crate::compiler::{CompilationContext, EmptyCompileResult};
use crate::compiler::instruction::expression::from_expression;
use crate::compiler::result::EMPTY_OK;

pub fn from_return(value: &Option<Expression>, compilation_context: &mut CompilationContext, instructions: &mut Vec<Instruction>) -> EmptyCompileResult {
    if let Some(expr) = value { 
        from_expression(expr, compilation_context, instructions)?;
        instructions.push(Instruction::Areturn);
    } else {
        instructions.push(Instruction::Return);
    }
    EMPTY_OK
}