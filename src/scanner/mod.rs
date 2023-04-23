use crate::token::Token;

pub struct Scanner {
    current: usize,
    line: usize,
    source: String,
    start: usize,
    tokens: Vec<Token>,
}

impl Scanner {
    fn is_at_end(self) -> bool {
        self.current >= self.source.len()
    }
    pub fn new(source: String) -> Scanner {
        Scanner { source }
    }

    pub fn scan_tokens(self) -> Vec<Token> {
        vec![]
    }
}
