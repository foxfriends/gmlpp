use std::fmt::{self, Display, Formatter};

use super::fragment::Fragment;
use super::expression::Expression;
use super::super::tokenizer::{Token, Tokens};
use error::Error;

#[derive(Clone, Debug)]
pub struct CommaList(Vec<Expression>);

impl Display for CommaList {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        for item in &self.0 {
            write!(f, "{}, ", item)?;
        }
        Ok(())
    }
}

impl Fragment for CommaList {
    fn parse(tokens: &Tokens) -> Result<Self, Error> {
        let mut items = Vec::new();
        loop {
            let start = tokens.pos();
            match Expression::parse(tokens) {
                Ok(expr) => items.push(expr),
                // just pretend it's ok? someone else will have problems later instead!
                Err(..) => {
                    tokens.seek(start);
                    eprintln!("Failed to match CommaList: {:?}", tokens.peek());
                    break;
                },
            }
            if tokens.peek() == Token::Comma {
                tokens.skip(1);
            } else {
                // no comma, must be last element
                break;
            }
        }
        Ok(CommaList(items))
    }
}
