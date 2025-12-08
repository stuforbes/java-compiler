use crate::ast::class::{AstClass, AstScope};
use crate::ast::class_builder::{Build, ClassBuilder};
use crate::ast::class_state_machine_factory::ClassState;
use crate::ast::method_builder::AstStatementBuilder;
use crate::ast::parser::AstParser;
use crate::scanner::{Token, TokenType};

pub mod class;
mod class_builder;
mod class_state_machine_factory;
pub mod expression;
mod method_builder;
mod state_machine;
pub mod statement;
mod statement_structure;
mod parser;
mod expression_structure;

pub fn to_ast<'a>(tokens: Vec<Token<'a>>) -> AstClass<'a> {
    let mut parser = AstParser::for_tokens(tokens);
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
                ClassState::MethodReturnArrayIndicatorStart => {
                    // No op
                }
                ClassState::MethodReturnArrayIndicatorEnd => {
                    let method = class_builder.latest_method();
                    method.return_type_is_array(true);
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
        TokenType::Private => AstScope::Private,
        _ => panic!("Unknown scope {:?}", token_type),
    }
}
