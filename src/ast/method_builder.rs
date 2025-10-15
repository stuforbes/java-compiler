use crate::ast::expression::{CallExpression, Expression, StringLiteral};
use crate::ast::statement::{ExpressionStatement, Statement};
use crate::ast::AstParser;
use crate::scanner::{Token, TokenType};

pub struct AstStatementBuilder<'p, 'src, 'tokens>
where
    'src: 'tokens,
    'tokens: 'p,
{
    parser: &'p mut AstParser<'src, 'tokens>,
    statements: Vec<Box<dyn Statement>>,
}

impl<'p, 'src, 'tokens> AstStatementBuilder<'p, 'src, 'tokens>
where
    'src: 'tokens,
    'tokens: 'p,
{
    pub(crate) fn new(parser: &'p mut AstParser<'src, 'tokens>) -> Self {
        Self {
            parser,
            statements: vec![],
        }
    }

    pub fn build(&mut self) {
        while self.parser.peek_next().token_type() != TokenType::RightBrace {
            self.next_statement();
        }
    }

    pub fn statements(self) -> Vec<Box<dyn Statement>> {
        self.statements
    }

    fn next_statement(&mut self) {
        // let next_token = self.parser.peek_next();
        self.expression_statement();
        // match next_token.token_type() {
        //     TokenType::Class => panic!("Unexpected token {:?}", next_token),
        //     TokenType::Public => panic!("Unexpected token {:?}", next_token),
        //     TokenType::Static => panic!("Unexpected token {:?}", next_token),
        //     TokenType::LeftParen => todo!(),
        //     TokenType::RightParen => todo!(),
        //     TokenType::LeftBrace => todo!(),
        //     TokenType::RightBrace => todo!(),
        //     TokenType::LeftSquareBracket => todo!(),
        //     TokenType::RightSquareBracket => todo!(),
        //     TokenType::SemiColon => todo!(),
        //     TokenType::Dot => todo!(),
        //     TokenType::Comma => todo!(),
        //     TokenType::Identifier => todo!(),
        //     TokenType::String => todo!(),
        //     TokenType::Eof => panic!("Unexpected token {:?}", next_token),
        // }
    }

    fn expression_statement<'ast>(&mut self) -> Box<dyn Statement + 'ast>
    where
        'src: 'ast,
    {
        let expression = self.expression();
        self.consume(TokenType::SemiColon);

        Box::new(ExpressionStatement::new(expression))
    }

    fn expression<'ast>(&mut self) -> Box<dyn Expression + 'ast>
    where
        'src: 'ast,
    {
        let next_token = self.parser.peek_next();
        if next_token.token_type() == TokenType::Identifier {
            return self.identifier_expression();
        }
        if next_token.token_type() == TokenType::String {
            return self.string_literal();
        }

        todo!()
    }

    fn identifier_expression<'ast>(&mut self) -> Box<dyn Expression + 'ast>
    where
        'src: 'ast,
    {
        self.identifier_expression_for_fully_qualified_object(
            self.parser.position(),
            self.parser.position(),
        )
    }

    fn identifier_expression_for_fully_qualified_object<'ast>(
        &mut self,
        object_path_start: usize,
        object_path_end: usize,
    ) -> Box<dyn Expression + 'ast>
    where
        'src: 'ast,
    {
        let position = self.parser.position();
        let start_token = self.consume(TokenType::Identifier);

        let next = self.parser.peek_next();
        if next.token_type() == TokenType::LeftParen {
            self.consume(TokenType::LeftParen);
            self.call_expression(start_token.lexeme(), object_path_start, object_path_end)
        } else if next.token_type() == TokenType::Dot {
            self.consume(TokenType::Dot);
            self.identifier_expression_for_fully_qualified_object(object_path_start, position)
        } else {
            panic!("Unexpected token {:?}", next.token_type())
        }
    }

    fn call_expression<'ast>(
        &mut self,
        method_name: &'src str,
        object_start_position: usize,
        object_end_position: usize,
    ) -> Box<dyn Expression + 'ast>
    where
        'src: 'ast,
    {
        let mut arguments: Vec<Box<dyn Expression>> = vec![];
        let mut next_token_type = self.parser.peek_next().token_type();
        while next_token_type != TokenType::RightParen {
            let arg = { self.expression() };
            arguments.push(arg);
            next_token_type = self.parser.peek_next().token_type()
        }
        self.consume(TokenType::RightParen);

        Box::new(CallExpression::new(
            self.parser
                .lexemes_from_position(object_start_position, object_end_position),
            method_name,
            arguments,
        ))
    }

    fn string_literal<'ast>(&mut self) -> Box<dyn Expression + 'ast>
    where
        'src: 'ast,
    {
        let token = self.consume(TokenType::String);
        Box::new(StringLiteral::new(token.lexeme()))
    }

    fn consume(&mut self, expected_type: TokenType) -> Token<'src> {
        if self.parser.peek_next().token_type() == expected_type {
            self.parser.next_token()
        } else {
            panic!(
                "Expected {:?}, but was {:?}",
                expected_type,
                self.parser.peek_next().token_type()
            )
        }
    }
}
