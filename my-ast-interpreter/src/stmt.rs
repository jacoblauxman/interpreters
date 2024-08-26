use crate::{Expr, Token};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub enum Stmt {
    Expression(Expr),
    Print(Expr),
    Var(Token, Expr),
    Block(Vec<Stmt>),
}

impl Display for Stmt {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Stmt::Var(tok, expr) => write!(f, "{} = {}", tok.lexeme, expr),
            Stmt::Print(expr) | Stmt::Expression(expr) => write!(f, "{}", expr),
            Stmt::Block(statements) => {
                let stmts = statements
                    .iter()
                    .map(|stmt| format!("{}", stmt))
                    .collect::<Vec<String>>()
                    .join("\n");

                write!(f, "{}", stmts)
            }
        }
    }
}
