use std::fmt::{self, Display, Formatter};

use super::fragment::Fragment;
use super::argument_list::ArgumentList;
use super::statements::Statements;
use super::doc_comment::DocComment;
use super::super::tokenizer::{Token, Tokens};
use error::Error;

#[derive(Clone, Debug)]
pub struct Code {
    docs: DocComment,
    args: ArgumentList,
    body: Statements,
}

impl Display for Code {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}\n{}", self.args, self.body)
    }
}

impl Fragment for Code {
    fn parse(tokens: &Tokens) -> Result<Self, Error> {
        assert_eq!(tokens[0], Token::BOF);
        tokens.skip(1);
        let docs = DocComment::parse(tokens)?;
        let args = ArgumentList::parse(tokens)?;
        let body = Statements::parse(tokens)?;
        Ok(Self { docs, args, body })
    }
}
