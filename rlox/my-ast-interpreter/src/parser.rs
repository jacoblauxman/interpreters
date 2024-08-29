use crate::{Callable, Environment};
use crate::{Expr, Stmt, Token, TokenLiteral, TokenType};
use std::{cell::RefCell, rc::Rc};

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
        } else if self.match_types(&[TokenType::FUN]) {
            self.function("function")
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

        self.consume(
            &TokenType::SEMICOLON,
            "Expect ';' after variable declaration.",
        )?;

        Ok(Stmt::Var(name, initializer))
    }

    fn statement(&mut self) -> ParseStmtResult {
        if self.match_types(&[TokenType::PRINT]) {
            self.print_statement()
        } else if self.match_types(&[TokenType::RETURN]) {
            self.return_statement()
        } else if self.match_types(&[TokenType::WHILE]) {
            self.while_statement()
        } else if self.match_types(&[TokenType::FOR]) {
            self.for_statement()
        } else if self.match_types(&[TokenType::IF]) {
            self.if_statement()
        } else if self.match_types(&[TokenType::LEFTBRACE]) {
            self.block()
        } else {
            self.expression_statement()
        }
    }

    fn for_statement(&mut self) -> ParseStmtResult {
        self.consume(&TokenType::LEFTPAREN, "Expect '(' after 'for'.")?;

        let mut initializer = None;

        if !self.match_types(&[TokenType::SEMICOLON]) {
            if self.match_types(&[TokenType::VAR]) {
                initializer = Some(self.var_declaration()?);
            } else {
                initializer = Some(self.expression_statement()?);
            }
        }

        let mut condition = None;

        if !self.check(&TokenType::SEMICOLON) {
            condition = Some(self.expression()?);
        }
        self.consume(
            &TokenType::SEMICOLON,
            "Expect ';' after 'for' loop condition.",
        )?;

        let mut increment = None;

        if !self.check(&TokenType::RIGHTPAREN) {
            increment = Some(self.expression()?);
        }

        self.consume(&TokenType::RIGHTPAREN, "Expect ')' after 'for' clauses.")?;

        let mut body = self.statement()?;

        body = match body {
            Stmt::Block(stmts) => {
                let mut statements = stmts.clone();

                if let Some(increment) = increment {
                    statements.push(Stmt::Expression(increment));
                }

                if condition.is_none() {
                    condition.replace(Expr::Bool(true));
                }

                let while_body = Stmt::While {
                    condition: condition.expect("condition should be some"),
                    body: Box::new(Stmt::Block(statements.clone())),
                };

                if initializer.is_some() {
                    Stmt::Block(vec![
                        initializer.expect("initializer should be some"),
                        while_body,
                    ])
                } else {
                    while_body
                }
            }
            _ => unreachable!(),
        };

        Ok(body)
    }

    fn while_statement(&mut self) -> ParseStmtResult {
        self.consume(&TokenType::LEFTPAREN, "Expect '(' after 'while'.")?;
        let condition = self.expression()?;
        self.consume(
            &TokenType::RIGHTPAREN,
            "Expect ')' after 'while' condition.",
        )?;

        let body = Box::new(self.statement()?);

        Ok(Stmt::While { condition, body })
    }

    fn if_statement(&mut self) -> ParseStmtResult {
        self.consume(&TokenType::LEFTPAREN, "Expect '(' after 'if'.")?;
        let condition = self.expression()?;
        self.consume(&TokenType::RIGHTPAREN, "Expect ')' after 'if' condition.")?;

        let then_branch = Box::new(self.statement()?);
        let mut else_branch = None;

        if self.match_types(&[TokenType::ELSE]) {
            else_branch = Some(Box::new(self.statement()?));
        }

        Ok(Stmt::If {
            condition,
            then_branch,
            else_branch,
        })
    }

    fn print_statement(&mut self) -> ParseStmtResult {
        let val = self.expression()?;
        // self.consume(&TokenType::SEMICOLON, "Expect ';' after value.")?;
        // - want to be able to parse expr even without ';' (re: stmt) if valid syntax
        // - evaluation stage should provide RTE instead)
        self.match_types(&[TokenType::SEMICOLON]);

        Ok(Stmt::Print(val))
    }

    fn return_statement(&mut self) -> ParseStmtResult {
        let keyword = self.previous().clone();

        let val = if !self.check(&TokenType::SEMICOLON) {
            Some(self.expression()?)
        } else {
            None
        };

        self.consume(&TokenType::SEMICOLON, "Expect ';' after return value.")?;

        Ok(Stmt::Return(keyword, val))
    }

    fn expression_statement(&mut self) -> ParseStmtResult {
        let expr = self.expression()?;
        self.match_types(&[TokenType::SEMICOLON]);

        Ok(Stmt::Expression(expr))
    }

    fn function(&mut self, kind: &str) -> ParseStmtResult {
        let name = self.consume(&TokenType::IDENTIFIER, &format!("Expect {kind} name."))?;

        self.consume(
            &TokenType::LEFTPAREN,
            &format!("Expect '(' after {kind} name."),
        )?;

        let mut params = Vec::new();

        if !self.check(&TokenType::RIGHTPAREN) {
            loop {
                if params.len() >= 255 {
                    return Err(ParseError(
                        "Can't have more than 255 parameters.".to_string(),
                    ));
                }

                params.push(self.consume(&TokenType::IDENTIFIER, "Expect parameter name.")?);
                if !self.match_types(&[TokenType::COMMA]) {
                    break;
                }
            }
        }

        self.consume(&TokenType::RIGHTPAREN, "Expect ')' after parameters.")?;

        self.consume(
            &TokenType::LEFTBRACE,
            &format!("Expect '{{' before {kind} body."),
        )?;

        let body = Box::new(self.block()?);

        Ok(Stmt::Function(Callable::Function {
            name,
            params,
            body,
            closure: Rc::new(RefCell::new(Environment::new())),
        }))
    }

    fn block(&mut self) -> Result<Stmt, ParseError> {
        let mut statements = Vec::new();

        while !self.check(&TokenType::RIGHTBRACE) && !self.is_at_end() {
            statements.push(self.declaration()?);
        }

        self.consume(&TokenType::RIGHTBRACE, "Expect '}' after block.")?;

        Ok(Stmt::Block(statements))
    }

    fn expression(&mut self) -> ParseResult {
        self.assignment()
    }

    fn assignment(&mut self) -> ParseResult {
        let expr = self.or()?;

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

    fn or(&mut self) -> ParseResult {
        let mut expr = self.and()?;

        while self.match_types(&[TokenType::OR]) {
            let operator = self.previous().clone();
            let right = self.and()?;

            expr = Expr::Logical {
                operator,
                left: Box::new(expr),
                right: Box::new(right),
            }
        }

        Ok(expr)
    }

    fn and(&mut self) -> ParseResult {
        let mut expr = self.equality()?;

        while self.match_types(&[TokenType::AND]) {
            let operator = self.previous().clone();
            let right = self.equality()?;

            expr = Expr::Logical {
                operator,
                left: Box::new(expr),
                right: Box::new(right),
            }
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

        self.call()
    }

    fn call(&mut self) -> ParseResult {
        let mut expr = self.primary()?;

        loop {
            if self.match_types(&[TokenType::LEFTPAREN]) {
                expr = self.finish_call(&expr)?;
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn finish_call(&mut self, callee: &Expr) -> ParseResult {
        let mut arguments = Vec::new();

        if !self.check(&TokenType::RIGHTPAREN) {
            loop {
                if arguments.len() >= 255 {
                    return Err(ParseError("Can't have more than 255 arguments".to_string()));
                }
                arguments.push(Box::new(self.expression()?));

                if !self.match_types(&[TokenType::COMMA]) {
                    break;
                }
            }
        }

        let paren = self.consume(
            &TokenType::RIGHTPAREN,
            "Expect ')' after function arguments.",
        )?;

        Ok(Expr::Call {
            callee: Box::new(callee.clone()),
            paren,
            arguments,
        })
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
