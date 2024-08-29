use crate::Callable;
use crate::{Expr, Token};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Expression(Expr),
    Print(Expr),
    Var(Token, Expr),
    Block(Vec<Stmt>),
    If {
        condition: Expr,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>,
    },
    While {
        condition: Expr,
        body: Box<Stmt>,
    },
    Function(Callable),
    Return(Token, Option<Expr>),
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
            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => {
                writeln!(f, "if ({}) {{", condition)?;
                writeln!(f, "{}", then_branch)?;

                if let Some(else_branch) = else_branch {
                    writeln!(f, "}} else {{")?;
                    write!(f, "{}", else_branch)?;
                }

                writeln!(f, "}}")
            }
            Stmt::While { condition, body } => {
                writeln!(f, "while ({}) {{", condition)?;
                writeln!(f, "{}", body)?;
                writeln!(f, "}}")
            }
            Stmt::Function(callable) => match callable {
                Callable::Function {
                    name, params, body, ..
                } => {
                    write!(f, "fun {}(", name.lexeme)?;
                    for (i, param) in params.iter().enumerate() {
                        if i < params.len() - 1 {
                            write!(f, "{}, ", param)?;
                        } else {
                            write!(f, "{}", param)?;
                        }
                    }
                    writeln!(f, ") {{")?;
                    write!(f, "{}", body)?;
                    write!(f, "}}")
                }
            },
            Stmt::Return(_, val) => {
                if let Some(expr) = val {
                    write!(f, "{}", expr)
                } else {
                    write!(f, "nil")
                }
            }
        }
    }
}
