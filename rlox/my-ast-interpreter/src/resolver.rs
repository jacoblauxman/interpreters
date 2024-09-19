use crate::{Callable, Expr, Interpreter, Stmt, Token};
use std::collections::HashMap;

#[derive(Debug, thiserror::Error)]
#[error("[line {line}] Binding Error: {message} : {token}")]
pub struct BindingError {
    token: String,
    message: &'static str,
    line: usize,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum FunctionType {
    None,
    Function,
}

pub struct Resolver<'a> {
    pub interpreter: &'a mut Interpreter,
    scopes: Vec<HashMap<String, bool>>,
    current_function: FunctionType,
}

impl<'a> Resolver<'a> {
    pub fn new(interpreter: &'a mut Interpreter) -> Self {
        Self {
            interpreter,
            scopes: vec![],
            current_function: FunctionType::None,
        }
    }

    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new())
    }

    fn end_scope(&mut self) {
        self.scopes.pop();
    }

    fn declare(&mut self, token: &Token) -> Result<(), BindingError> {
        if self.scopes.is_empty() {
            return Ok(());
        }

        // let mut scope = self.scopes.peek();
        let scope = self
            .scopes
            .last_mut()
            .expect("scopes should have `Some(scope)` last_mut");

        if scope.contains_key(&token.lexeme) {
            return Err(BindingError {
                token: token.lexeme.clone(),
                message: "Already a variable with this name in the current scope",
                line: token.line,
            });
        }

        scope.insert(token.lexeme.clone(), false);

        Ok(())
    }

    fn define(&mut self, token: &Token) {
        if self.scopes.is_empty() {
            return;
        }

        self.scopes
            .last_mut()
            .expect("scopes should have `Some(scope)` last_mut")
            .insert(token.lexeme.clone(), true);
    }

    fn resolve_local(&mut self, expr: &Expr, token: &Token) {
        for (i, scope) in self.scopes.iter().enumerate().rev() {
            if scope.contains_key(&token.lexeme) {
                self.interpreter.resolve(expr, self.scopes.len() - 1 - i);
            }
        }
    }

    pub fn resolve(&mut self, statements: &[Stmt]) -> Result<(), BindingError> {
        // pub fn resolve(&mut self, statements: &[Stmt]) -> Result<Interpreter, BindingError> {
        for stmt in statements {
            self.resolve_stmt(stmt)?
        }

        Ok(())
    }

    fn resolve_stmt(&mut self, stmt: &Stmt) -> Result<(), BindingError> {
        match stmt {
            Stmt::Block(stmts) => {
                self.begin_scope();
                self.resolve(stmts)?;
                self.end_scope();
            }
            Stmt::Expression(expr) => self.resolve_expr(expr)?,
            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => {
                self.resolve_expr(condition)?;
                self.resolve_stmt(then_branch)?;
                if let Some(else_branch) = else_branch {
                    self.resolve_stmt(else_branch)?;
                }
            }
            Stmt::Print(expr) => self.resolve_expr(expr)?,
            Stmt::Return(keyword, val) => {
                if self.current_function == FunctionType::None {
                    return Err(BindingError {
                        token: String::default(),
                        message: "Can't return from top-level code.",
                        line: keyword.line,
                    });
                }

                if let Some(val) = val {
                    self.resolve_expr(val)?;
                }
            }

            Stmt::Var(token, initializer) => {
                self.declare(token)?;
                if initializer == &Expr::Nil {
                    self.resolve_expr(initializer)?;
                }

                self.define(token);
            }
            Stmt::While { condition, body } => {
                self.resolve_expr(condition)?;
                self.resolve_stmt(body)?;
            }
            Stmt::Function(callable) => match callable {
                Callable::Function { name, .. } => {
                    self.declare(name)?;
                    self.define(name);

                    self.resolve_function_stmt(callable, FunctionType::Function)?;
                }
            },
        }

        Ok(())
    }

    fn resolve_function_stmt(
        &mut self,
        function: &Callable,
        fn_type: FunctionType,
    ) -> Result<(), BindingError> {
        let enclosing_fn = self.current_function;
        self.current_function = fn_type;

        self.begin_scope();

        match function {
            Callable::Function { params, body, .. } => {
                for param in params {
                    self.declare(param)?;
                    self.define(param);
                }

                self.resolve_stmt(body)?;
            }
        }

        self.end_scope();
        self.current_function = enclosing_fn;

        Ok(())
    }

    fn resolve_expr(&mut self, expr: &Expr) -> Result<(), BindingError> {
        match expr {
            Expr::Variable(token) => {
                if !self.scopes.is_empty() {
                    if let Some(&false) = self
                        .scopes
                        .last()
                        .expect("scopes should have `Some(scope)` re: last()")
                        .get(&token.lexeme)
                    {
                        return Err(BindingError {
                            token: token.lexeme.clone(),
                            message: "Can't read local variable in its own initializer",
                            line: token.line,
                        });
                    }

                    self.resolve_local(expr, token);
                }
            }
            Expr::Assign(name, val) => {
                self.resolve_expr(val)?;
                self.resolve_local(expr, name);
            }
            Expr::Unary { right, .. } => self.resolve_expr(right)?,
            Expr::Binary { left, right, .. } => {
                self.resolve_expr(left)?;
                self.resolve_expr(right)?;
            }
            Expr::Call {
                callee, arguments, ..
            } => {
                self.resolve_expr(callee)?;
                for arg in arguments {
                    self.resolve_expr(arg)?;
                }
            }
            Expr::Grouping(expr) => self.resolve_expr(expr)?,
            Expr::Logical { left, right, .. } => {
                self.resolve_expr(left)?;
                self.resolve_expr(right)?;
            }
            _ => return Ok(()), // literals don't need binding resolution
        }

        Ok(())
    }
}
