#[derive(Debug)]
pub enum TokenKind {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    Bang,
    BangEqual,
    Greater,
    GreaterEqual,
    Equal,
    EqualEqual,
    Less,
    LessEqual,

    Identifier,
    String,
    Number,

    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    EOF,
}

#[derive(Debug)]
pub struct Token {
    value: String,
    kind: TokenKind,
    line: usize,
}

impl Token {
    pub fn new(value: String, kind: TokenKind, line: usize) -> Token {
        Token { value, kind, line }
    }
}
