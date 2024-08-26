use crate::{Token, TokenLiteral, TokenType};
use std::collections::HashMap;

pub struct Scanner {
    pub source: Vec<char>,
    pub tokens: Vec<Token>,
    pub errors: Vec<String>,
    keywords: HashMap<&'static str, TokenType>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Scanner {
            source: source.chars().collect(),
            tokens: vec![],
            errors: vec![],
            start: 0,
            current: 0,
            line: 1,
            keywords: HashMap::from([
                ("and", TokenType::AND),
                ("class", TokenType::CLASS),
                ("else", TokenType::ELSE),
                ("false", TokenType::FALSE),
                ("for", TokenType::FOR),
                ("fun", TokenType::FUN),
                ("if", TokenType::IF),
                ("nil", TokenType::NIL),
                ("or", TokenType::OR),
                ("print", TokenType::PRINT),
                ("return", TokenType::RETURN),
                ("super", TokenType::SUPER),
                ("this", TokenType::THIS),
                ("true", TokenType::TRUE),
                ("var", TokenType::VAR),
                ("while", TokenType::WHILE),
            ]),
        }
    }

    pub fn scan_tokens(mut self) -> (Vec<Token>, Vec<String>) {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()
        }

        self.tokens
            .push(Token::new(TokenType::EOF, "".to_string(), None, self.line));

        (self.tokens, self.errors)
    }

    pub fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LEFTPAREN, None),
            ')' => self.add_token(TokenType::RIGHTPAREN, None),
            '{' => self.add_token(TokenType::LEFTBRACE, None),
            '}' => self.add_token(TokenType::RIGHTBRACE, None),
            ',' => self.add_token(TokenType::COMMA, None),
            '.' => self.add_token(TokenType::DOT, None),
            '-' => self.add_token(TokenType::MINUS, None),
            '+' => self.add_token(TokenType::PLUS, None),
            ';' => self.add_token(TokenType::SEMICOLON, None),
            '*' => self.add_token(TokenType::STAR, None),

            '!' => match self.operator_match('=') {
                true => self.add_token(TokenType::NOTEQUAL, None),
                false => self.add_token(TokenType::BANG, None),
            },
            '=' => match self.operator_match('=') {
                true => self.add_token(TokenType::EQUAL, None),
                false => self.add_token(TokenType::ASSIGN, None),
            },
            '<' => match self.operator_match('=') {
                true => self.add_token(TokenType::LESSEQUAL, None),
                false => self.add_token(TokenType::LESS, None),
            },
            '>' => match self.operator_match('=') {
                true => self.add_token(TokenType::GREATEREQUAL, None),
                false => self.add_token(TokenType::GREATER, None),
            },
            '/' => {
                // advance through comment in code
                if self.operator_match('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::SLASH, None)
                }
            }
            ' ' | '\r' | '\t' => (),
            '\n' => self.line += 1,

            '"' => self.string(),

            c => {
                if c.is_ascii_digit() {
                    self.number();
                } else if c.is_alphabetic() || c == '_' {
                    self.identifier();
                } else {
                    // unknown char
                    self.errors.push(format!(
                        "[line {}] Error: Unexpected character: {}",
                        self.line, c
                    ))
                }
            }
        }
    }

    fn identifier(&mut self) {
        while self.peek().is_alphanumeric() || self.peek() == '_' {
            self.advance();
        }

        let literal = self.source[self.start..self.current]
            .iter()
            .collect::<String>();

        let token_type = self
            .keywords
            .get(literal.as_str())
            .unwrap_or(&TokenType::IDENTIFIER)
            .clone();

        self.add_token(token_type, None);
    }

    fn number(&mut self) {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.advance();

            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        let num_str = &self.source[self.start..self.current]
            .iter()
            .collect::<String>();

        if let Ok(num) = num_str.parse::<f64>() {
            self.add_token(TokenType::NUMBER, Some(TokenLiteral::Number(num)));
        } else {
            self.errors.push(format!(
                "[line {}] Error: Invalid number literal: {}",
                self.line, num_str
            ))
        }
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            self.errors
                .push(format!("[line {}] Error: Unterminated string.", self.line));
            return;
        }

        self.advance(); // closing '"'

        let literal = self.source[self.start + 1..self.current - 1]
            .iter()
            .collect::<String>();

        self.add_token(TokenType::STRING, Some(TokenLiteral::String(literal)));
    }

    fn operator_match(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.source[self.current] != expected {
            return false;
        }

        self.current += 1;
        true
    }

    fn peek(&self) -> char {
        if let Some(c) = self.source.get(self.current) {
            *c
        } else {
            '\0'
        }
    }

    fn peek_next(&self) -> char {
        if let Some(c) = self.source.get(self.current + 1) {
            *c
        } else {
            '\0'
        }
    }

    fn advance(&mut self) -> char {
        let c = self.source[self.current];
        self.current += 1;
        c
    }

    fn add_token(&mut self, token_type: TokenType, literal: Option<TokenLiteral>) {
        let text = self.source[self.start..self.current]
            .iter()
            .collect::<String>();
        self.tokens
            .push(Token::new(token_type, text, literal, self.line))
    }

    pub fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}
