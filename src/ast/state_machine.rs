use std::collections::HashMap;
use lazy_static::lazy_static;
use crate::ast::state_machine::StateMachineError::NoTransitionAvailable;
use crate::scanner::TokenType;

#[derive(Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Copy, Clone)]
pub enum State {
    Initial,
    ClassScope,
    ClassDefinition,
    ClassName,
    ClassBody,

    MethodQualifier,
    MethodStatic,
    MethodReturn,
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

#[derive(Copy, Clone)]
enum Operation {
    Ignore,
    To(State)
}

lazy_static! {
    static ref STATE_CONTEXTS: HashMap<(State, TokenType), Operation> = vec![
        ((State::Initial, TokenType::Public), Operation::To(State::ClassScope)),
        ((State::ClassScope, TokenType::Class), Operation::To(State::ClassDefinition)),
        ((State::ClassDefinition, TokenType::Identifier), Operation::To(State::ClassName)),
        ((State::ClassName, TokenType::LeftBrace), Operation::To(State::ClassBody)),
        ((State::ClassBody, TokenType::Public), Operation::To(State::MethodQualifier)),

        ((State::MethodQualifier, TokenType::Static), Operation::To(State::MethodStatic)),
        ((State::MethodStatic, TokenType::Identifier), Operation::To(State::MethodReturn)),
        ((State::MethodQualifier, TokenType::Identifier), Operation::To(State::MethodReturn)),
        ((State::MethodReturn, TokenType::Identifier), Operation::To(State::MethodName)),
        ((State::MethodName, TokenType::LeftParen), Operation::To(State::MethodParameters)),
        ((State::MethodParameters, TokenType::RightParen), Operation::To(State::MethodParametersEnd)),
        ((State::MethodParameters, TokenType::Identifier), Operation::To(State::MethodParameterType)),
        ((State::MethodParameterType, TokenType::Identifier), Operation::To(State::MethodParameterName)),
        ((State::MethodParameterType, TokenType::LeftSquareBracket), Operation::To(State::MethodParameterArrayIndicatorStart)),
        ((State::MethodParameterArrayIndicatorStart, TokenType::RightSquareBracket), Operation::To(State::MethodParameterArrayIndicatorEnd)),
        ((State::MethodParameterArrayIndicatorEnd, TokenType::Identifier), Operation::To(State::MethodParameterName)),
        ((State::MethodParameterName, TokenType::Comma), Operation::To(State::MethodParameters)),
        ((State::MethodParameterName, TokenType::RightParen), Operation::To(State::MethodParametersEnd)),
        ((State::MethodParametersEnd, TokenType::LeftBrace), Operation::To(State::MethodBody)),
        ((State::MethodBody, TokenType::RightBrace), Operation::To(State::ClassBody)),

        ((State::ClassBody, TokenType::RightBrace), Operation::To(State::ClassEnd)),
        ((State::ClassEnd, TokenType::Eof), Operation::To(State::Eof))

    ].into_iter().collect();
}

pub struct StateMachine {
    current_state: State,
}

#[derive(Debug)]
#[allow(dead_code)]
enum StateMachineError {
    NoTransitionAvailable(State, TokenType)
}

impl StateMachine {
    pub fn new() -> Self {
        Self {
            current_state: State::Initial,
        }
    }

    pub fn on_token(&mut self, token_type: TokenType) -> Option<State> {
        match Self::operation_for(self.current_state, token_type) {
            Ok(Operation::Ignore) => {
                // nothing to do
                None
            }
            Ok(Operation::To(new_state)) => {
                self.current_state = new_state;
                Some(new_state)
            }
            Err(e) => {
                panic!("{:?}", e)
            }
        }
    }

    fn operation_for(state: State, token_type: TokenType) -> Result<Operation, StateMachineError> {
        let key = &(state, token_type);
        if let Some(operation) = STATE_CONTEXTS.get(key) {
           Ok(*operation)
        } else {
            Err(NoTransitionAvailable(state, token_type))
        }
    }
}

