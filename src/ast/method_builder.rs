use crate::ast::AstParser;
use crate::ast::class::AstStatement;
use crate::ast::class_builder::MethodBuilder;
use crate::ast::method_state_machine_factory::MethodState;
use crate::ast::state_machine::StateMachine;

pub fn build_method_statements<'a>(method: &mut MethodBuilder, parser: &mut AstParser, state_machine: &mut StateMachine<MethodState>) -> Vec<AstStatement<'a>> {
vec![]
}