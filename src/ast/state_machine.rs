use crate::ast::state_machine::StateMachineError::NoTransitionAvailable;
use crate::scanner::TokenType;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

#[derive(Copy, Clone)]
pub enum Operation<State> {
    Ignore,
    To(State),
}

#[derive(Debug)]
#[allow(dead_code)]
enum StateMachineError<State> {
    NoTransitionAvailable(State, TokenType),
}

pub struct StateMachine<State> {
    current_state: State,
    state_contexts: HashMap<(State, TokenType), Operation<State>>,
}

impl <State> StateMachine<State>
where State: Eq + Hash + Copy + Clone + Debug {
    pub fn new(initial_state: State, state_contexts: HashMap<(State, TokenType), Operation<State>>) -> Self {
        Self {
            current_state: initial_state,
            state_contexts,
        }
    }

    pub fn on_token(&mut self, token_type: TokenType) -> Option<State> {
        match self.operation_for(self.current_state, token_type) {
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

    fn operation_for(
        &self,
        state: State,
        token_type: TokenType,
    ) -> Result<Operation<State>, StateMachineError<State>> {
        let key = &(state, token_type);
        if let Some(operation) = self.state_contexts.get(key) {
            Ok(*operation)
        } else {
            Err(NoTransitionAvailable(state, token_type))
        }
    }
}
