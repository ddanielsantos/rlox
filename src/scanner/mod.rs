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

    fn scan_token(&mut self) -> () {
        let char = self.advance();

        if let Some(c) = char {
            match c {
                '(' => self.add_token(TokenKind::LeftParen),
                ')' => self.add_token(TokenKind::RightParen),
                '{' => self.add_token(TokenKind::LeftBrace),
                '}' => self.add_token(TokenKind::RightBrace),
                ',' => self.add_token(TokenKind::Comma),
                '.' => self.add_token(TokenKind::Dot),
                '-' => self.add_token(TokenKind::Minus),
                '+' => self.add_token(TokenKind::Plus),
                ';' => self.add_token(TokenKind::Semicolon),
                '*' => self.add_token(TokenKind::Star),
                '!' => match self.match_char('=') {
                    true => self.add_token(TokenKind::BangEqual),
                    false => self.add_token(TokenKind::Bang),
                },
                '=' => match self.match_char('=') {
                    true => self.add_token(TokenKind::EqualEqual),
                    false => self.add_token(TokenKind::Equal),
                },
                '<' => match self.match_char('=') {
                    true => self.add_token(TokenKind::LessEqual),
                    false => self.add_token(TokenKind::Less),
                },
                '>' => match self.match_char('=') {
                    true => self.add_token(TokenKind::GreaterEqual),
                    false => self.add_token(TokenKind::Greater),
                },
                _ => todo!(),
            }
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens
            .push(Token::new("".to_string(), TokenKind::EOF, self.line));

        self.tokens.clone()
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.source.chars().nth(self.current) != Some(expected) {
            return false;
        }

        self.current += 1;

        true
    }
}

#[cfg(test)]
mod tests {
    use super::{Scanner, Token, TokenKind};

    #[test]
    fn ids_empty() {
        let mut scanner = Scanner::new("".to_string());
        let result = scanner.scan_tokens();

        let expected: Vec<Token> = vec![Token::new("".to_string(), TokenKind::EOF, 1)];

        assert_eq!(result, expected);
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn ids_bang() {
        let mut scanner = Scanner::new("!".to_string());
        let result = scanner.scan_tokens();

        let expected: Vec<Token> = vec![
            Token::new("!".to_string(), TokenKind::Bang, 1),
            Token::new("".to_string(), TokenKind::EOF, 1),
        ];

        assert_eq!(result, expected);
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn ids_bang_equal() {
        let mut scanner = Scanner::new("!=".to_string());
        let result = scanner.scan_tokens();

        let expected: Vec<Token> = vec![
            Token::new("!=".to_string(), TokenKind::BangEqual, 1),
            Token::new("".to_string(), TokenKind::EOF, 1),
        ];

        assert_eq!(result, expected);
        assert_eq!(result.len(), 2);
    }
}
