use std::fmt::{self, Display, Formatter};

use super::fragment::Fragment;
use super::lvalue::LValue;
use super::expression::Expression;
use super::super::tokenizer::{Token, Tokens};
use error::{Error, ParseError};

#[derive(Clone, Debug)]
pub enum Assignment {
    Eq(LValue, Expression)
}

impl Display for Assignment {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Assignment::Eq(ref lvalue, ref expr) => write!(f, "{} = {}", lvalue, expr),
        }
    }
}

impl Fragment for Assignment {
    fn parse(tokens: &Tokens) -> Result<Self, Error> {
        let lvalue = LValue::parse(tokens)?;
        let _op = tokens.next();
        let expr = Expression::parse(tokens)?;
        // TODO: use op, not just eq always
        Ok(Assignment::Eq(lvalue, expr))
    }
}
