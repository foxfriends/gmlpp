use std::cell::Cell;
use std::ops::{Index, Range, RangeTo};
use super::Token;

/// A sequence of tokens with a marker at the current position, like a Cursor but less awkward
pub struct Tokens {
    tokens: Vec<Token>,
    pos: Cell<usize>,
}

impl Tokens {
    /// Creates a new instance
    pub fn new(tokens: Vec<Token>) -> Self {
        Tokens {
            tokens,
            pos: Cell::new(0),
        }
    }

    /// Returns the token at the current position
    pub fn peek(&self) -> Token {
        self[self.pos.get()].clone()
    }

    /// Returns the current token, and increments the counter by one
    pub fn next(&self) -> Token {
        self.skip(1);
        self[self.pos.get() - 1].clone()
    }

    /// Increments the internal counter by some amount
    pub fn skip(&self, n: usize) {
        self.pos.set(self.pos.get() + n);
    }

    /// Decrements the internal counter by some amount
    pub fn back(&self, n: usize) {
        self.pos.set(self.pos.get() - n);
    }
}

impl Index<usize> for Tokens {
    type Output = Token;
    fn index(&self, index: usize) -> &Token {
        &self.tokens[self.pos.get() + index]
    }
}

impl Index<Range<usize>> for Tokens {
    type Output = [Token];
    fn index(&self, index: Range<usize>) -> &[Token] {
        &self.tokens[index.start + self.pos.get()..index.end + self.pos.get()]
    }
}

impl Index<RangeTo<usize>> for Tokens {
    type Output = [Token];
    fn index(&self, index: RangeTo<usize>) -> &[Token] {
        &self[0..index.end]
    }
}
