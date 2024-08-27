pub mod environment;
pub mod expr;
pub mod interpreter;
pub mod parser;
pub mod scanner;
pub mod stmt;
pub mod token;

pub use environment::Environment;
pub use expr::*;
pub use interpreter::{ExprValue, Interpreter, RuntimeError};
pub use parser::Parser;
pub use scanner::Scanner;
pub use stmt::Stmt;
pub use token::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Callable {
    Function {
        name: Token,
        params: Vec<Token>,
        // body: Vec<Stmt>,
        body: Box<Stmt>,
    },
    // Method {
    //     name: Token,
    //     params: Vec<Token>,
    //     body: Vec<Stmt>,
    // },
}

impl Callable {
    pub fn arity(&self) -> usize {
        match self {
            Callable::Function {
                name: _,
                params,
                body: _,
            } => params.len(),
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Callable::Function {
                name,
                params: _,
                body: _,
            } => &name.lexeme,
        }
    }

    pub fn call(&self, interpreter: &mut Interpreter, args: Vec<ExprValue>) -> ExprValue {
        todo!()
    }
}
