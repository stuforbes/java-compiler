use crate::ast::state_machine::{Operation, StateMachine};
use crate::scanner::TokenType;
use std::collections::HashMap;

#[derive(Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Copy, Clone)]
pub enum MethodState {
    Initial,
    ClassScope,
}

pub fn load() -> StateMachine<MethodState> {
    let state_contexts: HashMap<(MethodState, TokenType), Operation<MethodState>> = vec![]
        .into_iter()
        .collect();

    StateMachine::new(MethodState::Initial, state_contexts)
}
