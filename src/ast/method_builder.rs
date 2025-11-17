use crate::ast::statement::Statement;
use crate::ast::AstParser;
use crate::ast::statement_structure::find_next_statement;
use crate::scanner::TokenType;

pub struct AstStatementBuilder<'p, 'src, 'ast>
where
    'src: 'ast,
{
    parser: &'p mut AstParser<'src>,
    statements: Vec<Statement<'ast>>,
}

impl<'p, 'src, 'tokens, 'ast> AstStatementBuilder<'p, 'src, 'ast>
where
    'src: 'tokens,
{
    pub(crate) fn new(parser: &'p mut AstParser<'src>) -> Self {
        Self {
            parser,
            statements: vec![]
        }
    }

    pub fn build(&mut self) {
        while self.parser.has_more_tokens() && self.parser.peek_next().token_type() != TokenType::RightBrace {
            self.next_statement();
        }
    }

    pub fn statements(self) -> Vec<Statement<'ast>> {
        self.statements
    }

    fn next_statement(&mut self) {
        if let Some(statement) = find_next_statement(self.parser) {
            self.statements.push(statement);
        }
        // let statement = self.expression_statement();
        // self.statements.push(statement);
    }

}