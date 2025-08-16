use crate::ast::state_machine::State::MethodBody;
use crate::ast::state_machine::StateMachine;
use crate::scanner::{Token, TokenType};

mod state_machine;
mod class_parser;

struct AstParser<'a> {
    position: usize,
    tokens: Vec<Token<'a>>
}

impl <'a> AstParser<'a> {
    fn for_tokens(tokens: Vec<Token<'a>>) -> Self {
        Self {
            position: 0,
            tokens,
        }
    }

    fn next_token(&mut self) -> Token<'a> {
        let i = self.position;
        self.position = i + 1;
        self.tokens[i]
    }

    fn peek_next(&self) -> Token<'a> {
        self.tokens[self.position]
    }

    fn has_more_tokens(&self) -> bool {
        self.position < self.tokens.len()
    }
}

pub fn to_ast<'a>(tokens: Vec<Token<'a>>) {
    let mut parser = AstParser::for_tokens(tokens);
    let mut state_machine = StateMachine::new();

    while parser.has_more_tokens() {
        let token = parser.next_token();
        if let Some(state) = state_machine.on_token(token.token_type()) {
            println!("State: {:?}", state);
            if state == MethodBody {
                parse_body(&mut parser);
            }
        }
    }
}

fn parse_body(parser: &mut AstParser) {
    while parser.has_more_tokens() && parser.peek_next().token_type() != TokenType::RightBrace {
        parser.next_token();
    }
}