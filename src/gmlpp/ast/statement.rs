use std::fmt::{self, Display, Formatter};

use super::expression::Expression;
use super::identifier::Identifier;
use super::assignment::Assignment;
use super::fragment::Fragment;
use super::helpers::{semi_or_eol, parenthesized};
use super::super::tokenizer::{Token, Tokens};
use error::{Error, ParseError};

#[derive(Clone, Debug)]
pub enum Statement {
    Noop,
    Assignment(Assignment),
    Expression(Expression),
    VarDecl(Identifier),
    VarDeclAssign(Identifier, Expression),
    GlobalvarDecl(Identifier),
    GlobalvarDeclAssign(Identifier, Expression),
    EnumDecl(Identifier, Vec<()>), // TODO: enum cases
    If(Expression, Box<Statement>, Option<Box<Statement>>),
    While(Expression, Box<Statement>),
    DoWhile(Box<Statement>, Expression),
    Until(Expression, Box<Statement>),
    DoUntil(Box<Statement>, Expression),
    Repeat(Expression, Box<Statement>),
    For(Box<Statement>, Expression, Box<Statement>, Box<Statement>),
    Block(Vec<Statement>),
    Switch(Box<Expression>, Vec<()>), // TODO: switch cases
    Return(Option<Expression>),
    Break,
    Exit,
    Continue,
}

impl Statement {
    fn is_noop(&self) -> bool {
        match self {
            Statement::Noop => true,
            _ => false
        }
    }

    fn is_block(&self) -> bool {
        match self {
            Statement::Block(..) => true,
            _ => false
        }
    }
}

impl Display for Statement {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let indent = f.precision().unwrap_or(0);
        match self {
            Statement::Noop => writeln!(f, "{0:1$};", "", indent),
            Statement::Block(ref statements) => {
                writeln!(f, "{{")?;
                for statement in statements {
                    match statement {
                        // Don't print noops in a block. There's no point
                        Statement::Noop => continue,
                        _ => writeln!(f, "{0:.1$}", statement, indent + 4)?,
                    }
                }
                write!(f, "{0:1$}}}", "", indent)
            }
            Statement::Assignment(ref assignment) => writeln!(f, "{0:1$}{2};", "", indent, assignment),
            Statement::Expression(ref expression) => writeln!(f, "{0:1$}{2};", "", indent, expression),
            Statement::VarDecl(ref ident) => writeln!(f, "{0:1$}var {2};", "", indent, ident),
            Statement::VarDeclAssign(ref ident, ref value) => writeln!(f, "{0:1$}var {2} = {3};", "", indent, ident, value),
            Statement::GlobalvarDecl(ref ident) => writeln!(f, "{0:1$}globalvar {2};", "", indent, ident),
            Statement::GlobalvarDeclAssign(ref ident, ref value) => writeln!(f, "{0:1$}globalvar {2} = {3};", "", indent, ident, value),
            Statement::If(ref cond, ref body, None) if body.is_block() =>
                writeln!(f, "{0:1$}if ({2}) {3:.1$}", "", indent, cond, body),
            Statement::If(ref cond, ref body, None) => {
                writeln!(f, "{0:1$}if ({2})", "", indent, cond)?;
                writeln!(f, "{0:.1$}", body, indent + 4)
            }
            Statement::If(ref cond, ref body, Some(ref fbody)) => {
                if body.is_block() {
                    write!(f, "{0:1$}if ({2}) {3:.1$} ", "", indent, cond, body)?;
                } else {
                    writeln!(f, "{0:1$}if ({2})", "", indent, cond)?;
                    writeln!(f, "{0:.1$}", body, indent + 4)?;
                }
                if fbody.is_block() {
                    writeln!(f, "else {0:.1$}", fbody, indent)?;
                } else {
                    writeln!(f, "else")?;
                    writeln!(f, "{0:.1$}", fbody, indent + 4)?;
                }
                Ok(())
            }
            Statement::While(ref cond, box Statement::Noop) =>
                writeln!(f, "{0:1$}while ({2});", "", indent, cond),
            Statement::While(ref cond, ref body) if body.is_block() =>
                writeln!(f, "{0:1$}while ({2}) {3:.1$}", "", indent, cond, body),
            Statement::While(ref cond, ref body) => {
                writeln!(f, "{0:1$}while ({2})", "", indent, cond)?;
                writeln!(f, "{0:.1$}", body, indent)
            }
            Statement::Until(ref cond, box Statement::Noop) =>
                writeln!(f, "{0:1$}until ({2});", "", indent, cond),
            Statement::Until(ref cond, ref body) if body.is_block() =>
                writeln!(f, "{0:1$}until ({2}) {3:.1$}", "", indent, cond, body),
            Statement::Until(ref cond, ref body) => {
                writeln!(f, "{0:1$}until ({2})", "", indent, cond)?;
                writeln!(f, "{0:.1$}", body, indent)
            }
            Statement::DoUntil(box Statement::Noop, ref cond) =>
                writeln!(f, "{0:1$}do; until ({2});", "", indent, cond),
            Statement::DoUntil(ref body, ref cond) if body.is_block() =>
                writeln!(f, "{0:1$}do {3:.1$} until ({2})", "", indent, cond, body),
            Statement::DoUntil(ref body, ref cond) => {
                writeln!(f, "{0:1$}do", "", indent)?;
                writeln!(f, "{0:.1$}", body, indent)?;
                writeln!(f, "{0:1$}until ({2})", "", indent, cond)
            }
            _ => unimplemented!(),
        }
    }
}

impl Fragment for Statement {
    fn parse(tokens: &Tokens) -> Result<Self, Error> {
        match tokens[..1] {
            [Token::Semi] => {
                semi_or_eol(tokens)?;
                Ok(Statement::Noop)
            }
            [Token::Var] | [Token::Globalvar] => {
                let var_type = tokens.next();
                let ident = Identifier::parse(tokens)?;
                if tokens.peek() == Token::Assign {
                    tokens.skip(1);
                    let value = Expression::parse(tokens)?;
                    semi_or_eol(tokens)?;
                    if var_type == Token::Var {
                        Ok(Statement::VarDeclAssign(ident, value))
                    } else {
                        Ok(Statement::GlobalvarDeclAssign(ident, value))
                    }
                } else {
                    semi_or_eol(tokens)?;
                    if var_type == Token::Var {
                        Ok(Statement::VarDecl(ident))
                    } else {
                        Ok(Statement::GlobalvarDecl(ident))
                    }
                }
            }
            [Token::If] => {
                tokens.skip(1);
                let cond = parenthesized::<Expression>(tokens)?;
                let body = Statement::parse(tokens)?;
                if tokens.peek() == Token::Else {
                    tokens.skip(1);
                    let fbody = Statement::parse(tokens)?;
                    Ok(Statement::If(cond, box body, Some(box fbody)))
                } else {
                    Ok(Statement::If(cond, box body, None))
                }
            }
            [Token::Do] => {
                tokens.skip(1);
                let body = Statement::parse(tokens)?;
                let loop_type = tokens.next();
                if loop_type != Token::While && loop_type != Token::Until {
                    return Err(Error::ParseError(ParseError::ExpectedKeyword));
                }
                let cond = parenthesized::<Expression>(tokens)?;
                if loop_type == Token::While {
                    Ok(Statement::DoWhile(box body, cond))
                } else {
                    Ok(Statement::DoUntil(box body, cond))
                }
            }
            // for loop is super sketchy. probably will not be so working
            [Token::For] => {
                tokens.skip(1);
                if tokens[0] != Token::LParen {
                    return Err(Error::ParseError(ParseError::ExpectedParentheses));
                }
                let init = Statement::parse(tokens)?;
                let cond = Expression::parse(tokens)?;
                let update = Statement::parse(tokens)?;
                if tokens.peek() != Token::RParen {
                    return Err(Error::ParseError(ParseError::MismatchedParentheses));
                }
                let body = Statement::parse(tokens)?;
                Ok(Statement::For(box init, cond, box update, box body))
            }
            [Token::Repeat] => {
                tokens.skip(1);
                let times = parenthesized::<Expression>(tokens)?;
                let body = Statement::parse(tokens)?;
                Ok(Statement::Repeat(times, box body))
            }
            [Token::While] | [Token::Until] => {
                let loop_type = tokens.next();
                let cond = parenthesized::<Expression>(tokens)?;
                let body = Statement::parse(tokens)?;
                if loop_type == Token::While {
                    Ok(Statement::While(cond, box body))
                } else {
                    Ok(Statement::Until(cond, box body))
                }
            }
            [Token::Break] => {
                tokens.skip(1);
                semi_or_eol(tokens)?;
                Ok(Statement::Break)
            }
            [Token::Continue] => {
                tokens.skip(1);
                semi_or_eol(tokens)?;
                Ok(Statement::Continue)
            }
            [Token::Exit] => {
                tokens.skip(1);
                semi_or_eol(tokens)?;
                Ok(Statement::Exit)
            }
            [Token::Return] => {
                tokens.skip(1);
                match semi_or_eol(tokens) {
                    Ok(..) => Ok(Statement::Return(None)),
                    Err(_) => {
                        let value = Expression::parse(tokens)?;
                        semi_or_eol(tokens)?;
                        Ok(Statement::Return(Some(value)))
                    }
                }
            }
            [Token::LBrace] => {
                let mut statements = Vec::new();
                tokens.skip(1);
                while tokens.peek() != Token::RBrace {
                    statements.push(Statement::parse(tokens)?);
                }
                tokens.skip(1);
                Ok(Statement::Block(statements))
            }
            _ => {
                let start = tokens.pos();
                // try for expression
                if let Ok(expr) = Expression::parse(tokens) {
                    if let Ok(()) = semi_or_eol(tokens) {
                        return Ok(Statement::Expression(expr));
                    }
                }
                tokens.seek(start);
                // try for assign
                if let Ok(assg) = Assignment::parse(tokens) {
                    if let Ok(()) = semi_or_eol(tokens) {
                        return Ok(Statement::Assignment(assg));
                    }
                }
                eprintln!("Failed to match Statement. {:?}", tokens[..1].to_vec());
                Err(Error::ParseError(ParseError::ExpectedStatement))
            }
        }
    }
}
