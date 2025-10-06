use crate::scanner::literal::Literal;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum TokenType {
    // Reserved keywords
    Class,
    Public,
    Static,

    // Single character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    // DoubleQuote,
    SemiColon,
    Dot,
    Comma,

    // Literals
    Identifier,
    String,

    Eof,
}

#[derive(Debug, Copy, Clone)]
pub struct Token<'a> {
    token_type: TokenType,
    lexeme: Option<&'a str>,
    literal: Option<Literal<'a>>,
}

impl<'a> Token<'a> {
    pub fn empty(token_type: TokenType) -> Self {
        Self {
            token_type,
            lexeme: None,
            literal: None,
        }
    }

    pub fn without_literal(token_type: TokenType, lexeme: &'a str) -> Self {
        Self {
            token_type,
            lexeme: Some(lexeme),
            literal: None,
        }
    }

    pub fn with_literal(token_type: TokenType, lexeme: &'a str, literal: Literal<'a>) -> Self {
        Self {
            token_type,
            lexeme: Some(lexeme),
            literal: Some(literal),
        }
    }

    pub fn token_type(&self) -> TokenType {
        self.token_type
    }

    pub fn lexeme(&self) -> &'a str {
        match self.lexeme {
            Some(l) => l,
            None => panic!("Unavailable for token {:?}", self.token_type)
        }
    }
}
