use std::io::Read;

use super::tokenizer;
use error::Error;

/// The abstract syntax tree of a .gmlpp program
pub enum AST {
    Block,
    Start,
    End,
}

impl AST {
    /// Creates a new AST by parsing a reader
    pub fn from_reader<R>(reader: R) -> Result<AST, Error> where R: Read {
        let tokens = tokenizer::tokenize(reader);
        Ok(AST::Start)
    }
}
