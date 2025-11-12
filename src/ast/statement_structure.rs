use crate::ast::AstParser;
use crate::ast::expression::Expression;
use crate::ast::statement::Statement;
use crate::scanner::{Token, TokenType};

#[derive(Copy, Clone)]
enum StatementType {
    Assignment,
}

#[derive(Clone)]
enum Modifier {
    Between { from: u8, to: u8 },
    Multiple,
    Two,
}

#[derive(Clone)]
enum Component {
    Identifier,
    Expression,
}
impl Component {
    fn matches(&self, token: &Token) -> bool {
        match self {
            Component::Identifier => token.token_type() == TokenType::Identifier,
            Component::Expression => false
        }
    }
}

#[derive(Clone)]
struct StatementFragment {
    modifier: Option<Modifier>,
    component: Component,
}

impl StatementFragment {
    fn matches(&self, token: &Token) -> bool {
        self.component.matches(token)
    }
}

#[derive(Clone)]
struct StatementStructure {
    statement_type: StatementType,
    fragments: Vec<StatementFragment>,
}

impl StatementStructure {
    pub fn fully_matches(&self, tokens: &Vec<Token>) -> bool {
        if self.fragments.len() != tokens.len() {
            return false
        }

        StatementStructure::matches(&self.fragments, tokens)
    }

    pub fn partially_matches(&self, tokens: &Vec<Token>) -> bool {
        if self.fragments.len() < tokens.len() {
            return false
        }

        StatementStructure::matches(&self.fragments, tokens)
    }

    fn matches(fragments: &Vec<StatementFragment>, tokens: &Vec<Token>) -> bool {
        for (fragment, token) in fragments.iter().zip(tokens) {
            if !fragment.matches(token) {
                return false
            }
        }
        true
    }
}

pub fn new_foo() -> Foo {
    Foo::new(vec![])
}

pub struct Foo {
    statement_structures: Vec<StatementStructure>
}
impl <'ast> Foo {
    fn new(statement_structures: Vec<StatementStructure>) -> Self {
        Self {
            statement_structures,
        }
    }

    pub fn find_next_statement<'src, 'token>(&self, parser: &mut AstParser<'src>) -> Option<Statement<'ast>>
    where
        'src: 'token,
        'token: 'ast,
    {
        let candidates: &mut Vec<StatementStructure> = &mut self.statement_structures.clone();
        let mut tokens: Vec<Token<'token>> = vec![];

        while parser.has_more_tokens() {
            let token = parser.next_token();
            tokens.push(token);

            for i in (0..candidates.len()).rev() {
                if candidates[i].fully_matches(&tokens) {
                    return Some(self.create_statement_for(candidates[i].statement_type, &tokens))
                }
                if !candidates[i].partially_matches(&tokens) {
                    candidates.remove(i);
                }
            }
        }

        None
    }

    fn create_statement_for(&self, statement_type: StatementType, tokens: &Vec<Token<'ast>>) -> Statement<'ast> {
        match statement_type {
            StatementType::Assignment => self.do_variable_assignment(tokens),
        }
    }

    fn do_variable_assignment(&self, tokens: &Vec<Token<'ast>>) -> Statement<'ast> {
        let is_final = false; // TODO: implement

        let (name, type_def, num_tokens_used) = if Self::is_token_of_type(tokens, 1, TokenType::Identifier)  {
            (tokens[1].lexeme(), Some(tokens[0].lexeme()), 2)
        } else {
            (tokens[0].lexeme(), None, 1)
        };

        let value = if tokens.len() >= num_tokens_used {
            Some(self.do_expression(&tokens[num_tokens_used..]))
        }  else {
            None
        };

        Statement::new_var_assignment(name, type_def, is_final, value)
    }

    fn do_expression(&self, _tokens: &[Token]) -> Expression<'ast> {
        todo!()
    }

    fn is_token_of_type(tokens: &Vec<Token>, position: usize, expected_token_type: TokenType) -> bool {
        if tokens.len() <= position {
            return false
        }

        return tokens[position].token_type() == expected_token_type
    }
}