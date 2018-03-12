use std::io::Read;

mod token;
mod state;

pub use self::token::Token;
use self::state::State;

/// Extracts tokens from a source file
#[derive(Clone, Debug)]
pub struct Tokenizer<'r, R> where R: Read + 'r {
    state: State,
    current_token: String,
    reader: &'r R,
}
