use crate::Token;
use std::{
    fmt::{Display, Formatter},
    hash::{Hash, Hasher},
};

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
    Logical {
        operator: Token,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Call {
        callee: Box<Expr>,
        paren: Token, // for location/RTE info re: fn's closing paren
        arguments: Vec<Box<Expr>>,
    },
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
            Expr::Logical {
                operator,
                left,
                right,
            } => write!(f, "{} {} {}", left, operator, right),
            Expr::Call {
                callee, arguments, ..
            } => {
                write!(f, "{}(", callee)?;
                for (i, arg) in arguments.iter().enumerate() {
                    if i < arguments.len() - 1 {
                        write!(f, "{}, ", arg)?;
                    } else {
                        write!(f, "{}", arg)?;
                    }
                }
                write!(f, ")")
            }
        }
    }
}

impl PartialEq for Expr {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Expr::Number(a), Expr::Number(b)) => a.to_bits() == b.to_bits(),
            (Expr::String(a), Expr::String(b)) => a == b,
            (Expr::Bool(a), Expr::Bool(b)) => a == b,
            (Expr::Grouping(a), Expr::Grouping(b)) => a == b,
            (
                Expr::Unary {
                    operator: o_a,
                    right: r_a,
                },
                Expr::Unary {
                    operator: o_b,
                    right: r_b,
                },
            ) => o_a == o_b && r_a == r_b,
            (
                Expr::Binary {
                    operator: o_a,
                    left: l_a,
                    right: r_a,
                },
                Expr::Binary {
                    operator: o_b,
                    left: l_b,
                    right: r_b,
                },
            ) => o_a == o_b && l_a == l_b && r_a == r_b,
            (Expr::Variable(a), Expr::Variable(b)) => a == b,
            (Expr::Assign(t_a, expr_a), Expr::Assign(t_b, expr_b)) => {
                t_a == t_b && expr_a == expr_b
            }
            (
                Expr::Logical {
                    operator: o_a,
                    left: l_a,
                    right: r_a,
                },
                Expr::Logical {
                    operator: o_b,
                    left: l_b,
                    right: r_b,
                },
            ) => o_a == o_b && l_a == l_b && r_a == r_b,
            (
                Expr::Call {
                    callee: c_a,
                    paren: p_a,
                    arguments: a_a,
                },
                Expr::Call {
                    callee: c_b,
                    paren: p_b,
                    arguments: a_b,
                },
            ) => c_a == c_b && p_a == p_b && a_a == a_b,
            _ => false,
        }
    }
}

impl Eq for Expr {}

impl Hash for Expr {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Expr::Number(f) => f.to_bits().hash(state), // convert f64 to u64
            Expr::String(s) => s.hash(state),
            Expr::Bool(b) => b.hash(state),
            Expr::Nil => "nil".hash(state),
            Expr::Grouping(expr) => expr.hash(state),
            Expr::Unary { operator, right } => {
                operator.hash(state);
                right.hash(state);
            }
            Expr::Binary {
                operator,
                left,
                right,
            } => {
                operator.hash(state);
                left.hash(state);
                right.hash(state);
            }
            Expr::Variable(t) => t.hash(state),
            Expr::Assign(t, expr) => {
                t.hash(state);
                expr.hash(state);
            }
            Expr::Logical {
                operator,
                left,
                right,
            } => {
                operator.hash(state);
                left.hash(state);
                right.hash(state);
            }
            Expr::Call {
                callee,
                paren,
                arguments,
            } => {
                callee.hash(state);
                paren.hash(state);
                arguments.hash(state);
            }
        }
    }
}
