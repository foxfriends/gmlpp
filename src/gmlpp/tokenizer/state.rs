use error::Error;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum State {
    Start,

    // comments
    SlashSlash,
    SlashSlashSlash,
    LineComment,
    SlashStar(u8),
    SlashStarSlash(u8),
    SlashStarStar(u8),

    // numbers
    Zero,
    ZeroX,
    ZeroB,
    Hex,
    Bin,
    Dec,
    DecFloat,
    DecE,
    DecEMinus,
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

            // numbers

            // 0
            Zero =>
                match c {
                    'x' => Ok(Some(ZeroX)),
                    'b' => Ok(Some(ZeroB)),
                    '_' => Ok(Some(Dec)),
                    '.' => Ok(Some(DecFloat)),
                    'e' => Ok(Some(DecE)),
                    c if c.is_digit(10) => Ok(Some(Dec)),
                    c if !c.is_alphanumeric() => Ok(None),
                    _ => Err(Error::MalformedNumericLiteral),
                }

            // 0x
            ZeroX =>
                match c {
                    '_' => Ok(Some(ZeroX)),
                    c if c.is_digit(16) => Ok(Some(Hex)),
                    _ => Err(Error::MalformedNumericLiteral),
                }

            // 0x1
            Hex =>
                match c {
                    '_' => Ok(Some(Hex)),
                    c if c.is_digit(16) => Ok(Some(Hex)),
                    c if !c.is_alphanumeric() => Ok(None),
                    _ => Err(Error::MalformedNumericLiteral),
                }

            // 0b
            ZeroB =>
                match c {
                    '_' => Ok(Some(ZeroB)),
                    c if c.is_digit(2) => Ok(Some(Bin)),
                    _ => Err(Error::MalformedNumericLiteral),
                }

            // 0b1
            Bin =>
                match c {
                    '_' => Ok(Some(Bin)),
                    c if c.is_digit(2) => Ok(Some(Bin)),
                    c if !c.is_alphanumeric() => Ok(None),
                    _ => Err(Error::MalformedNumericLiteral)
                }

            // 1
            Dec =>
                match c {
                    '_' => Ok(Some(Dec)),
                    '.' => Ok(Some(DecFloat)),
                    'e' => Ok(Some(DecE)),
                    c if c.is_digit(10) => Ok(Some(Dec)),
                    c if !c.is_alphanumeric() => Ok(None),
                    _ => Err(Error::MalformedNumericLiteral)
                }

            // 1.
            DecFloat =>
                match c {
                    '_' => Ok(Some(DecFloat)),
                    'e' => Ok(Some(DecE)),
                    c if c.is_digit(10) => Ok(Some(DecFloat)),
                    c if !c.is_alphanumeric() => Ok(None),
                    _ => Err(Error::MalformedNumericLiteral),
                }

            // 1.3e
            DecE =>
                match c {
                    '-' => Ok(Some(DecEMinus)),
                    c if c.is_digit(10) => Ok(Some(DecExp)),
                    _ => Err(Error::MalformedNumericLiteral),
                }

            // 1.3e-
            DecEMinus =>
                match c {
                    c if c.is_digit(10) => Ok(Some(DecExp)),
                    _ => Err(Error::MalformedNumericLiteral),
                }

            // 1.3e-5
            DecExp =>
                match c {
                    '_' => Ok(Some(DecExp)),
                    c if c.is_digit(10) => Ok(Some(DecExp)),
                    c if !c.is_alphanumeric() => Ok(None),
                    _ => Err(Error::MalformedNumericLiteral),
                }

            // operators

            // /
            Slash =>
                match c {
                    '/' => Ok(Some(SlashSlash)),
                    '=' => Ok(Some(SlashEqual)),
                    '*' => Ok(Some(SlashStar(1))),
                    '(' | '-' | '.' | '_' => Ok(None),
                    c if c.is_whitespace() => Ok(None),
                    c if c.is_alphanumeric() => Ok(None),
                    _ => Err(Error::UnexpectedCharacter),
                }

            // comments

            // //
            SlashSlash =>
                match c {
                    '/' => Ok(Some(SlashSlashSlash)),
                    '\n' => Ok(None),
                    _ => Ok(Some(LineComment)),
                }

            // ///
            SlashSlashSlash =>
                match c {
                    '\n' => Ok(None),
                    _ => Ok(Some(SlashSlashSlash)),
                }

            // //a
            LineComment =>
                match c {
                    '\n' => Ok(None),
                    _ => Ok(Some(LineComment)),
                }

            // /*
            SlashStar(depth) =>
                if depth == 0 {
                    Ok(None)
                } else {
                    match c {
                        '/' => Ok(Some(SlashStarSlash(depth))),
                        '*' => Ok(Some(SlashStarStar(depth))),
                        _ => Ok(Some(SlashStar(depth))),
                    }
                }

            // /* *
            SlashStarStar(depth) =>
                match c {
                    '/' => Ok(Some(SlashStar(depth - 1))),
                    '*' => Ok(Some(SlashStarStar(depth))),
                    _ => Ok(Some(SlashStar(depth))),
                }

            // /* *
            SlashStarSlash(depth) =>
                match c {
                    '/' => Ok(Some(SlashStarSlash(depth))),
                    '*' => 
                        if depth == ::std::u8::MAX {
                            Err(Error::CommentNestingDepth)
                        } else {
                            Ok(Some(SlashStar(depth + 1)))
                        }
                    _ => Ok(Some(SlashStar(depth))),
                }

            // the rest
            _ => Ok(None)
        }
    }
}

