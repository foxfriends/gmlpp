use std::fmt::{self, Display, Formatter};

use super::value::Value;
use super::call::Call;
use super::fragment::Fragment;
use super::super::tokenizer::{Token, Tokens};
use error::{Error, ParseError};

#[derive(Clone, Debug)]
pub enum Expression {
    Plus(Box<Expression>, Box<Expression>),
    Minus(Box<Expression>, Box<Expression>),
    Times(Box<Expression>, Box<Expression>),
    Slash(Box<Expression>, Box<Expression>),
    Pct(Box<Expression>, Box<Expression>),
    Exp(Box<Expression>, Box<Expression>),
    Mod(Box<Expression>, Box<Expression>),
    Div(Box<Expression>, Box<Expression>),
    LShift(Box<Expression>, Box<Expression>),
    RShift(Box<Expression>, Box<Expression>),
    And(Box<Expression>, Box<Expression>),
    Or(Box<Expression>, Box<Expression>),
    Xor(Box<Expression>, Box<Expression>),
    BAnd(Box<Expression>, Box<Expression>),
    BOr(Box<Expression>, Box<Expression>),
    Eq(Box<Expression>, Box<Expression>),
    Neq(Box<Expression>, Box<Expression>),
    Lt(Box<Expression>, Box<Expression>),
    Gt(Box<Expression>, Box<Expression>),
    Leq(Box<Expression>, Box<Expression>),
    Geq(Box<Expression>, Box<Expression>),
    Pipe(Box<Expression>, Call),
    Ternary(Box<Expression>, Box<Expression>, Box<Expression>),
    Value(Value),
}

impl Display for Expression {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        use self::Expression::*;
        match self {
            Plus(ref lhs, ref rhs) => write!(f, "{} + {}", lhs, rhs),
            Minus(ref lhs, ref rhs) => write!(f, "{} - {}", lhs, rhs),
            Times(ref lhs, ref rhs) => write!(f, "{} * {}", lhs, rhs),
            Slash(ref lhs, ref rhs) => write!(f, "{} / {}", lhs, rhs),
            Pct(ref lhs, ref rhs) => write!(f, "{} % {}", lhs, rhs),
            Exp(ref lhs, ref rhs) => write!(f, "{} ** {}", lhs, rhs),
            Div(ref lhs, ref rhs) => write!(f, "{} div {}", lhs, rhs),
            Mod(ref lhs, ref rhs) => write!(f, "{} mod {}", lhs, rhs),
            LShift(ref lhs, ref rhs) => write!(f, "{} << {}", lhs, rhs),
            RShift(ref lhs, ref rhs) => write!(f, "{} >> {}", lhs, rhs),
            And(ref lhs, ref rhs) => write!(f, "{} & {}", lhs, rhs),
            Or(ref lhs, ref rhs) => write!(f, "{} | {}", lhs, rhs),
            Xor(ref lhs, ref rhs) => write!(f, "{} ^ {}", lhs, rhs),
            BAnd(ref lhs, ref rhs) => write!(f, "{} && {}", lhs, rhs),
            BOr(ref lhs, ref rhs) => write!(f, "{} || {}", lhs, rhs),
            Eq(ref lhs, ref rhs) => write!(f, "{} == {}", lhs, rhs),
            Neq(ref lhs, ref rhs) => write!(f, "{} != {}", lhs, rhs),
            Lt(ref lhs, ref rhs) => write!(f, "{} < {}", lhs, rhs),
            Gt(ref lhs, ref rhs) => write!(f, "{} > {}", lhs, rhs),
            Leq(ref lhs, ref rhs) => write!(f, "{} <= {}", lhs, rhs),
            Geq(ref lhs, ref rhs) => write!(f, "{} >= {}", lhs, rhs),
            Pipe(ref lhs, ref rhs) => write!(f, "{} |> {}", lhs, rhs),
            Ternary(ref cond, ref first, ref second) => write!(f, "{} ? {} : {}", cond, first, second),
            Value(ref val) => write!(f, "{}", val),
        }
    }
}

impl Fragment for Expression {
    fn parse(tokens: &Tokens) -> Result<Self, Error> {
        let value = Value::parse(tokens)?;
        let pos = tokens.pos();
        match Self::prec_parse(tokens, Expression::Value(value), Precedence::default()) {
            Err(error) => {
                tokens.seek(pos);
                eprintln!("Failed to match Expression: {}", error);
                Err(error)
            }
            Ok(expr) => Ok(expr),
        }
    }
}

impl Expression {
    fn prec_parse(tokens: &Tokens, lhs: Expression, min_prec: Precedence) -> Result<Self, Error> {
        let op = tokens.peek();
        match Precedence::of(&op) {
            Some(op_prec) if op_prec >= min_prec => {
                tokens.skip(1);
                if op == Token::Pipe {
                    // the pipe is a special case because the rhs needs to be just one call
                    let pipe = Expression::Pipe(box lhs, Call::parse(tokens)?);
                    Self::prec_parse(tokens, pipe, op_prec)
                } else if op == Token::Question {
                    // the question is special because it has three components
                    let first = Expression::parse(tokens)?;
                    if tokens.next() != Token::Colon { return Err(Error::ParseError(ParseError::IncompleteTernaryOperator)) }
                    let second = Expression::parse(tokens)?;
                    let ternary = Expression::Ternary(box lhs, box first, box second);
                    Self::prec_parse(tokens, ternary, op_prec)
                } else {
                    let mut rhs = Expression::Value(Value::parse(tokens)?);
                    loop {
                        match Precedence::of(&tokens.peek()) {
                            Some(peek_prec) if peek_prec > op_prec =>
                                rhs = Self::prec_parse(tokens, rhs, peek_prec)?,
                            _ => break,
                        }
                    }
                    match op {
                        Token::Plus => Ok(Expression::Plus(box lhs, box rhs)),
                        Token::Minus => Ok(Expression::Minus(box lhs, box rhs)),
                        Token::Slash => Ok(Expression::Slash(box lhs, box rhs)),
                        Token::Star => Ok(Expression::Times(box lhs, box rhs)),
                        Token::Pct => Ok(Expression::Pct(box lhs, box rhs)),
                        Token::Exp => Ok(Expression::Exp(box lhs, box rhs)),
                        Token::NotEqual => Ok(Expression::Neq(box lhs, box rhs)),
                        Token::Equal => Ok(Expression::Eq(box lhs, box rhs)),
                        Token::Less => Ok(Expression::Lt(box lhs, box rhs)),
                        Token::More => Ok(Expression::Gt(box lhs, box rhs)),
                        Token::LessEqual => Ok(Expression::Leq(box lhs, box rhs)),
                        Token::MoreEqual => Ok(Expression::Geq(box lhs, box rhs)),
                        Token::Div => Ok(Expression::Div(box lhs, box rhs)),
                        Token::Mod => Ok(Expression::Mod(box lhs, box rhs)),
                        Token::LShift => Ok(Expression::LShift(box lhs, box rhs)),
                        Token::RShift => Ok(Expression::RShift(box lhs, box rhs)),
                        Token::Or => Ok(Expression::Or(box lhs, box rhs)),
                        Token::Xor => Ok(Expression::Xor(box lhs, box rhs)),
                        Token::And => Ok(Expression::And(box lhs, box rhs)),
                        Token::BAnd => Ok(Expression::BAnd(box lhs, box rhs)),
                        Token::BOr => Ok(Expression::BOr(box lhs, box rhs)),
                        _ => panic!("Unreachable! {:?} is not an operator", op),
                    }
                }
            }
            _ => Ok(lhs),
        }
    }
}

#[derive(Eq, Ord, PartialEq, PartialOrd, Clone, Copy)]
enum Precedence {
    Zero,
    Pipe,
    Ternary,
    BoolOr,
    BoolAnd,
    Comparison,
    BitOr,
    BitXor,
    BitAnd,
    Shift,
    DivMod,
    AddSub,
    MultDivPct,
    Exp,
    #[allow(dead_code)] Max,
}

impl Default for Precedence {
    fn default() -> Self { Precedence::Zero }
}

impl Precedence {
    fn of(token: &Token) -> Option<Self> {
        use self::Precedence::*;
        match token {
            Token::Pipe => Some(Pipe),
            Token::Question | Token::Colon => Some(Ternary),
            Token::BOr => Some(BoolOr),
            Token::BAnd => Some(BoolAnd),
            Token::Equal | Token::NotEqual | Token::Less | Token::More | Token::LessEqual | Token::MoreEqual => Some(Comparison),
            Token::Or => Some(BitOr),
            Token::Xor => Some(BitXor),
            Token::And => Some(BitAnd),
            Token::LShift | Token::RShift => Some(Shift),
            Token::Div | Token::Mod => Some(DivMod),
            Token::Plus | Token::Minus => Some(AddSub),
            Token::Star | Token::Slash | Token::Pct => Some(MultDivPct),
            Token::Exp => Some(Exp),
            _ => None,
        }
    }
}
