use crate::ast::class::{AstClass, AstScope};
use crate::ast::class_builder::{Build, ClassBuilder};
use crate::ast::class_state_machine_factory::ClassState;
use crate::ast::method_builder::build_method_statements;
use crate::scanner::{Token, TokenType};

mod state_machine;
mod class_builder;
pub mod class;
mod class_state_machine_factory;
mod method_builder;

struct AstParser<'a> {
    position: usize,
    source: &'a str,
    tokens: Vec<Token<'a>>
}

impl <'a> AstParser<'a> {
    fn for_tokens(source: &'a str, tokens: Vec<Token<'a>>) -> Self {
        Self {
            position: 0,
            source,
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
    
    fn position(&self) -> usize {
        self.position
    }
    
    fn lexemes_from_position(&self, from: usize, to: usize) -> &str {
        let start_token = self.tokens[from];
        let end_token = self.tokens[to];
        
        &self.source[start_token.start()..end_token.end()]
    }
}

pub fn to_ast<'a>(source: &'a str, tokens: Vec<Token<'a>>) -> AstClass<'a> {
    let mut parser = AstParser::for_tokens(source, tokens);
    let mut class_state_machine = class_state_machine_factory::load();
    let mut class_builder = ClassBuilder::new();

    while parser.has_more_tokens() {
        let token = parser.next_token();
        if let Some(state) = class_state_machine.on_token(token.token_type()) {
            match state {
                ClassState::Initial => {
                    // No op
                }
                ClassState::ClassScope => {
                    class_builder.with_scope(scope_for(token.token_type()))
                }
                ClassState::ClassDefinition => {
                    // TODO: Figure out how to put class builder creation here
                }
                ClassState::ClassName => {
                    class_builder.named(token.lexeme())
                }
                ClassState::ClassBody => {
                    // No op
                }
                ClassState::MethodQualifier => {
                    class_builder.with_new_method();
                    class_builder.latest_method().with_scope(scope_for(token.token_type()));
                }
                ClassState::MethodStatic => {
                    class_builder.latest_method().as_static()
                }
                ClassState::MethodReturn => {
                    let method = class_builder.latest_method();
                    method.with_return_type(token.lexeme())
                }
                ClassState::MethodName => {
                    let method = class_builder.latest_method();
                    method.with_name(token.lexeme())
                }
                ClassState::MethodParameters => {
                    // No op
                }
                ClassState::MethodParameterType => {
                    let method = class_builder.latest_method();
                    method.with_new_parameter();
                    method.latest_parameter().with_type(token.lexeme())
                }
                ClassState::MethodParameterArrayIndicatorStart => {
                    // No op
                }
                ClassState::MethodParameterArrayIndicatorEnd => {
                    let method = class_builder.latest_method();
                    method.latest_parameter().as_array();
                }
                ClassState::MethodParameterName => {
                    let method = class_builder.latest_method();
                    method.latest_parameter().with_name(token.lexeme())
                }
                ClassState::MethodParametersEnd => {
                    // No op
                }
                ClassState::MethodBody => {
                    let mut method = class_builder.latest_method();
                    build_method_statements(&mut method, &mut parser);
                }
                ClassState::ClassEnd => {}
                ClassState::Eof => {}
            }
            if state == ClassState::MethodBody {
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

fn scope_for(token_type: TokenType) -> AstScope {
    match token_type {
        TokenType::Public => AstScope::Public,
        _ => panic!("Unknown scope {:?}", token_type)
    }
}