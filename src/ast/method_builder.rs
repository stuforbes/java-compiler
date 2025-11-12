use crate::ast::expression::Expression;
use crate::ast::statement::Statement;
use crate::ast::AstParser;
use crate::ast::statement_structure::{new_foo, Foo};
use crate::scanner::{Literal, Token, TokenType};

pub struct AstStatementBuilder<'p, 'src, 'ast>
where
    'src: 'ast,
{
    parser: &'p mut AstParser<'src>,
    foo: Foo,
    statements: Vec<Statement<'ast>>,
}

impl<'p, 'src, 'tokens, 'ast> AstStatementBuilder<'p, 'src, 'ast>
where
    'src: 'tokens,
{
    pub(crate) fn new(parser: &'p mut AstParser<'src>) -> Self {
        Self {
            parser,
            foo: new_foo(),
            statements: vec![]
        }
    }

    pub fn build(&mut self) {
        while self.parser.peek_next().token_type() != TokenType::RightBrace {
            self.next_statement();
        }
    }

    pub fn statements(self) -> Vec<Statement<'ast>> {
        self.statements
    }

    fn next_statement(&mut self) {
        if let Some(statement) = self.foo.find_next_statement(self.parser) {
            self.statements.push(statement);
        }
        // let statement = self.expression_statement();
        // self.statements.push(statement);
    }

    fn expression_statement(&mut self) -> Statement<'ast> {
        let expression = self.expression();
        self.consume(TokenType::SemiColon);

        Statement::new_expression_statement(expression)
    }

    fn expression(&mut self) -> Expression<'ast> {
        self.assignment()
    }

    fn assignment(&mut self) -> Expression<'ast> {
        let mut expression = self.call();

        if self.parser.is_next_token(TokenType::Identifier) {
            expression = match expression {
                Expression::Variable { name, type_def: None } => Expression::new_variable(self.consume(TokenType::Identifier).lexeme(), Some(name)),
                expr => expr,
            }
        }

        if self.parser.is_next_token(TokenType::Equal) {
            self.consume(TokenType::Equal);
            let value = self.expression();
            expression = match expression {
                Expression::Variable { name, type_def } => Expression::new_assignment(name, type_def, value),
                expr => {
                    panic!("unexpected expression {:?}", expr);
                }
            }
        }

        return expression;
    }

    fn call(&mut self) -> Expression<'ast> {
        let mut expr = self.primary();

        while true {
            if self.parser.is_next_token(TokenType::LeftParen) {
                self.consume(TokenType::LeftParen);
                let mut arguments: Vec<Expression<'ast>> = vec![];
                let mut next_token_type = self.parser.peek_next().token_type();
                while next_token_type != TokenType::RightParen {
                    let arg = { self.expression() };
                    arguments.push(arg);
                    next_token_type = self.parser.peek_next().token_type()
                }
                self.consume(TokenType::RightParen);

                let (parent_expr, method_name) = Self::deconstruct_method_name_from(expr);

                return Expression::new_call(parent_expr, method_name, arguments);
            } else if self.parser.is_next_token(TokenType::Dot) {
                self.consume(TokenType::Dot);

                expr = Expression::new_child_identifier(expr, self.consume(TokenType::Identifier).lexeme())
            } else {
                break;
            }
        }

        expr
    }

    fn primary(&mut self) -> Expression<'ast> {
        let next_token = self.parser.peek_next();
        if next_token.token_type() == TokenType::Identifier {
            return Expression::new_variable(self.consume(TokenType::Identifier).lexeme(), None);
        }
        if next_token.token_type() == TokenType::String {
            return self.string_literal();
        }

        panic!("Unknown token {:?}", next_token);
    }

    fn string_literal(&mut self) -> Expression<'ast> {
        let token = self.consume(TokenType::String);
        match token.literal() {
            Literal::String(value) => Expression::new_string_literal(value),
        }
    }

    fn consume(&mut self, expected_type: TokenType) -> Token<'src> {
        if self.parser.peek_next().token_type() == expected_type {
            self.parser.next_token()
        } else {
            panic!("Expected {:?}, but was {:?}", expected_type, self.parser.peek_next().token_type())
        }
    }

    fn deconstruct_method_name_from<'a>(expression: Expression<'a>) -> (Expression<'a>, &'a str) {
        match expression {
            Expression::ChildIdentifier { parent, name } => (unbox(parent), name),
            _expr => panic!("Unsupported"),
        }
    }
}

fn unbox<T>(value: Box<T>) -> T {
    *value
}
