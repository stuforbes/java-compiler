use std::ops::Not;
use crate::scanner::{Token, TokenType};

pub struct AstParser<'src> {
    tokens: Vec<Token<'src>>,
    position: usize,
    transaction_positions: Vec<usize>,
    auto_commit: bool,
}

impl<'src> AstParser<'src> {
    pub fn for_tokens(tokens: Vec<Token<'src>>) -> Self {
        Self {
            tokens,
            position: 0,
            transaction_positions: vec![],
            auto_commit: true,
        }
    }

    pub(crate) fn next_token(&mut self) -> Token<'src> {
        let i = self.position;
        self.position = i + 1;
        self.tokens[i]
    }

    pub fn has_more_tokens(&self) -> bool {
        self.position < self.tokens.len()
    }

    pub fn auto_commit(&mut self, auto_commit: bool) {
        if auto_commit && self.transaction_positions.is_empty().not() {
            panic!("Turning on auto commit, but commit checkpoint depth is still {:}", self.transaction_positions.len())
        }
        self.auto_commit = auto_commit
    }

    pub fn start_transaction(&mut self) {
        self.transaction_positions.push(self.position)
    }

    pub fn commit(&mut self) {
        self.ensure_auto_commit_disabled();
        self.transaction_positions.pop();
    }

    pub fn rollback(&mut self) {
        self.ensure_auto_commit_disabled();
        self.position = self.transaction_positions.pop().unwrap();
    }

    pub(crate) fn peek_next(&self) -> Token<'src> {
        self.tokens[self.position]
    }

    pub(crate) fn is_next_token(&self, token_type: TokenType) -> bool {
        self.peek_next().token_type() == token_type
    }

    fn ensure_auto_commit_disabled(&mut self) {
        if self.auto_commit {
            panic!("Auto commit is enabled")
        }
    }
}
