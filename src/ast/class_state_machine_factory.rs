use std::collections::HashMap;
use crate::ast::state_machine::{Operation, StateMachine};
use crate::scanner::TokenType;

#[derive(Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Copy, Clone)]
pub enum ClassState {
    Initial,
    ClassScope,
    ClassDefinition,
    ClassName,
    ClassBody,

    MethodQualifier,
    MethodStatic,
    MethodReturn,
    MethodReturnArrayIndicatorStart,
    MethodReturnArrayIndicatorEnd,
    MethodName,
    MethodParameters,
    MethodParameterType,
    MethodParameterArrayIndicatorStart,
    MethodParameterArrayIndicatorEnd,
    MethodParameterName,
    MethodParametersEnd,
    MethodBody,

    ClassEnd,
    Eof,
}

pub fn load() -> StateMachine<ClassState> {
    let state_contexts:  HashMap<(ClassState, TokenType), Operation<ClassState>> = vec![
        ((ClassState::Initial, TokenType::Public), Operation::To(ClassState::ClassScope)),
        ((ClassState::ClassScope, TokenType::Class), Operation::To(ClassState::ClassDefinition)),
        ((ClassState::ClassDefinition, TokenType::Identifier), Operation::To(ClassState::ClassName)),
        ((ClassState::ClassName, TokenType::LeftBrace), Operation::To(ClassState::ClassBody)),
        ((ClassState::ClassBody, TokenType::Public), Operation::To(ClassState::MethodQualifier)),
        ((ClassState::ClassBody, TokenType::Private), Operation::To(ClassState::MethodQualifier)),

        ((ClassState::MethodQualifier, TokenType::Static), Operation::To(ClassState::MethodStatic)),
        ((ClassState::MethodStatic, TokenType::Identifier), Operation::To(ClassState::MethodReturn)),
        ((ClassState::MethodQualifier, TokenType::Identifier), Operation::To(ClassState::MethodReturn)),
        ((ClassState::MethodReturn, TokenType::Identifier), Operation::To(ClassState::MethodName)),
        ((ClassState::MethodReturn, TokenType::LeftSquareBracket), Operation::To(ClassState::MethodReturnArrayIndicatorStart)),
        ((ClassState::MethodReturnArrayIndicatorStart, TokenType::RightSquareBracket), Operation::To(ClassState::MethodReturnArrayIndicatorEnd)),
        ((ClassState::MethodReturnArrayIndicatorEnd, TokenType::Identifier), Operation::To(ClassState::MethodName)),
        ((ClassState::MethodName, TokenType::LeftParen), Operation::To(ClassState::MethodParameters)),
        ((ClassState::MethodParameters, TokenType::RightParen), Operation::To(ClassState::MethodParametersEnd)),
        ((ClassState::MethodParameters, TokenType::Identifier), Operation::To(ClassState::MethodParameterType)),
        ((ClassState::MethodParameterType, TokenType::Identifier), Operation::To(ClassState::MethodParameterName)),
        ((ClassState::MethodParameterType, TokenType::LeftSquareBracket), Operation::To(ClassState::MethodParameterArrayIndicatorStart)),
        ((ClassState::MethodParameterArrayIndicatorStart, TokenType::RightSquareBracket), Operation::To(ClassState::MethodParameterArrayIndicatorEnd)),
        ((ClassState::MethodParameterArrayIndicatorEnd, TokenType::Identifier), Operation::To(ClassState::MethodParameterName)),
        ((ClassState::MethodParameterName, TokenType::Comma), Operation::To(ClassState::MethodParameters)),
        ((ClassState::MethodParameterName, TokenType::RightParen), Operation::To(ClassState::MethodParametersEnd)),
        ((ClassState::MethodParametersEnd, TokenType::LeftBrace), Operation::To(ClassState::MethodBody)),
        ((ClassState::MethodBody, TokenType::RightBrace), Operation::To(ClassState::ClassBody)),

        ((ClassState::ClassBody, TokenType::RightBrace), Operation::To(ClassState::ClassEnd)),
        ((ClassState::ClassEnd, TokenType::Eof), Operation::To(ClassState::Eof))
    ].into_iter().collect();

    StateMachine::new(ClassState::Initial, state_contexts)
}