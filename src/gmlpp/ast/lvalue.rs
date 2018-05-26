use std::fmt::{self, Display, Formatter};

use super::fragment::Fragment;
use super::identifier::Identifier;
use super::super::tokenizer::{Token, Tokens};
use error::{Error, ParseError};

// TODO: other case
#[derive(Clone, Debug)]
pub enum LValue {
    Identifier(Identifier),
}

impl Display for LValue {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            LValue::Identifier(ref ident) => ident.fmt(f),
        }
    }
}

impl Fragment for LValue {
    fn parse(tokens: &Tokens) -> Result<Self, Error> {
        Ok(LValue::Identifier(Identifier::parse(tokens)?))
    }
}
