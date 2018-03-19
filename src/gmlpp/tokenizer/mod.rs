use std::io::Read;

use error::Error;

mod token;
mod state;

pub use self::token::Token;
use self::state::State;

pub fn tokenize<R>(reader: R) -> Result<Vec<Token>, Error>
where R: Read {
    let chars = reader.chars();
    let mut state = State::default();
    let mut token = "".to_owned();
    let mut tokens = vec![Token::BOF];
    for ch in chars {
        let c = ch?;
        match state.next(c)? {
            Some(next) => {
                state = next;
                if state != State::default() {
                    token.push(c);
                }
            }
            None => {
                tokens.push(Token::new(state, token.clone()));
                token.clear();
                state = State::default().next(c)?.unwrap();
                if state != State::default() {
                    token = c.to_string();
                }
            }
        }
    }
    if state != State::default() {
        match state.next('\n')? {
            Some(_) => return Err(Error::UnexpectedEOF),
            None => { tokens.push(Token::new(state, token)) }
        }
    }
    tokens.push(Token::EOF);
    Ok(tokens)
}
