use crate::ast::state_machine::{Operation, StateMachine};
use crate::scanner::TokenType;
use std::collections::HashMap;
use crate::ast::class_state_machine_factory::ClassState::MethodStatic;

#[derive(Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Copy, Clone)]
pub enum MethodState {
    Initial,
    Identifier,
    IdentifierFullyQualifiedSeparator,

    MethodInvocationParameters,
    MethodInvocationParameterIdentifier,
    MethodInvocationParameterIdentifierFullyQualifiedSeparator,

}

pub fn load() -> StateMachine<MethodState> {
    let state_contexts: HashMap<(MethodState, TokenType), Operation<MethodState>> = vec![
        ((MethodState::Initial, TokenType::Identifier), Operation::To(MethodState::Identifier)),
        ((MethodState::Identifier, TokenType::Dot), Operation::To(MethodState::IdentifierFullyQualifiedSeparator)),
        ((MethodState::IdentifierFullyQualifiedSeparator, TokenType::Identifier), Operation::To(MethodState::Identifier)),
        ((MethodState::Identifier, TokenType::LeftParen), Operation::To(MethodState::MethodInvocationParameters)),

        ((MethodState::MethodInvocationParameters, TokenType::Identifier), Operation::To(MethodState::MethodInvocationParameterIdentifier)),
        ((MethodState::MethodInvocationParameterIdentifier, TokenType::Dot), Operation::To(MethodState::MethodInvocationParameterIdentifierFullyQualifiedSeparator)),

    ]
        .into_iter()
        .collect();

    StateMachine::new(MethodState::Initial, state_contexts)
}
