use crate::ast::expression_structure::{consume_next_token_if_type, find_next_expression};
use crate::ast::statement::Statement;
use crate::ast::AstParser;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use crate::scanner::TokenType;

#[derive(Clone)]
pub enum Modifier {
    Between { from: u8, to: u8 },
    Multiple,
    Two,
}

impl Modifier {
    pub(crate) fn is_complete(&self, number_of_matches: u8) -> bool {
        match self {
            Modifier::Between { from: _from, to } => &number_of_matches == to,
            Modifier::Multiple => false,
            Modifier::Two => number_of_matches == 2,
        }
    }

    pub(crate) fn is_finite(&self) -> bool {
        match self {
            Modifier::Between { .. } => true,
            Modifier::Multiple => false,
            Modifier::Two => true,
        }
    }
}

#[derive(Clone, EnumIter)]
enum StatementBuilder {
    Assignment,
    Expression,
}

impl StatementBuilder {
    pub fn next_statement_from<'src, 'ast>(parser: &mut AstParser<'src>) -> Option<Statement<'ast>>
    where
        'src: 'ast,
    {
        for statement in StatementBuilder::iter() {
            parser.start_transaction();
            let result = statement.optionally_get_statement_from(parser);
            if result.is_some() {
                parser.commit();
                return result;
            } else {
                parser.rollback();
            }
        }
        None
    }

    pub fn optionally_get_statement_from<'src, 'ast>(&self, parser: &mut AstParser<'src>) -> Option<Statement<'ast>>
    where
        'src: 'ast,
    {
        match self {
            StatementBuilder::Assignment => None,
            StatementBuilder::Expression => {
                let statement = find_next_expression(parser).map(|expression| Statement::new_expression_statement(expression));
                if statement.is_some() {
                    consume_next_token_if_type(TokenType::SemiColon, parser)?;
                }
                statement
            },
        }
    }
}

pub fn find_next_statement<'src, 'token, 'ast>(parser: &mut AstParser<'src>) -> Option<Statement<'ast>>
where
    'src: 'token,
    'token: 'ast,
{
    parser.auto_commit(false);

    while parser.has_more_tokens() {
        parser.start_transaction();
        let result = StatementBuilder::next_statement_from(parser);
        if result.is_some() {
            parser.commit();
            parser.auto_commit(true);
            return result;
        } else {
            todo!("Something went wrong. Seek to the end of the statement and error");
            parser.rollback();
        }
    }

    parser.auto_commit(true);

    // ensure we haven't consumed any tokens in this failed attempt
    parser.rollback();
    None
}
