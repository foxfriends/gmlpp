use std::error::Error;
use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub enum ParseError {
    ExpectedValue,
    ExpectedLiteral,
    ExpectedIdentifier,
    ExpectedFunctionCall,
    ExpectedArgument,
    ExpectedKeyword,
    ExpectedStatement,
    ExpectedParentheses,
    IncompleteTernaryOperator,
    MismatchedParentheses,
    ExpectedEndOfStatement,
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
            ExpectedValue => "Expected value",
            ExpectedLiteral => "Expected literal",
            ExpectedIdentifier => "Expected identifier",
            ExpectedFunctionCall => "Expected function call",
            ExpectedArgument => "Expected argument",
            ExpectedParentheses => "Expected parentheses",
            ExpectedKeyword => "Expected keyword",
            ExpectedStatement => "Expected statement",
            IncompleteTernaryOperator => "Incomplete ternary operator",
            MismatchedParentheses => "Mismatched parentheses",
            ExpectedEndOfStatement => "Expected end of statement",
        }
    }
}
