use std::fmt::{self, Display, Formatter};

use super::fragment::Fragment;
use super::argument_list::ArgumentList;
use super::statements::Statements;
use super::super::tokenizer::Tokens;
use error::Error;

#[derive(Clone, Debug)]
pub struct Code {
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
        let args = ArgumentList::parse(tokens)?;
        let body = Statements::parse(tokens)?;
        Ok(Self { args, body })
    }
}
