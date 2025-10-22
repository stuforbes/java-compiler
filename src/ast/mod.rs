use crate::ast::class::{AstClass, AstScope};
use crate::ast::class_builder::{Build, ClassBuilder};
use crate::ast::class_state_machine_factory::ClassState;
use crate::ast::method_builder::AstStatementBuilder;
use crate::scanner::{Token, TokenType};

pub mod class;
mod class_builder;
mod class_state_machine_factory;
pub mod expression;
mod method_builder;
mod state_machine;
pub mod statement;

struct AstParser<'src, 'token>
where
    'src: 'token,
{
    position: usize,
    source: &'src str,
    tokens: &'token [Token<'src>],
}

impl<'src, 'token> AstParser<'src, 'token> {
    fn for_tokens(source: &'src str, tokens: &'token [Token<'src>]) -> Self {
        Self {
            position: 0,
            source,
            tokens,
        }
    }

    fn next_token(&mut self) -> Token<'src> {
        let i = self.position;
        self.position = i + 1;
        self.tokens[i]
    }

    fn peek_next(&self) -> Token<'src> {
        self.tokens[self.position]
    }

    fn has_more_tokens(&self) -> bool {
        self.position < self.tokens.len()
    }

    fn position(&self) -> usize {
        self.position
    }

    fn lexemes_from_position(&self, from: usize, to: usize) -> Vec<&'src str> {

        self.tokens[from..=to]
            .iter()
            .filter(|t| t.token_type() != TokenType::Dot)
            .map(|t| t.lexeme())
            .collect()
    }
}

pub fn to_ast<'a>(source: &'a str, tokens: Vec<Token<'a>>) -> AstClass<'a> {
    let mut parser = AstParser::for_tokens(source, &tokens);
    let mut class_state_machine = class_state_machine_factory::load();
    let mut class_builder = ClassBuilder::new();

    while parser.has_more_tokens() {
        let token = parser.next_token();
        if let Some(state) = class_state_machine.on_token(token.token_type()) {
            match state {
                ClassState::Initial => {
                    // No op
                }
                ClassState::ClassScope => class_builder.with_scope(scope_for(token.token_type())),
                ClassState::ClassDefinition => {
                    // TODO: Figure out how to put class builder creation here
                }
                ClassState::ClassName => class_builder.named(token.lexeme()),
                ClassState::ClassBody => {
                    // No op
                }
                ClassState::MethodQualifier => {
                    class_builder.with_new_method();
                    class_builder
                        .latest_method()
                        .with_scope(scope_for(token.token_type()));
                }
                ClassState::MethodStatic => class_builder.latest_method().as_static(),
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
                    let mut statement_builder = AstStatementBuilder::new(&mut parser);
                    statement_builder.build();

                    class_builder
                        .latest_method()
                        .with_statements(statement_builder.statements());
                }
                ClassState::ClassEnd => {}
                ClassState::Eof => {}
            }
        }
    }

    class_builder.build()
}

fn scope_for(token_type: TokenType) -> AstScope {
    match token_type {
        TokenType::Public => AstScope::Public,
        _ => panic!("Unknown scope {:?}", token_type),
    }
}
