#[derive(Debug)]
pub enum TokenKind {
    Line,
    BinaryOp,
}

#[derive(Debug)]
pub struct Token {
    value: String,
    kind: TokenKind,
}
