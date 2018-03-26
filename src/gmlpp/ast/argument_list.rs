use std::fmt::{self, Display, Formatter};

use super::identifier::Identifier;
use super::expression::Expression;
use super::fragment::Fragment;
use super::super::tokenizer::{Token, Tokens};
use error::Error;

#[derive(Clone, Debug)]
pub enum ArgumentList {
    Argument(Identifier, Box<ArgumentList>),
    DefaultArgument(Identifier, Expression, Box<ArgumentList>),
    OptionalArgument(Identifier, Box<ArgumentList>),
    VariadicArgument(Identifier),
    End,
}

impl Display for ArgumentList {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        use self::ArgumentList::*;
        match self {
            &Argument(ref ident, ref rest) => write!(f, "argument {}\n{}", ident, rest),
            &DefaultArgument(ref ident, ref expr, ref rest) => write!(f, "argument {} = {}\n{}", ident, expr, rest),
            &OptionalArgument(ref ident, ref rest) => write!(f, "argument {}?\n{}", ident, rest),
            &VariadicArgument(ref ident) => write!(f, "argument ...{}\n", ident),
            &End => write!(f, ""),
        }
    }
}

impl Fragment for ArgumentList {
    fn parse(tokens: &Tokens) -> Result<Self, Error> {
        use self::ArgumentList::*;
        match tokens[..2] {
            [Token::Argument, Token::DotDotDot] => {
                tokens.skip(2);
                let ident = Identifier::parse(tokens)?;
                Ok(VariadicArgument(ident))
            }
            [Token::Argument, ..] => {
                tokens.skip(1);
                let ident = Identifier::parse(tokens)?;
                match tokens.peek() {
                    Token::Question => {
                        tokens.skip(1);
                        let rest = Self::parse(tokens)?;
                        Ok(OptionalArgument(ident, box rest))
                    }
                    Token::Equal => {
                        tokens.skip(1);
                        let expr = Expression::parse(tokens)?;
                        let rest = Self::parse(tokens)?;
                        Ok(DefaultArgument(ident, expr, box rest))
                    }
                    _ => Ok(Argument(ident, box Self::parse(tokens)?))
                }
            }
            _ => Ok(End),
        }
    }
}
