use crate::ast::class::AstStatement;
use crate::ast::AstParser;
use crate::scanner::{Token, TokenType};

pub struct AstStatementBuilder<'p, 'src, 'tokens, 'ast,>
where
    'src: 'tokens,
    'tokens: 'ast,
    'ast: 'p
{
    parser: &'p mut AstParser<'src, 'tokens>,
    statements: Vec<AstStatement<'ast>>,
}

impl<'p, 'src, 'tokens, 'ast> AstStatementBuilder<'p, 'src, 'tokens, 'ast>
where
    'src: 'tokens,
    'tokens: 'ast,
    'ast: 'p,
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

    pub fn statements(self) -> Vec<AstStatement<'ast>> {
        self.statements
    }

    fn next_statement(&mut self) {
        let next_token = self.parser.peek_next();
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

    fn expression_statement(&mut self) -> Box<dyn Statement + 'ast> {
        Box::new(ExpressionStatement {
            expression: self.expression(),
        })
    }

    fn expression(&mut self) -> Box<dyn Expression + 'ast> {
        let next_token = self.parser.peek_next();
        if next_token.token_type() == TokenType::Identifier {
            return self.identifier_expression();
        }
        if next_token.token_type() == TokenType::String {
            return self.string_literal();
        }

        todo!()
    }

    fn identifier_expression(&mut self) -> Box<dyn Expression + 'ast> {
        self.identifier_expression_for_fully_qualified_object(self.parser.position())
    }

    fn identifier_expression_for_fully_qualified_object(
        &mut self,
        object_path_start: usize,
    ) -> Box<dyn Expression + 'ast> {
        let start_token = self.consume(TokenType::Identifier);

        let next = self.parser.peek_next();
        if next.token_type() == TokenType::LeftParen {
            self.call_expression(start_token.lexeme(), object_path_start, start_token.start())
        } else if next.token_type() == TokenType::Dot {
            self.consume(TokenType::Dot);
            self.identifier_expression_for_fully_qualified_object(object_path_start)
        } else {
            panic!("Unexpected token {:?}", next.token_type())
        }
    }

    fn call_expression(
        &mut self,
        method_name: &'src str,
        object_start_position: usize,
        object_end_position: usize,
    ) -> Box<dyn Expression + 'ast> {
        let mut arguments: Vec<Box<dyn Expression + 'ast>> = vec![];
        let next_token_type = self.parser.peek_next().token_type();
        while next_token_type != TokenType::RightParen {
            let arg = { self.expression() };
            arguments.push(arg);
        }

        let s: &'ast str = self
            .parser
            .lexemes_from_position(object_start_position, object_end_position)
            .clone();

        Box::new(CallExpression {
            object_path: s, //self.parser.lexemes_from_position(object_start_position, object_end_position),
            method_name,
            arguments,
        })
    }

    fn string_literal(&mut self) -> Box<dyn Expression + 'ast> {
        let token = self.consume(TokenType::String);
        Box::new(StringLiteral {
            value: token.lexeme(),
        })
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

trait Statement {}

struct ExpressionStatement<'ast> {
    expression: Box<dyn Expression + 'ast>,
}
impl<'ast> Statement for ExpressionStatement<'ast> {}

trait Expression {}

struct CallExpression<'ast> {
    object_path: &'ast str,
    method_name: &'ast str,
    arguments: Vec<Box<dyn Expression + 'ast>>,
}
impl<'ast> Expression for CallExpression<'ast> {}

struct StringLiteral<'ast> {
    value: &'ast str,
}
impl<'ast> Expression for StringLiteral<'ast> {}
