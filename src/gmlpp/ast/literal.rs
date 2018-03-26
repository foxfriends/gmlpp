use std::fmt::{self, Display, Formatter};

use super::fragment::Fragment;
use super::super::tokenizer::Tokens;
use error::Error;

#[derive(Clone, Debug)]
pub enum Literal {}

impl Display for Literal {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "")
    }
}

impl Fragment for Literal {
    fn parse(tokens: &Tokens) -> Result<Self, Error> {
        unimplemented!()
    }
}
