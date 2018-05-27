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
    Negative(Box<Value>),
    Not(Box<Value>),
    Inverted(Box<Value>),
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        use self::Value::*;
        match self {
            Ident(ref ident) => ident.fmt(f),
            Literal(ref literal) => literal.fmt(f),
            Expr(ref expr) => write!(f, "({})", expr),
            Call(ref call) => call.fmt(f),
            Negative(ref inner) => write!(f, "-{}", inner),
            Not(ref inner) => write!(f, "!{}", inner),
            Inverted(ref inner) => write!(f, "~{}", inner),
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
            [Token::Minus, _] => {
                tokens.skip(1);
                Ok(Value::Negative(box Value::parse(tokens)?))
            },
            [Token::Bang, _] => {
                tokens.skip(1);
                Ok(Value::Not(box Value::parse(tokens)?))
            },
            [Token::Inv, _] => {
                tokens.skip(1);
                Ok(Value::Inverted(box Value::parse(tokens)?))
            }
            _ => {
                eprintln!("Failed to match Value: {:?}", tokens.peek());
                Err(Error::ParseError(ParseError::ExpectedValue))
            }
        }
    }
}
