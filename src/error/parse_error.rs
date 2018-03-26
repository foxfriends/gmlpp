use std::error::Error;
use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub enum ParseError {
    ExpectedIdentifier,
    ExpectedFunctionCall,
    IncompleteTernaryOperator,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for ParseError {
    fn description(&self) -> &str {
        use self::ParseError::*;
        match self {
            &ExpectedIdentifier => "Expected identifier",
            &ExpectedFunctionCall => "Expected function call",
            &IncompleteTernaryOperator => "Incomplete ternary operator",
        }
    }
} 
