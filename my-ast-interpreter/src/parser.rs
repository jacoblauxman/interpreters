use crate::{Expr, Stmt, Token, TokenLiteral, TokenType};

#[derive(Debug, thiserror::Error)]
#[error("{0}")]
pub struct ParseError(String);

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

pub type ParseResult = Result<Expr, ParseError>;
pub type ParseStmtResult = Result<Stmt, ParseError>;

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, ParseError> {
        let mut statements: Vec<Stmt> = Vec::new();

        while !self.is_at_end() {
            statements.push(self.declaration()?);
        }

        Ok(statements)
    }

    fn declaration(&mut self) -> ParseStmtResult {
        if self.match_types(&[TokenType::VAR]) {
            self.var_declaration()
        } else {
            self.statement()
        }
        .map_err(|err| {
            self.synchronize();
            err
        })
    }

    fn var_declaration(&mut self) -> ParseStmtResult {
        let name = self.consume(&TokenType::IDENTIFIER, "Expect variable name.")?;

        let initializer = if self.match_types(&[TokenType::ASSIGN]) {
            self.expression()?
        } else {
            Expr::Nil
        };

        let _ = self.consume(
            &TokenType::SEMICOLON,
            "Expect ';' after variable declaration.",
        )?;

        Ok(Stmt::Var(name, initializer))
    }

    fn statement(&mut self) -> ParseStmtResult {
        if self.match_types(&[TokenType::PRINT]) {
            self.print_statement()
        } else if self.match_types(&[TokenType::LEFTBRACE]) {
            self.block()
        } else {
            self.expression_statement()
        }
    }

    fn print_statement(&mut self) -> ParseStmtResult {
        let val = self.expression()?;
        // let _ = self.consume(&TokenType::SEMICOLON, "Expect ';' after value.")?;
        // - want to be able to parse expr even without ';' (re: stmt) if valid syntax
        // - evaluation stage should provide RTE instead)
        self.match_types(&[TokenType::SEMICOLON]);

        Ok(Stmt::Print(val))
    }

    fn expression_statement(&mut self) -> ParseStmtResult {
        let expr = self.expression()?;
        self.match_types(&[TokenType::SEMICOLON]);

        Ok(Stmt::Expression(expr))
    }

    fn block(&mut self) -> Result<Stmt, ParseError> {
        let mut statements = Vec::new();

        while !self.check(&TokenType::RIGHTBRACE) && !self.is_at_end() {
            statements.push(self.declaration()?);
        }

        let _ = self.consume(&TokenType::RIGHTBRACE, "Expect '}' after block.")?;

        Ok(Stmt::Block(statements))
    }

    fn expression(&mut self) -> ParseResult {
        self.assignment()
    }

    fn assignment(&mut self) -> ParseResult {
        let expr = self.equality()?;

        if self.match_types(&[TokenType::ASSIGN]) {
            let equals = self.previous().clone();
            let value = self.assignment()?;

            if let Expr::Variable(name) = expr {
                return Ok(Expr::Assign(name, Box::new(value)));
            }

            return Err(ParseError(format!(
                "[line {}] Invalid assignment target {}",
                equals.line, value
            )));
        }

        Ok(expr)
    }

    fn equality(&mut self) -> ParseResult {
        let mut expr = self.comparison()?;

        while self.match_types(&[TokenType::NOTEQUAL, TokenType::EQUAL]) {
            let operator = self.previous().clone();
            let right = self.comparison()?;

            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> ParseResult {
        let mut expr = self.term()?;

        while self.match_types(&[
            TokenType::GREATER,
            TokenType::GREATEREQUAL,
            TokenType::LESS,
            TokenType::LESSEQUAL,
        ]) {
            let operator = self.previous().clone();
            let right = self.term()?;

            expr = Expr::Binary {
                operator,
                right: Box::new(right),
                left: Box::new(expr),
            };
        }

        Ok(expr)
    }

    fn term(&mut self) -> ParseResult {
        let mut expr = self.factor()?;

        while self.match_types(&[TokenType::MINUS, TokenType::PLUS]) {
            let operator = self.previous().clone();
            let right = self.factor()?;

            expr = Expr::Binary {
                operator,
                right: Box::new(right),
                left: Box::new(expr),
            };
        }

        Ok(expr)
    }

    fn factor(&mut self) -> ParseResult {
        let mut expr = self.unary()?;

        while self.match_types(&[TokenType::SLASH, TokenType::STAR]) {
            let operator = self.previous().clone();
            let right = self.unary()?;

            expr = Expr::Binary {
                operator,
                right: Box::new(right),
                left: Box::new(expr),
            };
        }

        Ok(expr)
    }

    fn unary(&mut self) -> ParseResult {
        if self.match_types(&[TokenType::BANG, TokenType::MINUS]) {
            let (operator, right) = (self.previous().clone(), self.unary()?);

            return Ok(Expr::Unary {
                operator,
                right: Box::new(right),
            });
        }

        self.primary()
    }

    fn primary(&mut self) -> ParseResult {
        if self.match_types(&[TokenType::TRUE]) {
            return Ok(Expr::Bool(true));
        } else if self.match_types(&[TokenType::FALSE]) {
            return Ok(Expr::Bool(false));
        } else if self.match_types(&[TokenType::NIL]) {
            return Ok(Expr::Nil);
        }

        if self.match_types(&[TokenType::NUMBER]) {
            if let Some(TokenLiteral::Number(num)) = &self.previous().literal {
                return Ok(Expr::Number(*num));
            }
        }

        if self.match_types(&[TokenType::STRING]) {
            if let Some(TokenLiteral::String(s)) = &self.previous().literal {
                return Ok(Expr::String(s.clone()));
            }
        }

        if self.match_types(&[TokenType::IDENTIFIER]) {
            return Ok(Expr::Variable(self.previous().clone()));
        }

        if self.match_types(&[TokenType::LEFTPAREN]) {
            let expr = self.expression()?;
            self.consume(&TokenType::RIGHTPAREN, "Expect ')' after expression.")?;
            return Ok(Expr::Grouping(Box::new(expr)));
        }

        let peeked = self.peek();

        Err(ParseError(format!(
            // "Expect expression. Got {:?}",
            "[line {}] Parse Error: Expected valid primary expression. Received '{}'.",
            peeked.line, peeked.lexeme
        )))
    }

    fn match_types(&mut self, token_types: &[TokenType]) -> bool {
        for token_type in token_types.iter() {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }

        false
    }

    fn consume(&mut self, token_type: &TokenType, message: &str) -> Result<Token, ParseError> {
        if self.check(token_type) {
            return Ok(self.advance().clone());
        } else {
            Err(ParseError(message.to_string()))
        }
    }

    fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }

        &self.peek().token_type == token_type
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }

        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::EOF
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type == TokenType::SEMICOLON {
                return;
            }

            match self.peek().token_type {
                TokenType::CLASS
                | TokenType::FUN
                | TokenType::VAR
                | TokenType::FOR
                | TokenType::IF
                | TokenType::WHILE
                | TokenType::PRINT
                | TokenType::RETURN => return,
                _ => (),
            }

            self.advance();
        }
    }
}
