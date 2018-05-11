use std::fmt::{self, Display, Formatter};

use super::fragment::Fragment;
use super::identifier::Identifier;
use super::literal::Literal;
use super::expression::Expression;
use super::call::Call;
use super::super::tokenizer::{Token, Tokens};
use error::{Error, ParseError};

#[derive(Clone, Debug)]
pub enum Value {
    Ident(Identifier),
    Literal(Literal),
    Expr(Box<Expression>),
    Call(Call),
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        use self::Value::*;
        match self {
            &Ident(ref ident) => ident.fmt(f),
            &Literal(ref literal) => literal.fmt(f),
            &Expr(ref expr) => write!(f, "({})", expr),
            &Call(ref call) => call.fmt(f),
        }
    }
}

impl Fragment for Value {
    fn parse(tokens: &Tokens) -> Result<Self, Error> {
        match tokens[..2] {
            [Token::Identifier(..), Token::LParen] => Ok(Value::Call(Call::parse(tokens)?)),
            [Token::Identifier(..), _] => Ok(Value::Ident(Identifier::parse(tokens)?)),
            [Token::BinLiteral(..), _] |
            [Token::DecLiteral(..), _] |
            [Token::HexLiteral(..), _] |
            [Token::StrLiteral(..), _] |
            [Token::CharLiteral(..), _] |
            [Token::FalseLiteral, _] |
            [Token::TrueLiteral, _] |
            [Token::UndefinedLiteral, _] => Ok(Value::Literal(Literal::parse(tokens)?)),
            [Token::LParen, _] => {
                tokens.skip(1);
                let expr = Expression::parse(tokens)?;
                if tokens.peek() != Token::RParen {
                    Err(Error::ParseError(ParseError::MismatchedParentheses))
                } else {
                    Ok(Value::Expr(box expr))
                }
            }
            _ => {
                eprintln!("Failed to match Value: {:?}", tokens.peek());
                Err(Error::ParseError(ParseError::ExpectedValue))
            }
        }
    }
}
