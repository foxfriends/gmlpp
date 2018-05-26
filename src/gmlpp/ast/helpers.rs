//! Some helper functions for parsing

use super::super::tokenizer::{Token, Tokens};
use error::ParseError;

pub fn semi_or_eol(tokens: &Tokens) -> Result<(), ParseError> {
    match tokens[..2] {
        [Token::Semi, Token::EOL] => tokens.skip(2),
        [Token::Semi, _] | [Token::EOL, _] => tokens.skip(1),
        _ => return Err(ParseError::ExpectedEndOfStatement),
    }
    Ok(())
}
