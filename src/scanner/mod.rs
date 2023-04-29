use crate::token::{Token, TokenKind};

pub struct Scanner {
    current: usize,
    line: usize,
    source: String,
    start: usize,
    tokens: Vec<Token>,
}

impl Scanner {
    fn add_token(&mut self, kind: TokenKind) -> () {
        let substr = &self.source[self.start..self.current];

        self.tokens
            .push(Token::new(substr.to_string(), kind, self.line));
    }

    fn advance(&mut self) -> Option<char> {
        self.current += 1;

        let res = self.source.chars().nth(self.current - 1);

        res
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    pub fn new(source: String) -> Scanner {
        Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    fn scan_token(self) {}

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens
            .push(Token::new("".to_string(), TokenKind::EOF, self.line));

        self.tokens.clone()
    }
}
