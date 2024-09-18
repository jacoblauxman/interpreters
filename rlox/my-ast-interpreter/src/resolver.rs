use crate::{Expr, Interpreter, Stmt, Token};
use std::collections::HashMap;

#[derive(Debug, thiserror::Error)]
#[error("[line {line}] Binding Error: {message} : {token}")]
pub struct BindingError {
    token: String,
    message: &'static str,
    line: usize,
}

pub enum FunctionType {
    None,
    Function,
}

pub struct Resolver {
    interpreter: &'static mut Interpreter,
    scopes: Vec<HashMap<String, bool>>,
    function_type: FunctionType,
}

impl Resolver {
    fn new(interpreter: &'static mut Interpreter) -> Self {
        Self {
            interpreter,
            scopes: vec![],
            function_type: FunctionType::None,
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
                message: &"Already a variable with this name in the current scope",
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

    fn resolve(&mut self, statements: &[Stmt]) -> Result<(), BindingError> {
        for stmt in statements {
            self.resolve_stmt(stmt)?
        }

        Ok(())
    }

    fn resolve_stmt(&mut self, stmt: &Stmt) -> Result<(), BindingError> {
        match stmt {
            Stmt::Expression(expr) => self.resolve_expr(expr),
            Stmt::Print(expr) => self.resolve_expr(expr),
            Stmt::Var(token, initializer) => {
                self.declare(token)?;
                if initializer == &Expr::Nil {
                    self.resolve_expr(initializer)?;
                }

                self.define(token);
                Ok(())
            }
            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => {
                todo!()
            }
            _ => todo!(),
        }
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
