use crate::{Environment, ExprValue, Interpreter, RuntimeError, Stmt, Token};
#[derive(Debug, Clone, PartialEq)]
pub enum Callable {
    Function {
        name: Token,
        params: Vec<Token>,
        body: Box<Stmt>,
        // closure: Environment,
    },
    // NativeFn {
    //     name: Token,
    //     params: Vec<Token>,
    //     // closure: Environment,
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
        }
    }

    pub fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<ExprValue>,
    ) -> Result<ExprValue, RuntimeError> {
        match self {
            Callable::Function { params, body, .. } => {
                let mut call_environment =
                    Environment::with_enclosing(interpreter.environment.clone());

                for (param, arg) in params.iter().zip(arguments.iter()) {
                    call_environment.define(param.lexeme.clone(), arg.clone());
                }

                match interpreter.eval_block_stmt(&[*body.clone()], Some(call_environment)) {
                    Ok(_) => Ok(ExprValue::Nil),
                    Err(RuntimeError::Return(return_val)) => Ok(return_val),
                    Err(e) => Err(e),
                }
            }
        }
    }
}

impl std::fmt::Display for Callable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Callable::Function { name, .. } => write!(f, "<fn {}>", name.lexeme),
        }
    }
}
