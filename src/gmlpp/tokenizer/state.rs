use error::Error;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum State {
    Start,

    // comments
    SlashSlash,
    SlashSlashSlash,
    LineComment,
    DocComment,
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
    PlusPlus,
    MinusMinus,

    // comparison operators
    Equal,
    EqualEqual,
    NotEqual,
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
    BarEqual,
    XorEqual,
    Tilde,
    Bang,
    LShift,
    LShiftEqual,
    RShift,
    RShiftEqual,

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
    DotDot,
    DotDotDot,

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
                    _ => Ok(None),
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
                    _ => Ok(Some(DocComment)),
                }

            // //a
            LineComment =>
                match c {
                    '\n' => Ok(None),
                    _ => Ok(Some(LineComment)),
                }

            // ///a
            DocComment =>
                match c {
                    '\n' => Ok(None),
                    _ => Ok(Some(DocComment)),
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

            // '
            CharStart =>
                match c {
                    '\\' => Ok(Some(CharSlash)),
                    '\'' => Err(Error::UnexpectedCharacter),
                    _ => Ok(Some(CharX)),
                }

            // 'a
            CharX =>
                match c {
                    '\'' => Ok(Some(Char)),
                    _ => Err(Error::UnexpectedCharacter),
                }

            // '\
            CharSlash => Ok(Some(CharX)),

            // 'a'
            Char => Ok(None),

            // "
            StrStart =>
                match c {
                    '"' => Ok(Some(Str)),
                    '\\' => Ok(Some(StrSlash)),
                    _ => Ok(Some(StrStart)),
                }

            // "xx\
            StrSlash => Ok(Some(StrStart)),

            // "xxx"
            Str => Ok(None),

            // +
            Plus =>
                match c {
                    '+' => Ok(Some(PlusPlus)),
                    '=' => Ok(Some(PlusEqual)),
                    _ => Ok(None),
                }

            // ++
            PlusPlus => Ok(None),

            // +=
            PlusEqual => Ok(None),

            // -
            Minus =>
                match c {
                    '-' => Ok(Some(MinusMinus)),
                    '=' => Ok(Some(MinusEqual)),
                    _ => Ok(None),
                }

            // --
            MinusMinus => Ok(None),

            // -=
            MinusEqual => Ok(None),

            // *
            Star =>
                match c {
                    '*' => Ok(Some(StarStar)),
                    '=' => Ok(Some(StarEqual)),
                    _ => Ok(None),
                },

            // **
            StarStar =>
                match c {
                    '=' => Ok(Some(StarStarEqual)),
                    _ => Ok(None),
                },

            // *=
            StarEqual => Ok(None),

            StarStarEqual => Ok(None),

            // /=
            SlashEqual => Ok(None),

            // %
            Percent =>
                match c {
                    '=' => Ok(Some(PercentEqual)),
                    _ => Ok(None),
                }

            // %=
            PercentEqual => Ok(None),

            // |
            Bar =>
                match c {
                    '|' => Ok(Some(BarBar)),
                    '=' => Ok(Some(BarEqual)),
                    '>' => Ok(Some(BarMore)),
                    _ => Ok(None),
                }

            // ||
            BarBar => Ok(None),

            // |=
            BarEqual => Ok(None),

            // &
            And =>
                match c {
                    '&' => Ok(Some(AndAnd)),
                    '=' => Ok(Some(AndEqual)),
                    _ => Ok(None),
                }

            // &&
            AndAnd => Ok(None),

            // &=
            AndEqual => Ok(None),

            // ^
            Xor =>
                match c {
                    '=' => Ok(Some(XorEqual)),
                    _ => Ok(None),
                }

            // ^=
            XorEqual => Ok(None),

            // ~
            Tilde => Ok(None),

            // !
            Bang =>
                match c {
                    '=' => Ok(Some(NotEqual)),
                    _ => Ok(None),
                }

            // !=
            NotEqual => Ok(None),

            // =
            Equal =>
                match c {
                    '=' => Ok(Some(EqualEqual)),
                    _ => Ok(None),
                }

            // ==
            EqualEqual => Ok(None),

            // <
            Less =>
                match c {
                    '<' => Ok(Some(LShift)),
                    '=' => Ok(Some(LessEqual)),
                    _ => Ok(None),
                }

            // <<
            LShift =>
                match c {
                    '=' => Ok(Some(LShiftEqual)),
                    _ => Ok(None),
                }

            // <<=
            LShiftEqual => Ok(None),

            // <=
            LessEqual => Ok(None),

            // >
            More =>
                match c {
                    '>' => Ok(Some(RShift)),
                    '=' => Ok(Some(MoreEqual)),
                    _ => Ok(None),
                }

            // >>
            RShift =>
                match c {
                    '=' => Ok(Some(RShiftEqual)),
                    _ => Ok(None),
                },

            // >>=
            RShiftEqual => Ok(None),

            // >=
            MoreEqual => Ok(None),

            Question => Ok(None),

            Colon => Ok(None),

            Hash =>
                match c {
                    'm' => Ok(Some(HashM)),
                    'p' => Ok(Some(HashP)),
                    _ => Ok(None),
                },

            At => Ok(None),

            Underscore =>
                match c {
                    c if c.is_alphanumeric() => Ok(Some(Identifier)),
                    _ => Ok(None),
                }

            BarMore => Ok(None),

            Dot =>
                match c {
                    '.' => Ok(Some(DotDot)),
                    c if c.is_numeric() => Ok(Some(DecFloat)),
                    _ => Ok(None),
                }

            DotDot =>
                match c {
                    '.' => Ok(Some(DotDotDot)),
                    _ => Err(Error::UnexpectedCharacter),
                }

            DotDotDot =>
                match c {
                    '.' => Err(Error::UnexpectedCharacter),
                    _ => Ok(None),
                }

            Comma => Ok(None),

            Semi => Ok(None),

            // preprocessor
            HashM => if c == 'a' { Ok(Some(HashMa)) } else { Err(Error::InvalidPreprocessorDirective) }
            HashMa => if c == 'c' { Ok(Some(HashMac)) } else { Err(Error::InvalidPreprocessorDirective) },
            HashMac => if c == 'r' { Ok(Some(HashMacr)) } else { Err(Error::InvalidPreprocessorDirective) },
            HashMacr => if c == 'o' { Ok(Some(HashMacro)) } else { Err(Error::InvalidPreprocessorDirective) },
            HashMacro =>
                match c {
                    '_' => Err(Error::InvalidPreprocessorDirective),
                    c if c.is_alphanumeric() => Err(Error::InvalidPreprocessorDirective),
                    _ => Ok(None),
                }
            HashP => if c == 'r' { Ok(Some(HashPr)) } else { Err(Error::InvalidPreprocessorDirective) },
            HashPr => if c == 'a' { Ok(Some(HashPra)) } else { Err(Error::InvalidPreprocessorDirective) },
            HashPra => if c == 'g' { Ok(Some(HashPrag)) } else { Err(Error::InvalidPreprocessorDirective) },
            HashPrag => if c == 'm' { Ok(Some(HashPragm)) } else { Err(Error::InvalidPreprocessorDirective) },
            HashPragm => if c == 'a' { Ok(Some(HashPragma)) } else { Err(Error::InvalidPreprocessorDirective) },
            HashPragma =>
                match c {
                    '_' => Err(Error::InvalidPreprocessorDirective),
                    c if c.is_alphanumeric() => Err(Error::InvalidPreprocessorDirective),
                    _ => Ok(None),
                },

            // whitespace
            EOL => if c.is_whitespace() { Ok(Some(EOL)) } else { Ok(None) },

            // brackets
            LParen => Ok(None),
            RParen => Ok(None),
            LBrack => Ok(None),
            RBrack => Ok(None),
            LBrace => Ok(None),
            RBrace => Ok(None),

            Identifier =>
                match c {
                    '_' => Ok(Some(Identifier)),
                    c if c.is_alphanumeric() => Ok(Some(Identifier)),
                    _ => Ok(None),
                },
        }
    }
}
