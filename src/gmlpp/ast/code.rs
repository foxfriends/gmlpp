use std::fmt::{self, Display, Formatter};

use super::fragment::Fragment;
use super::argument_list::ArgumentList;
use super::sequence::Sequence;
use super::super::tokenizer::Tokens;
use error::Error;

#[derive(Clone, Debug)]
pub struct Code {
    args: ArgumentList,
    body: Sequence,
}

impl Display for Code {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.args)
    }
}

impl Fragment for Code {
    fn parse(tokens: &Tokens) -> Result<Self, Error> {
        let args = ArgumentList::parse(tokens)?;
        Ok(Code { args, body: Sequence::End })
    }
}
