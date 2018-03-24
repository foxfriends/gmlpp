use std::fmt::{self, Display, Formatter};

use super::identifier::Identifier;
use super::fragment::Fragment;
use super::super::tokenizer::Tokens;
use error::Error;

#[derive(Clone, Debug)]
pub enum Expression {
    Ident(Identifier),
}

impl Display for Expression {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        use self::Expression::*;
        match self {
            &Ident(ref ident) => write!(f, "{}", ident),
        }
    }
}

impl Fragment for Expression {
    fn parse(tokens: &Tokens) -> Result<Self, Error> {
        unimplemented!()
    }
}
