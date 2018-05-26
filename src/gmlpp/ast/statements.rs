use std::fmt::{self, Display, Formatter};

use super::statement::Statement;
use super::fragment::Fragment;
use super::super::tokenizer::{Token, Tokens};
use error::Error;

/// The top level list of statements
#[derive(Clone, Debug)]
pub struct Statements(Vec<Statement>);

impl Display for Statements {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        for statement in &self.0 {
            match statement {
                // don't need to print noops at the top level
                Statement::Noop => continue,
                _ => writeln!(f, "{0:.0}", statement)?,
            }
        }
        Ok(())
    }
}

impl Fragment for Statements {
    fn parse(tokens: &Tokens) -> Result<Self, Error> {
        let mut statements = Vec::new();
        while tokens[0] != Token::EOF {
            statements.push(Statement::parse(tokens)?);
        }
        Ok(Statements(statements))
    }
}
