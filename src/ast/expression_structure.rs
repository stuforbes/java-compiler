use crate::ast::expression::Expression;
use crate::ast::parser::AstParser;
use crate::scanner::{Literal, Token, TokenType};
use std::ops::Not;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Clone, EnumIter)]
enum ExpressionStructure {
    ObjectExpression,
    Call,
    Identifier,
    StringLiteral,
}

impl ExpressionStructure {
    pub fn next_expression_from_parser<'src, 'ast>(parser: &mut AstParser<'src>) -> Option<Expression<'ast>>
    where
        'src: 'ast,
    {
        for expression_structure in ExpressionStructure::iter() {
            parser.start_transaction();
            let result = expression_structure.optionally_get_expression_from(parser);
            if result.is_some() {
                parser.commit();
                return result;
            } else {
                parser.rollback();
            }
        }
        None
    }

    fn next_expression_from_parser_within_range<'src, 'ast>(
        parser: &mut AstParser<'src>,
        within: Vec<ExpressionStructure>,
    ) -> Option<Expression<'ast>>
    where
        'src: 'ast,
    {
        for expression_structure in within {
            parser.start_transaction();
            let result = expression_structure.optionally_get_expression_from(parser);
            if result.is_some() {
                parser.commit();
                return result;
            } else {
                parser.rollback();
            }
        }
        None
    }

    pub fn optionally_get_expression_from<'src, 'ast>(&self, parser: &mut AstParser<'src>) -> Option<Expression<'ast>>
    where
        'src: 'ast,
    {
        match self {
            ExpressionStructure::ObjectExpression => {
                let parent = ExpressionStructure::next_expression_from_parser_within_range(
                    parser,
                    vec![ExpressionStructure::Identifier, ExpressionStructure::Call],
                )?;
                consume_next_token_if_type(TokenType::Dot, parser)?;

                let child = ExpressionStructure::next_expression_from_parser(parser)?;
                Some(Expression::new_object_expression(parent, child))
            }
            ExpressionStructure::Call => {
                let method_name = consume_next_token_if_type(TokenType::Identifier, parser)?;
                consume_next_token_if_type(TokenType::LeftParen, parser)?;

                let mut args = vec![];
                while parser.is_next_token(TokenType::RightParen).not() {
                    args.push(ExpressionStructure::next_expression_from_parser(parser)?);

                    if parser.is_next_token(TokenType::Comma) {
                        consume_next_token_if_type(TokenType::Comma, parser)?;
                    }
                }
                consume_next_token_if_type(TokenType::RightParen, parser)?;

                Some(Expression::new_call(method_name.lexeme(), args))
            }
            ExpressionStructure::Identifier => {
                let identifier = consume_next_token_if_type(TokenType::Identifier, parser)?;
                Some(Expression::new_static_identifier(identifier.lexeme()))
            }
            ExpressionStructure::StringLiteral => {
                let token = consume_next_token_if_type(TokenType::String, parser)?;
                Some(Expression::new_string_literal(string_value_from(token.literal())))
            }
        }
    }
}

pub fn find_next_expression<'src, 'ast>(parser: &mut AstParser<'src>) -> Option<Expression<'ast>>
where
    'src: 'ast,
{
    ExpressionStructure::next_expression_from_parser(parser)
}

pub fn consume_next_token_if_type<'src>(expected_token_type: TokenType, parser: &mut AstParser<'src>) -> Option<Token<'src>> {
    if parser.has_more_tokens() && parser.is_next_token(expected_token_type) {
        Some(parser.next_token())
    } else {
        None
    }
}

fn string_value_from<'ast>(literal: &Literal<'ast>) -> &'ast str {
    match literal {
        Literal::String(s) => s,
    }
}
