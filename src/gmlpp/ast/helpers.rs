//! Some helper functions for parsing

use super::super::tokenizer::{Token, Tokens};
use super::fragment::Fragment;
use error::{Error, ParseError};

pub fn semi_or_eol(tokens: &Tokens) -> Result<(), ParseError> {
    match tokens[..2] {
        [Token::Semi, Token::EOL] => tokens.skip(2),
        [Token::Semi, _] | [Token::EOL, _] => tokens.skip(1),
        _ => return Err(ParseError::ExpectedEndOfStatement),
    }
    Ok(())
}

pub fn parenthesized<T: Fragment>(tokens: &Tokens) -> Result<T, Error> {
    if tokens[0] != Token::LParen {
        return Err(Error::ParseError(ParseError::ExpectedParentheses));
    }
    tokens.skip(1);
    let thing = T::parse(tokens)?;
    if tokens[0] != Token::RParen {
        return Err(Error::ParseError(ParseError::MismatchedParentheses));
    }
    tokens.skip(1);
    Ok(thing)
}
