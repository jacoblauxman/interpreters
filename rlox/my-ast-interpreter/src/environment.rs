use crate::{ExprValue, RuntimeError, Token};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Default)]
pub struct Environment {
    values: HashMap<String, ExprValue>,
    enclosing: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            values: HashMap::new(),
            enclosing: None,
        }
    }

    pub fn with_enclosing(enclosing: Rc<RefCell<Environment>>) -> Self {
        Environment {
            values: HashMap::new(),
            enclosing: Some(enclosing),
        }
    }

    pub fn define(&mut self, name: String, value: ExprValue) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &Token) -> Result<ExprValue, RuntimeError> {
        if let Some(val) = self.values.get(&name.lexeme) {
            Ok(val.clone())
        } else if let Some(enclosing) = &self.enclosing {
            enclosing.borrow().get(name)
        } else {
            Err(RuntimeError {
                token: name.lexeme.clone(),
                message: format!("Undefined variable '{}'.", name.lexeme),
                line: name.line,
            })
        }
    }

    pub fn assign(&mut self, name: &Token, value: ExprValue) -> Result<(), RuntimeError> {
        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme.clone(), value);
            Ok(())
        } else if let Some(enclosing) = &mut self.enclosing {
            enclosing.borrow_mut().assign(name, value)
        } else {
            Err(RuntimeError {
                token: name.lexeme.clone(),
                message: format!("Undefined variable '{}'", name.lexeme),
                line: name.line,
            })
        }
    }
}
