use std::fmt::{self, Display, Formatter};

use super::expression::Expression;
use super::identifier::Identifier;
use super::fragment::Fragment;
use super::super::tokenizer::{Token, Tokens};
use error::{Error, ParseError};

#[derive(Clone, Debug)]
pub enum Sequence {
    VarDecl(Identifier, Box<Sequence>),
    VarDeclAssign(Identifier, Expression, Box<Sequence>),
    GlobalvarDecl(Identifier, Box<Sequence>),
    GlobalvarDeclAssign(Identifier, Expression, Box<Sequence>),

    End,
}
