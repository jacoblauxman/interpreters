use crate::{Environment, ExprValue, Interpreter, RuntimeError, Stmt, Token};
use std::{cell::RefCell, rc::Rc};

#[derive(Debug, Clone, PartialEq)]
pub enum Callable {
    Function {
        name: Token,
        params: Vec<Token>,
        body: Box<Stmt>,
        closure: Rc<RefCell<Environment>>,
    },
    // NativeFn {
    //     name: String,
    //     params: &[ExprValue],
    //     closure: Rc<RefCell<Environment>>,
    //     func: fn(&[ExprValue]) -> Result<ExprValue, RuntimeError>,
    // },
    // Method {
    //     name: Token,
    //     params: Vec<Token>,
    //     body: Vec<Stmt>,
    // },
}

impl Callable {
    pub fn arity(&self) -> usize {
        match self {
            Callable::Function { params, .. } => params.len(),
            // Callable::NativeFn {params, ..} => params.len(),
        }
    }

    pub fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<ExprValue>,
    ) -> Result<ExprValue, RuntimeError> {
        match self {
            Callable::Function {
                params,
                body,
                closure,
                ..
            } => {
                // let mut call_environment =
                // Environment::with_enclosing(interpreter.environment.clone());
                let mut call_environment = Environment::with_enclosing(closure.clone());

                for (param, arg) in params.iter().zip(arguments.iter()) {
                    call_environment.define(param.lexeme.clone(), arg.clone());
                }
                let prev_environment = interpreter.environment.clone();
                interpreter.environment = Rc::new(RefCell::new(call_environment));

                // match interpreter.eval_block_stmt(&[*body.clone()], Some(call_environment)) {
                let eval_result = match interpreter.eval_block_stmt(&[*body.clone()], None) {
                    Ok(_) => Ok(ExprValue::Nil),
                    Err(RuntimeError::Return(return_val)) => Ok(return_val),
                    Err(e) => Err(e),
                };

                interpreter.environment = prev_environment;
                eval_result
            }
        }
    }
}

impl std::fmt::Display for Callable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Callable::Function { name, .. } => write!(f, "<fn {}>", name.lexeme),
            // Callable::NativeFn {name, .. } => write!(f, "<native fn {}>", name.lexeme)
        }
    }
}
