use std::fmt::{self, Display, Formatter};

use super::fragment::Fragment;
use super::super::tokenizer::{Token, Tokens};
use error::{Error, ParseError};

#[derive(Clone, Debug)]
pub struct Identifier(String);

impl Display for Identifier {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Fragment for Identifier {
    fn parse(tokens: &Tokens) -> Result<Self, Error> {
        match tokens.peek() {
            Token::Identifier(ident) => {
                tokens.skip(1);
                Ok(Identifier(ident))
            }
            _ => {
                eprintln!("Failed to match Identifier: {:?}", tokens.peek());
                Err(Error::ParseError(ParseError::ExpectedIdentifier))
            }
        }
    }
}
