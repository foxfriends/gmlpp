use std::fmt::{self, Display, Formatter};

use super::identifier::Identifier;
use super::comma_list::CommaList;
use super::super::tokenizer::{Token, Tokens};
use super::fragment::Fragment;
use error::{Error, ParseError};

#[derive(Clone, Debug)]
pub struct Call {
    name: Identifier,
    arguments: CommaList,
}

impl Display for Call {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}({})", self.name, self.arguments)
    }
}

impl Fragment for Call {
    fn parse(tokens: &Tokens) -> Result<Self, Error> {
        let pos = tokens.pos();
        let name = Identifier::parse(tokens)?;
        if tokens.next() != Token::LParen {
            tokens.seek(pos);
            return Err(Error::ParseError(ParseError::ExpectedFunctionCall))
        }
        let arguments = CommaList::parse(tokens)?;
        if tokens.next() != Token::RParen {
            tokens.back(1);
            eprintln!("Failed to match Call: {:?}", tokens.peek());
            tokens.seek(pos);
            return Err(Error::ParseError(ParseError::ExpectedArgument))
        }
        Ok(Self { name, arguments })
    }
}
