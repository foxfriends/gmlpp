use std::io::Read;

use super::tokenizer;
use error::Error;

mod fragment;
mod code;

mod argument_list;
mod comma_list;
mod identifier;
mod expression;
mod statements;
mod value;
mod literal;
mod call;
mod doc_comment;

use self::code::Code;
use self::fragment::Fragment;

/// The abstract syntax tree of a .gmlpp program
#[derive(Debug)]
pub struct AST(Code);

impl AST {
    fn new(code: Code) -> Self {
        AST(code)
    }

    /// Creates an AST of an event by parsing a reader
    pub fn from_reader<R>(reader: R) -> Result<Self, Error> where R: Read + Send {
        let tokens = tokenizer::tokenize(reader)?;
        Code::parse(&tokens).map(AST::new)
    }

    /// Prints the GMLPP code this tree is encoding
    pub fn print(&self) -> String {
        format!("{}", self.0)
    }
}
