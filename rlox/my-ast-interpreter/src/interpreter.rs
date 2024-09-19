use crate::Callable;
use crate::{Environment, Expr, Stmt, Token, TokenType};
use std::{
    cell::RefCell,
    collections::HashMap,
    fmt::{Display, Formatter},
    rc::Rc,
};

#[derive(Debug, thiserror::Error)]
pub enum RuntimeError {
    #[error("[line {line}] Runtime Error: {message}")]
    RTE {
        token: String,
        message: String,
        line: usize,
    },
    #[error("return value of {0:?}")]
    Return(ExprValue),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExprValue {
    Bool(bool),
    Number(f64),
    String(String),
    Nil,
    Call(Callable),
}

impl Display for ExprValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ExprValue::Bool(b) => write!(f, "{b}"),
            ExprValue::Number(n) => {
                write!(f, "{n}")
            }
            ExprValue::String(s) => write!(f, "{s}"),
            ExprValue::Nil => write!(f, "nil"),
            ExprValue::Call(callable) => write!(f, "{}", callable),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
enum InterpreterStatus {
    #[default]
    Evaluate,
    Run,
}

impl TryFrom<&str> for InterpreterStatus {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "evaluate" => Ok(InterpreterStatus::Evaluate),
            "run" => Ok(InterpreterStatus::Run),
            _ => Err("should only accept `evaluate` and `run` string values".to_string()),
        }
    }
}

#[derive(Default, Debug, Clone)]
#[allow(dead_code)]
pub struct Interpreter {
    pub environment: Rc<RefCell<Environment>>,
    status: InterpreterStatus,
    // globals: Environment,
    globals: Rc<RefCell<Environment>>,
    pub locals: HashMap<Expr, usize>,
}

impl Interpreter {
    pub fn new() -> Self {
        // TODO: need to organize globals for the `clock` function implemented in 'jlox'
        let globals = Rc::new(RefCell::new(Environment::new()));
        Interpreter {
            environment: globals.clone(),
            status: InterpreterStatus::Evaluate,
            globals,
            locals: HashMap::new(),
        }
    }

    pub fn interpret(&mut self, statements: Vec<Stmt>) -> Result<(), RuntimeError> {
        for statement in statements.iter() {
            self.execute(statement)?;
        }

        Ok(())
    }

    pub fn resolve(&mut self, expr: &Expr, depth: usize) {
        self.locals.insert(expr.clone(), depth);
    }

    fn look_up_var(&mut self, token: &Token, expr: &Expr) -> ExprValue {
        if let Some(distance) = self.locals.get(expr) {
            self.environment.borrow().get_at(*distance, token)
        } else {
            // self.globals
            //     .get(&expr)
            //     .expect("globals should have var from variable expression")
            self.globals
                .borrow()
                .get(token)
                .expect("globals should have defined var from variable expression")
        }
    }

    pub fn set_status(&mut self, status: &str) -> Result<(), String> {
        let status = InterpreterStatus::try_from(status)?;
        self.status = status;

        Ok(())
    }

    fn execute(&mut self, stmt: &Stmt) -> Result<(), RuntimeError> {
        match stmt {
            Stmt::Expression(_) => self.eval_expr_stmt(stmt),
            Stmt::Print(_) => self.eval_print_stmt(stmt),
            Stmt::Var(name, initializer) => self.eval_var_stmt(name, initializer),
            Stmt::Block(statements) => self.eval_block_stmt(statements, None),
            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => self.eval_if_stmt(condition, then_branch, else_branch),
            Stmt::While { condition, body } => self.eval_while_stmt(condition, body),
            Stmt::Function(callable) => self.eval_function_stmt(callable),
            Stmt::Return(_, val) => self.eval_return_stmt(val),
        }
    }

    fn eval_function_stmt(&mut self, callable: &Callable) -> Result<(), RuntimeError> {
        match callable {
            Callable::Function {
                name, params, body, ..
            } => {
                let function = Callable::Function {
                    name: name.clone(),
                    params: params.clone(),
                    body: body.clone(),
                    closure: self.environment.clone(), // update to current interpreter env (re: parser sets to empty env)
                };
                let function_value = ExprValue::Call(function);
                self.environment
                    .borrow_mut()
                    .define(name.lexeme.clone(), function_value);
            }
        }

        Ok(())
    }

    pub fn eval_block_stmt(
        &mut self,
        statements: &[Stmt],
        environment: Option<Environment>,
    ) -> Result<(), RuntimeError> {
        let prev_env = self.environment.clone();

        let block_env = match environment {
            Some(env) => env,
            None => Environment::with_enclosing(prev_env.clone()),
        };

        self.environment = Rc::new(RefCell::new(block_env));

        let block_eval: Result<(), RuntimeError> = (|| {
            for stmt in statements.iter() {
                self.execute(stmt)?;
            }

            Ok(())
        })();

        self.environment = prev_env;
        block_eval
    }

    fn eval_expr_stmt(&mut self, stmt: &Stmt) -> Result<(), RuntimeError> {
        match stmt {
            Stmt::Expression(expr) => {
                let stmt = self.evaluate(expr)?;
                if self.status == InterpreterStatus::Evaluate {
                    println!("{}", stmt);
                }

                Ok(())
            }
            _ => unreachable!("use with expression statements only!"),
        }
    }

    fn eval_print_stmt(&mut self, stmt: &Stmt) -> Result<(), RuntimeError> {
        match stmt {
            Stmt::Print(expr) => {
                let stmt = self.evaluate(expr)?;
                println!("{}", stmt);

                Ok(())
            }
            _ => unreachable!("use with print statements only!"),
        }
    }

    fn eval_return_stmt(&mut self, val: &Option<Expr>) -> Result<(), RuntimeError> {
        match val {
            Some(stmt_val) => {
                let return_val = self.evaluate(stmt_val)?;

                Err(RuntimeError::Return(return_val))
            }
            None => Ok(()),
        }
    }

    fn eval_var_stmt(&mut self, name: &Token, initializer: &Expr) -> Result<(), RuntimeError> {
        let expr = self.evaluate(initializer)?;
        self.environment
            .borrow_mut()
            .define(name.lexeme.clone(), expr);

        Ok(())
    }

    fn eval_while_stmt(&mut self, condition: &Expr, body: &Stmt) -> Result<(), RuntimeError> {
        // let expr_val = self.evaluate(condition)?; // nope! this makes conditional only evaluated once (!!)
        while self.is_truthy(&self.clone().evaluate(condition)?) {
            self.execute(body)?;
        }

        Ok(())
    }

    fn eval_if_stmt(
        &mut self,
        condition: &Expr,
        then_branch: &Stmt,
        else_branch: &Option<Box<Stmt>>,
    ) -> Result<(), RuntimeError> {
        let expr_val = self.evaluate(condition)?;

        if self.is_truthy(&expr_val) {
            self.execute(then_branch)?;
        } else if else_branch.is_some() {
            self.execute(else_branch.as_ref().expect("else_branch is some"))?;
        }

        Ok(())
    }

    pub fn evaluate(&mut self, expr: &Expr) -> Result<ExprValue, RuntimeError> {
        match expr {
            Expr::Bool(b) => Ok(ExprValue::Bool(*b)),
            Expr::Number(n) => Ok(ExprValue::Number(*n)),
            Expr::String(s) => Ok(ExprValue::String(s.to_owned())),
            Expr::Nil => Ok(ExprValue::Nil),
            Expr::Grouping(expr) => self.evaluate(expr),
            Expr::Unary { operator, right } => self.evaluate_unary(operator, right),
            Expr::Binary {
                operator,
                right,
                left,
            } => self.evaluate_binary(operator, left, right),
            // Expr::Variable(name) => self.environment.borrow().get(name),
            Expr::Variable(name) => Ok(self.look_up_var(name, expr)),
            Expr::Assign(name, val) => {
                let val = self.evaluate(val)?;
                if let Some(distance) = self.locals.get(expr) {
                    self.environment
                        .borrow_mut()
                        .assign_at(*distance, name, val.clone());
                } else {
                    self.globals.borrow_mut().assign(name, val.clone())?;
                }
                // self.environment.borrow_mut().assign(name, val.clone())?;
                Ok(val)
            }
            Expr::Logical {
                operator,
                left,
                right,
            } => self.evaluate_logical(operator, left, right),
            Expr::Call {
                callee,
                paren,
                arguments,
            } => self.evaluate_call(callee, paren, arguments),
        }
    }

    fn evaluate_unary(
        &mut self,
        operator: &Token,
        right: &Expr,
    ) -> Result<ExprValue, RuntimeError> {
        let right = self.evaluate(right)?;

        match operator.token_type {
            TokenType::BANG => Ok(ExprValue::Bool(!self.is_truthy(&right))),
            TokenType::MINUS => {
                let expr_num = self.check_num_operand(operator, &right)?;
                Ok(ExprValue::Number(-expr_num))
            }
            _ => Err(RuntimeError::RTE {
                token: operator.to_string(),
                message: "Invalid operator found in unary expression".to_string(),
                line: operator.line,
            }),
        }
    }

    fn evaluate_binary(
        &mut self,
        operator: &Token,
        left: &Expr,
        right: &Expr,
    ) -> Result<ExprValue, RuntimeError> {
        let left = self.evaluate(left)?;
        let right = self.evaluate(right)?;

        match operator.token_type {
            TokenType::GREATER => {
                let (left, right) = self.check_num_operands(operator, &left, &right)?;
                Ok(ExprValue::Bool(left > right))
            }
            TokenType::GREATEREQUAL => {
                let (left, right) = self.check_num_operands(operator, &left, &right)?;
                Ok(ExprValue::Bool(left >= right))
            }
            TokenType::LESS => {
                let (left, right) = self.check_num_operands(operator, &left, &right)?;
                Ok(ExprValue::Bool(left < right))
            }
            TokenType::LESSEQUAL => {
                let (left, right) = self.check_num_operands(operator, &left, &right)?;
                Ok(ExprValue::Bool(left <= right))
            }
            TokenType::MINUS => {
                let (left, right) = self.check_num_operands(operator, &left, &right)?;
                Ok(ExprValue::Number(left - right))
            }
            TokenType::PLUS => match (left, right) {
                (ExprValue::Number(left), ExprValue::Number(right)) => {
                    Ok(ExprValue::Number(left + right))
                }
                (ExprValue::String(left), ExprValue::String(right)) => {
                    let expr_val = left + &right;
                    Ok(ExprValue::String(expr_val))
                }
                _ => Err(RuntimeError::RTE {
                    token: operator.lexeme.to_string(),
                    message: "Operands must be two numbers or two strings.".to_string(),
                    line: operator.line,
                }),
            },
            TokenType::SLASH => {
                let (left, right) = self.check_num_operands(operator, &left, &right)?;
                Ok(ExprValue::Number(left / right))
            }
            TokenType::STAR => {
                let (left, right) = self.check_num_operands(operator, &left, &right)?;
                Ok(ExprValue::Number(left * right))
            }
            TokenType::NOTEQUAL => Ok(ExprValue::Bool(!self.is_equal(&left, &right))),
            TokenType::EQUAL => Ok(ExprValue::Bool(self.is_equal(&left, &right))),
            _ => Err(RuntimeError::RTE {
                token: operator.lexeme.to_string(),
                message: "Unrecognized binary operator.".to_string(),
                line: operator.line,
            }),
        }
    }

    fn evaluate_call(
        &mut self,
        callee: &Expr,
        paren: &Token,
        arguments: &[Box<Expr>],
    ) -> Result<ExprValue, RuntimeError> {
        let callee = self.evaluate(callee)?;

        let mut args = Vec::new();

        for arg in arguments {
            args.push(self.evaluate(arg)?);
        }

        if let ExprValue::Call(function) = callee {
            let (args_len, fn_arity) = (args.len(), function.arity());
            if args_len != fn_arity {
                return Err(RuntimeError::RTE {
                    token: paren.lexeme.clone(),
                    message: format!("Expected {} arguments but got {}.", fn_arity, args_len),
                    line: paren.line,
                });
            }

            // let mut interpreter = self.clone(); // TODO: FIX THIS MESS?
            // Ok(function.call(&mut interpreter, args)?)
            Ok(function.call(self, args)?)
        } else {
            Err(RuntimeError::RTE {
                token: paren.lexeme.clone(),
                message: "Can only call functions and methods".to_string(),
                line: paren.line,
            })
        }
    }

    fn evaluate_logical(
        &mut self,
        operator: &Token,
        left: &Expr,
        right: &Expr,
    ) -> Result<ExprValue, RuntimeError> {
        match operator.token_type {
            TokenType::OR => Ok(self.evaluate(left)?),
            _ => Ok(self.evaluate(right)?),
        }
    }

    // helpers
    fn check_num_operand(
        &self,
        operator: &Token,
        expr_val: &ExprValue,
    ) -> Result<f64, RuntimeError> {
        match expr_val {
            ExprValue::Number(n) => Ok(*n),
            _ => Err(RuntimeError::RTE {
                token: operator.lexeme.to_string(),
                message: "Operand must be a number.".to_string(),
                line: operator.line,
            }),
        }
    }

    fn check_num_operands(
        &self,
        operator: &Token,
        left: &ExprValue,
        right: &ExprValue,
    ) -> Result<(f64, f64), RuntimeError> {
        match (left, right) {
            (ExprValue::Number(left), ExprValue::Number(right)) => Ok((*left, *right)),
            _ => Err(RuntimeError::RTE {
                token: operator.lexeme.to_string(),
                message: "Operands must be numbers".to_string(),
                line: operator.line,
            }),
        }
    }

    fn is_truthy(&self, expr_val: &ExprValue) -> bool {
        match expr_val {
            ExprValue::Nil => false,
            ExprValue::Bool(b) => *b,
            _ => true,
        }
    }

    fn is_equal(&self, left: &ExprValue, right: &ExprValue) -> bool {
        match (left, right) {
            (ExprValue::Nil, ExprValue::Nil) => true,
            (ExprValue::Bool(a), ExprValue::Bool(b)) => a == b,
            (ExprValue::Number(a), ExprValue::Number(b)) => (a - b).abs() < f64::EPSILON,
            (ExprValue::String(a), ExprValue::String(b)) => a == b,
            _ => false,
        }
    }
}
