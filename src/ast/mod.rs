use crate::ast::class::Class;
use crate::ast::class_builder::{Build, ClassBuilder, Scope};
use crate::ast::state_machine::State::MethodBody;
use crate::ast::state_machine::{State, StateMachine};
use crate::scanner::{Token, TokenType};

mod state_machine;
mod class_builder;
mod class;

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

pub fn to_ast(tokens: Vec<Token>) -> Class {
    let mut parser = AstParser::for_tokens(tokens);
    let mut state_machine = StateMachine::new();
    let mut class_builder = ClassBuilder::new();

    while parser.has_more_tokens() {
        let token = parser.next_token();
        if let Some(state) = state_machine.on_token(token.token_type()) {
            match state {
                State::Initial => {
                    // No op
                }
                State::ClassScope => {
                    class_builder.with_scope(scope_for(token.token_type()))
                }
                State::ClassDefinition => {
                    // TODO: Figure out how to put class builder creation here
                }
                State::ClassName => {
                    class_builder.named(token.lexeme())
                }
                State::ClassBody => {
                    // No op
                }
                State::MethodQualifier => {
                    class_builder.with_new_method()
                }
                State::MethodReturn => {
                    let method = class_builder.latest_method();
                    method.with_return_type(token.lexeme())
                }
                State::MethodName => {
                    let method = class_builder.latest_method();
                    method.with_name(token.lexeme())
                }
                State::MethodParameters => {
                    // No op
                }
                State::MethodParameterType => {
                    let method = class_builder.latest_method();
                    method.with_new_parameter();
                    method.latest_parameter().with_type(token.lexeme())
                }
                State::MethodParameterName => {
                    let method = class_builder.latest_method();
                    method.latest_parameter().with_name(token.lexeme())
                }
                State::MethodParametersEnd => {
                    // No op
                }
                MethodBody => {}
                State::ClassEnd => {}
                State::Eof => {}
            }
            if state == MethodBody {
                parse_body(&mut parser);
            }
        }
    }

    class_builder.build()
}

fn parse_body(parser: &mut AstParser) {
    while parser.has_more_tokens() && parser.peek_next().token_type() != TokenType::RightBrace {
        parser.next_token();
    }
}

fn scope_for(token_type: TokenType) -> Scope {
    match token_type {
        TokenType::Public => Scope::Public,
        _ => panic!("Unknown scope {:?}", token_type)
    }
}