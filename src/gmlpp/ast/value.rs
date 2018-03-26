use std::fmt::{self, Display, Formatter};

use super::fragment::Fragment;
use super::identifier::Identifier;
use super::literal::Literal;
use super::expression::Expression;
use super::call::Call;
use super::super::tokenizer::Tokens;
use error::Error;

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
        unimplemented!()
    }
}
