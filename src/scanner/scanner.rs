use crate::scanner::literal::Literal;
use crate::scanner::token::{Token, TokenType};
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref IDENTIFIER_KEYWORDS: HashMap<&'static str, TokenType> = vec![
        ("class", TokenType::Class),
        ("public", TokenType::Public),
        ("static", TokenType::Static),
    ].into_iter().collect();

    static ref SINGLE_CHAR_TOKENS: HashMap<char, TokenType> = vec![
        ('(', TokenType::LeftParen),
        (')', TokenType::RightParen),
        ('{', TokenType::LeftBrace),
        ('}', TokenType::RightBrace),
        ('[', TokenType::LeftSquareBracket),
        (']', TokenType::RightSquareBracket),
        (';', TokenType::SemiColon),
        ('.', TokenType::Dot),
        (',', TokenType::Comma),
    ].into_iter().collect();
}

#[allow(clippy::needless_lifetimes)]
pub fn scan<'a>(source: &'a str) -> Vec<Token<'a>> {
    let mut scanner = crate::scanner::scanner::Scanner::for_source(source);
    scanner.scan_tokens();
    scanner.tokens()
}

pub struct Scanner<'a> {
    source: &'a str,
    token_start: usize,
    current_position: usize,
    end: usize,
    line: i32,
    tokens: Vec<Token<'a>>,
}

impl<'a> Scanner<'a> {
    pub fn for_source(source: &'a str) -> Self {
        Self {
            source,
            token_start: 0,
            current_position: 0,
            end: source.len(),
            line: 1,
            tokens: vec![],
        }
    }
    pub fn scan_tokens(&mut self) {
        while !self.is_finished() {
            self.prepare_token_start();

            self.next_token();
        }
        self.tokens.push(Token::empty(TokenType::Eof));
    }

    pub fn tokens(self) -> Vec<Token<'a>> {
        self.tokens
    }

    fn next_token(&mut self) {
        let next_char = self.next_char();
        if Self::is_newline(next_char) {
            self.line = self.line + 1;
            return;
        }

        if Self::is_whitespace(next_char) {
            return;
        }

        if let Some(token_type) = SINGLE_CHAR_TOKENS.get(&next_char) {
            self.tokens.push(self.create_token(*token_type));
            return;
        }

        if next_char == '"' {
            if let Some(token) = self.string_token() {
                self.tokens.push(token);
            }
            return;
        }

        if Self::is_alpha(next_char) {
            if let Some(token) = self.identifier_token() {
                self.tokens.push(token);
            }
            return;
        }

        // Handle unexpected character

    }

    fn create_token(&self, token_type: TokenType) -> Token<'a> {
        Token::without_literal(
            token_type,
            &self.source[self.token_start..self.current_position],
        )
    }

    fn create_token_with_literal(&self, token_type: TokenType, literal: Literal<'a>) -> Token<'a> {
        Token::with_literal(
            token_type,
            &self.source[self.token_start..self.current_position],
            literal,
        )
    }

    fn string_token(&mut self) -> Option<Token<'a>> {
        while self.peek() != '"' && !self.is_finished() {
            if self.peek() == '\n' {
                self.line = self.line + 1;
            }
            self.next_char();
        }

        if self.is_finished() {
            // TODO: handle unterminated string error
            return None;
        }

        // Closing quote
        self.next_char();

        let string_literal = &self.source[self.token_start + 1..self.current_position - 1];
        Some(self.create_token_with_literal(TokenType::String, Literal::String(string_literal)))
    }

    fn identifier_token(&mut self) -> Option<Token<'a>> {
        while Self::is_alpha_numeric(self.peek()) {
            self.next_char();
        }

        let identifier = &self.source[self.token_start..self.current_position];
        match IDENTIFIER_KEYWORDS.get(&identifier) {
            Some(identifier) => Some(self.create_token(*identifier)),
            None => Some(self.create_token(TokenType::Identifier)),
        }
    }

    fn prepare_token_start(&mut self) {
        self.token_start = self.current_position;
    }

    fn is_finished(&self) -> bool {
        self.current_position >= self.end
    }

    fn is_newline(c: char) -> bool {
        c == '\n'
    }

    fn is_whitespace(c: char) -> bool {
        c == ' ' || c == '\r' || c == '\t'
    }

    fn is_alpha(c: char) -> bool {
        (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_'
    }

    fn is_digit(c: char) -> bool {
        c >= '0' && c <= '9'
    }

    fn is_alpha_numeric(c: char) -> bool {
        Self::is_alpha(c) || Self::is_digit(c)
    }

    fn next_char(&mut self) -> char {
        let c = self.source[self.current_position..].chars().next().unwrap();
        self.current_position = self.current_position + 1;
        c
    }

    fn peek(&self) -> char {
        if self.is_finished() {
            return '\0';
        }
        self.source[self.current_position..].chars().next().unwrap()
    }
}
