use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

use super::fragment::Fragment;
use super::super::tokenizer::{Token, Tokens};
use error::{Error, ParseError};

#[derive(Clone, Debug)]
pub enum Literal {
    Numeric(f64),
    Boolean(bool),
    String(String),
    Char(char),
    Undefined,
}

impl Display for Literal {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Literal::Numeric(n) => n.fmt(f),
            Literal::Boolean(b) => b.fmt(f),
            Literal::String(s) => write!(f, "{:?}", s),
            Literal::Char(c) => write!(f, "{:?}", c),
            Literal::Undefined => write!(f, "undefined"),
        }
    }
}

impl Fragment for Literal {
    fn parse(tokens: &Tokens) -> Result<Self, Error> {
        match tokens.next() {
            Token::BinLiteral(bin) => Ok(Literal::Numeric(i64::from_str_radix(&bin, 2)? as f64)),
            Token::HexLiteral(hex) => Ok(Literal::Numeric(i64::from_str_radix(&hex, 2)? as f64)),
            Token::DecLiteral(dec) => Ok(Literal::Numeric(f64::from_str(&dec)?)),
            Token::TrueLiteral => Ok(Literal::Boolean(true)),
            Token::FalseLiteral => Ok(Literal::Boolean(false)),
            Token::StrLiteral(string) => Ok(Literal::String(string)),
            Token::CharLiteral(string) => Ok(Literal::Char(unimplemented!())),
            Token::UndefinedLiteral => Ok(Literal::Undefined),
            _ => {
                tokens.back(1);
                eprintln!("Failed to match Literal: {:?}", tokens.peek());
                Err(Error::ParseError(ParseError::ExpectedLiteral))
            }
        }
    }
}
