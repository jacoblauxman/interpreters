use crate::Token;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub enum Expr {
    Number(f64),
    String(String),
    Bool(bool),
    Nil,
    Grouping(Box<Expr>),
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
    Binary {
        operator: Token,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Variable(Token),
    Assign(Token, Box<Expr>),
}

impl Display for Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Number(num) => write!(f, "{num:?}"),
            Expr::String(string) => write!(f, "{string}"),
            Expr::Bool(boolean) => write!(f, "{boolean}"),
            Expr::Nil => write!(f, "nil"),
            Expr::Unary { operator, right } => write!(f, "({} {right})", operator.lexeme),
            Expr::Binary {
                operator,
                right,
                left,
            } => write!(f, "({} {left} {right})", operator.lexeme),
            Expr::Grouping(expr) => write!(f, "(group {expr})"),
            Expr::Variable(var) => write!(f, "{}", var.lexeme),
            Expr::Assign(tok, expr) => write!(f, "{} = {}", tok.lexeme, expr),
        }
    }
}
