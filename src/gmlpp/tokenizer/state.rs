use error::Error;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum State {
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
    CharStart,
    CharX,
    CharSlash,
    Char,
    StrStart,
    StrSlash,
    Str,

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

    // preprocessor
    HashM,
    HashMa,
    HashMac,
    HashMacr,
    HashMacro,
    HashP,
    HashPr,
    HashPra,
    HashPrag,
    HashPragm,
    HashPragma,

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
}

impl Default for State {
    fn default() -> Self {
        State::Start
    }
}

impl State {
    /// transitions to the next state, given a character
    pub fn next(self, c: char) -> Result<Option<Self>, Error> {
        use self::State::*;
        match self {
            Start => 
                match c {
                    '0' => Ok(Some(Zero)),
                    '(' => Ok(Some(LParen)),
                    ')' => Ok(Some(RParen)),
                    '{' => Ok(Some(LBrace)),
                    '}' => Ok(Some(RBrace)),
                    '[' => Ok(Some(LBrack)),
                    ']' => Ok(Some(RBrack)),
                    '+' => Ok(Some(Plus)),
                    '-' => Ok(Some(Minus)),
                    '*' => Ok(Some(Star)),
                    '/' => Ok(Some(Slash)),
                    '%' => Ok(Some(Percent)),
                    '=' => Ok(Some(Equal)),
                    '|' => Ok(Some(Bar)),
                    '&' => Ok(Some(And)),
                    '^' => Ok(Some(Xor)),
                    '~' => Ok(Some(Tilde)),
                    '<' => Ok(Some(Less)),
                    '>' => Ok(Some(More)),
                    '!' => Ok(Some(Bang)),
                    '#' => Ok(Some(Hash)),
                    '@' => Ok(Some(At)),
                    '.' => Ok(Some(Dot)),
                    ',' => Ok(Some(Comma)),
                    '\n' => Ok(Some(EOL)),
                    '\'' => Ok(Some(CharStart)),
                    '"' => Ok(Some(StrStart)),
                    ';' => Ok(Some(Semi)),
                    '_' => Ok(Some(Underscore)),
                    '?' => Ok(Some(Question)),
                    ':' => Ok(Some(Colon)),
                    c if c.is_whitespace() => Ok(Some(Start)),
                    c if c.is_digit(10) => Ok(Some(Dec)),
                    c if c.is_alphabetic() => Ok(Some(Identifier)),
                    _ => Err(Error::InvalidCharacter),
                }
            _ => Ok(None)
        }
    }
}

