use std::fmt::{Display, Debug};

use super::super::tokenizer::Tokens;
use error::Error;

pub trait Fragment: Display + Debug + Clone { 
    fn parse(tokens: &Tokens) -> Result<Self, Error>;
}
