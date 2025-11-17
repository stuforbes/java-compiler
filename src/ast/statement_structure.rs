use crate::ast::expression_structure::{consume_next_token_if_type, find_next_expression};
use crate::ast::statement::Statement;
use crate::ast::AstParser;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use crate::scanner::TokenType;

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
            StatementBuilder::Assignment => {
                let mut name = consume_next_token_if_type(TokenType::Identifier, parser)?;
                let mut type_def = None;
                if parser.is_next_token(TokenType::Identifier) {
                    type_def = Some(name.lexeme());
                    name = consume_next_token_if_type(TokenType::Identifier, parser)?;
                }

                consume_next_token_if_type(TokenType::Equal, parser)?;

                let statement = Statement::new_var_assignment(name.lexeme(), type_def, false, find_next_expression(parser));
                consume_next_token_if_type(TokenType::SemiColon, parser)?;
                Some(statement)
            },
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
            parser.rollback();
            todo!("Something went wrong. Seek to the end of the statement and error");
        }
    }

    parser.auto_commit(true);

    // ensure we haven't consumed any tokens in this failed attempt
    parser.rollback();
    None
}
