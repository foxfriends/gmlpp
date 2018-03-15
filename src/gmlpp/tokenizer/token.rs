/// All possible tokens from a gmlpp program
#[derive(Clone, Eq, PartialEq, Debug)]
pub enum Token {
    // values
    Identifier(String),
    BinLiteral(String),
    OctLiteral(String),
    HexLiteral(String),
    DecLiteral(String),
    StrLiteral(String),
    CharLiteral(String),
    BoolLiteral(String),
    UndefinedLiteral,

    // comments
    Comment,
    DocComment,
    BlockCommentStart,
    BlockCommentEnd,

    // bitwise operators
    And,
    Or,
    Xor,
    Inv,
    LShift,
    RShift,
    AndEqual,
    OrEqual,
    XorEqual,
    LShiftEqual,
    RShiftEqual,

    // boolean operators
    BAnd,
    BOr,
    Bang,

    // arithmetic operators
    Plus,
    Minus,
    Star,
    Slash,
    Pct,
    Exp,
    PlusEqual,
    MinusEqual,
    StarEqual,
    SlashEqual,
    PctEqual,
    ExpEqual,
    PlusPlus,
    MinusMinus,

    // comparison operators
    Equal,
    Less,
    More,
    NotEqual,
    LessEqual,
    MoreEqual,
    Assign,

    // accessors
    Hash,
    At,

    // control flow
    Question,
    Colon,
    Pipe,
    Placeholder,
    Underscore,

    // delimiters
    LBrack, 
    RBrack,
    LParen, 
    RParen,
    RBrace,
    LBrace,
    Comma,
    Dot,
    Semi,
    EOL,
    BOF,
    EOF,

    // keywords
    For,
    Do,
    While,
    Until,
    Repeat,
    Loop,
    With,
    If,
    Else,
    Switch,
    Case,
    Default,
    Break,
    Return,
    Exit,
    Var,
    Globalvar,
    Enum,
    Global,
    HashMacro,
    HashPragma,
    Div,
    Mod,
    Argument,

    // reserved words (keywords)
    Public,
    Protected,
    Private,
    Let,
    Const,
    Property,
    Method,
    Function,
    Local,
    Struct,
    Class,
    Trait,
    Interface,
    Protocol,
    Extension,
    Implementation,
    Type,
    Data,
    In,
    Is,
    Of,
    TypeOf,
    InstanceOf,
    Match,
    Otherwise,
    Throw,
    Catch,
    Try,
    Unreachable,
    Null,
    
    // reserved words (types)
    TBool,
    TNumber,
    TString,
    TChar,
    TArray,
    TSymbol,
    TVoid,
    TNull,
    TNever,
    TMap,
    TList,
    TGrid,
    TObject,
    TRoom,
    TSprite,
    TScript,
    TPath,
    TTileSet,
    TSound,
    TFont,
    TTimeline,

    // future symbols
    TemplateLiteral(String),
    MatchEqual,
}
