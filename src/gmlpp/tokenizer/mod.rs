use std::io::Read;

mod token;

pub use self::token::Token;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum State {
    Start,

    // comments
    SlashSlash,
    SlashSlashSlash,
    SlashStar,
    StarSlash,

    // numbers
    Zero,
    ZeroX,
    ZeroB,
    Hex,
    Bin,
    Dec,
    DecFloat,
    DecE,
    DecExp,

    // strings
    Char,
    Str,
    StrSlash,
    CharSlash,

    // arithmetic operators
    Minus,
    Plus,
    PlusEqual,
    MinusEqual,
    Star,
    StarStar,
    Slash,
    Percent,
    StarEqual,
    StarStarEqual,
    SlashEqual,
    PercentEqual,

    // comparison operators
    Equal,
    EqualEqual,
    Less,
    LessEqual,
    More,
    MoreEqual,

    // boolean operators
    And,
    AndAnd,
    Bar,
    BarBar,
    Xor,
    AndEqual,
    OrEqual,
    XorEqual,
    Tilde,
    Bang,

    // symbols
    Question,
    Colon,
    Hash,
    At,
    Underscore,
    BarMore,
    Dot,
    Comma,
    Semi,
    
    // whitespace
    EOL,

    // brackets
    LParen,
    RParen,
    LBrack,
    RBrack,
    LBrace,
    RBrace,

    // identifier
    Identifier,
    End,
}

impl Default for State {
    fn default() -> Self {
        State::Start
    }
}

/// Extracts tokens from a source file
#[derive(Clone, Debug)]
pub struct Tokenizer<'r, R> where R: Read + 'r {
    state: State,
    current_token: String,
    reader: &'r R,
}
