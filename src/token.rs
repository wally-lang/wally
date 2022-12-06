#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
    pub line: usize,
    pub column: usize,
}

impl Token {
    pub fn new(kind: TokenKind, lexeme: String, line: usize, column: usize) -> Self {
        Self { kind, lexeme, line, column }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // Keywords
    ArrayKw,            // array
    MapKw,              // map
    StringKw,           // string
    CharKw,             // char
    IntegerKw,          // integer
    FloatKw,            // float
    BoolKw,             // bool
    TrueKw,             // true
    FalseKw,            // false
    NullKw,             // null
    VarKw,              // var
    ConstKw,            // const
    FnKw,               // fn
    ClassKw,            // class
    ReturnKw,           // return

    // Operators
    Plus,               // +
    Minus,              // -
    Star,               // *
    Slash,              // /
    Percent,            // %
    Caret,              // ^
    Bang,               // !
    Equal,              // =
    EqualEqual,         // ==
    BangEqual,          // !=
    Greater,            // >
    GreaterEqual,       // >=
    Less,               // <
    LessEqual,          // <=
    And,                // &&
    Or,                 // ||
    LeftParen,          // (
    RightParen,         // )
    LeftBrace,          // {
    RightBrace,         // }
    LeftBracket,        // [
    RightBracket,       // ]
    Comma,              // ,
    Dot,                // .
    Colon,              // :
    Semicolon,          // ;
    Question,           // ?
    At,                 // @
    Hash,               // #
    Dollar,             // $
    Underscore,         // _
    Tilde,              // ~
    Pipe,               // |
    Ampersand,          // &

    // Literals
    Identifier(String),
    Integer(i64),
    Float(f64),
    String(String),
    Character(char),
    Boolean(bool),
    Null,

    // Special
    Error(String),
    Eof,
}