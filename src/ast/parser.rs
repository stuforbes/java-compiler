use crate::scanner::{Token, TokenType};

pub struct AstParser<'src> {
    tokens: Vec<Token<'src>>,
    position: usize,
    committed_position: usize,
    auto_commit: bool,
}

impl<'src> AstParser<'src> {
    pub fn for_tokens(tokens: Vec<Token<'src>>) -> Self {
        Self {
            tokens,
            position: 0,
            committed_position: 0,
            auto_commit: true,
        }
    }

    pub(crate) fn next_token(&mut self) -> Token<'src> {
        let i = self.position;
        self.position = i + 1;
        if self.auto_commit {
            self.committed_position = self.position
        }
        self.tokens[i]
    }

    pub fn has_more_tokens(&self) -> bool {
        self.position < self.tokens.len()
    }

    pub fn auto_commit(&mut self, auto_commit: bool) {
        self.auto_commit = auto_commit
    }

    pub fn commit(&mut self) {
        self.ensure_auto_commit_disabled();
        self.committed_position = self.position
    }

    pub fn rollback(&mut self) {
        self.ensure_auto_commit_disabled();
        self.position = self.committed_position
    }

    pub(crate) fn peek_next(&self) -> Token<'src> {
        self.tokens[self.position]
    }

    pub(crate) fn is_next_token(&self, token_type: TokenType) -> bool {
        self.peek_next().token_type() == token_type
    }

    fn position(&self) -> usize {
        self.position
    }

    fn lexemes_from_position(&self, from: usize, to: usize) -> Vec<&'src str> {
        self.tokens[from..=to]
            .iter()
            .filter(|t| t.token_type() != TokenType::Dot)
            .map(|t| t.lexeme())
            .collect()
    }

    fn ensure_auto_commit_disabled(&mut self) {
        if !self.auto_commit {
            panic!("Auto commit is enabled")
        }
    }
}
