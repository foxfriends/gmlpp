use std::io::Read;

use error::Error;

mod token;
mod state;
mod tokens;

pub use self::token::Token;
pub use self::tokens::Tokens;
use self::state::State;

pub fn tokenize<R>(reader: R) -> Result<Tokens, Error>
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
            None | Some(State::EOL) => tokens.push(Token::new(state, token)),
            Some(_) => return Err(Error::UnexpectedEOF),
        }
    }
    tokens.push(Token::EOF);
    println!("{:?}", tokens);
    Ok(Tokens::new(
        tokens
            .into_iter()
            // Remove all comments because they're dumb
            .filter(|token|
                if let Token::Comment(..) = token {
                    false
                } else {
                    true
                }
            )
            .collect()
    ))
}
