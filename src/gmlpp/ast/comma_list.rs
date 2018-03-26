use std::fmt::{self, Display, Formatter};

use super::fragment::Fragment;
use super::expression::Expression;
use super::super::tokenizer::Tokens;
use error::Error;

#[derive(Clone, Debug)]
pub enum CommaList {
    Item(Box<Expression>, Box<CommaList>),
    End,
}

impl Display for CommaList {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        use self::CommaList::*;
        match self {
            &Item(ref expr, ref rest) => write!(f, "{}, {}", expr, rest),
            &End => write!(f, ""),
        }
    }
}

impl Fragment for CommaList {
    fn parse(tokens: &Tokens) -> Result<Self, Error> {
        unimplemented!()
    }
}
