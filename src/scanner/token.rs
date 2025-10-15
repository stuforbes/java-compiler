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
    LeftSquareBracket,
    RightSquareBracket,
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
    start: usize,
    end: usize,
}

impl<'a> Token<'a> {
    pub fn empty(token_type: TokenType, position: usize) -> Self {
        Self {
            token_type,
            lexeme: None,
            literal: None,
            start: position,
            end: position,
        }
    }

    pub fn without_literal(token_type: TokenType, lexeme: &'a str, start: usize, end: usize) -> Self {
        Self {
            token_type,
            lexeme: Some(lexeme),
            literal: None,
            start,
            end,
        }
    }

    pub fn with_literal(token_type: TokenType, lexeme: &'a str, literal: Literal<'a>, start: usize, end: usize) -> Self {
        Self {
            token_type,
            lexeme: Some(lexeme),
            literal: Some(literal),
            start,
            end,
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
    
    pub fn literal(&self) -> &Literal<'a> {
        match &self.literal {
            Some(l) => l,
            None => panic!("Unavailable for token {:?}", self.token_type)
        }
    }
    
    pub fn start(&self) -> usize {
        self.start
    }

    pub fn end(&self) -> usize {
        self.end
    }
}
