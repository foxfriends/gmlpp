use std::fmt::{self, Display, Formatter};

use super::expression::Expression;
use super::identifier::Identifier;
use super::fragment::Fragment;
use super::super::tokenizer::{Token, Tokens};
use error::{Error, ParseError};

#[derive(Clone, Debug)]
pub enum Statements {
    VarDecl(Identifier, Box<Statements>),
    VarDeclAssign(Identifier, Expression, Box<Statements>),
    GlobalvarDecl(Identifier, Box<Statements>),
    GlobalvarDeclAssign(Identifier, Expression, Box<Statements>),

    End,
}

impl Display for Statements {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "")
    }
}

impl Fragment for Statements {
    fn parse(tokens: &Tokens) -> Result<Self, Error> { 
        Ok(Statements::End)
    }
}
