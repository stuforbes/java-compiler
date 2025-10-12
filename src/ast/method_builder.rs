use crate::ast::class::AstStatement;
use crate::ast::class_builder::MethodBuilder;
use crate::ast::AstParser;
use crate::scanner::{Token, TokenType};

pub fn build_method_statements<'a>(
    method: &mut MethodBuilder,
    parser: &mut AstParser,
) -> Vec<AstStatement<'a>> {
    let mut statements: Vec<AstStatement<'a>> = vec![];

    while parser.peek_next().token_type() != TokenType::RightBrace {
        next_statement(method, parser, &mut statements)
    }

    statements
}

fn next_statement(
    method: &mut MethodBuilder,
    parser: &mut AstParser,
    statements: &mut Vec<AstStatement>,
) {
    let next_token = parser.peek_next();
    match next_token.token_type() {
        TokenType::Class => panic!("Unexpected token {:?}", next_token),
        TokenType::Public => panic!("Unexpected token {:?}", next_token),
        TokenType::Static => panic!("Unexpected token {:?}", next_token),
        TokenType::LeftParen => todo!(),
        TokenType::RightParen => todo!(),
        TokenType::LeftBrace => todo!(),
        TokenType::RightBrace => todo!(),
        TokenType::LeftSquareBracket => todo!(),
        TokenType::RightSquareBracket => todo!(),
        TokenType::SemiColon => todo!(),
        TokenType::Dot => todo!(),
        TokenType::Comma => todo!(),
        TokenType::Identifier => todo!(),
        TokenType::String => todo!(),
        TokenType::Eof => panic!("Unexpected token {:?}", next_token),
    }
}

trait Statement {}

struct ExpressionStatement<'a> {
    expression: Box<dyn Expression + 'a>,
}
impl <'a> Statement for ExpressionStatement<'a> {}

trait Expression {}

struct CallExpression<'a> {
    object_path: &'a str,
    method_name: &'a str,
    arguments: Vec<Box<dyn Expression + 'a>>,
}
impl <'a> Expression for CallExpression<'a> {}

struct StringLiteral<'a> {
    value: &'a str,
}
impl <'a> Expression for StringLiteral<'a> {}

fn expression_statement<'a>(parser: &'a mut AstParser<'a>) -> Box<dyn Statement + 'a> {
    Box::new(ExpressionStatement {
        expression: expression(parser),
    })
}

fn expression<'a>(parser: &'a mut AstParser<'a>) -> Box<dyn Expression + 'a> {
    let next_token = parser.peek_next();
    if next_token.token_type() == TokenType::Identifier {
        return identifier_expression(parser);
    }
    if next_token.token_type() == TokenType::String {
        return string_literal(parser);
    }

    todo!()
}

fn identifier_expression<'a>(parser: &'a mut AstParser<'a>) -> Box<dyn Expression + 'a> {
    identifier_expression_for_fully_qualified_object(parser, parser.position())
}

fn identifier_expression_for_fully_qualified_object<'a>(
    parser: &'a mut AstParser<'a>,
    object_path_start: usize,
) -> Box<dyn Expression + 'a> {
    let start_token = consume(parser, TokenType::Identifier);

    let next = parser.peek_next();
    if next.token_type() == TokenType::LeftParen {
        call_expression(
            parser,
            start_token.lexeme(),
            object_path_start,
            start_token.start(),
        )
    } else if next.token_type() == TokenType::Dot {
        consume(parser, TokenType::Dot);
        identifier_expression_for_fully_qualified_object(parser, object_path_start)
    } else {
        panic!("Unexpected token {:?}", next.token_type())
    }
}

fn call_expression<'a>(
    parser: &'a mut AstParser<'a>,
    method_name: &'a str,
    object_start_position: usize,
    object_end_position: usize,
) -> Box<dyn Expression + 'a> {
    let mut arguments: Vec<Box<dyn Expression + 'a>> = vec![];
    let next_token_type = parser.peek_next().token_type()
    while next_token_type != TokenType::RightParen {
        let arg = {
            expression(parser)
        };
        arguments.push(arg);
    }

    Box::new(CallExpression {
        object_path: parser.lexemes_from_position(object_start_position, object_end_position),
        method_name,
        arguments,
    })
}

fn string_literal<'a>(parser: &'a mut AstParser<'a>) -> Box<dyn Expression + 'a> {
    let token = consume(parser, TokenType::String);
    Box::new(StringLiteral { value: token.lexeme() })
}

fn consume<'a>(parser: &mut AstParser<'a>, expected_type: TokenType) -> Token<'a> {
    if parser.peek_next().token_type() == expected_type {
        parser.next_token()
    } else {
        panic!(
            "Expected {:?}, but was {:?}",
            expected_type,
            parser.peek_next().token_type()
        )
    }
}
