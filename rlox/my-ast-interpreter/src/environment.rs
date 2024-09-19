use crate::{ExprValue, RuntimeError, Token};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Environment {
    values: HashMap<String, ExprValue>,
    pub enclosing: Option<Rc<RefCell<Environment>>>,
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
            Err(RuntimeError::RTE {
                token: name.lexeme.clone(),
                message: format!("Undefined variable '{}'.", name.lexeme),
                line: name.line,
            })
        }
    }

    pub fn get_at(&self, distance: usize, name: &Token) -> ExprValue {
        self.ancestor(distance)
            .borrow()
            .values
            .get(&name.lexeme)
            .expect("should find a valid variable expression value")
            .clone()
    }

    fn ancestor(&self, distance: usize) -> Rc<RefCell<Environment>> {
        // let mut environment = self;
        // let mut environment = self.clone();
        let mut environment = Rc::new(RefCell::new(self.clone()));

        for _ in 0..distance {
            // if let Some(enclosing) = environment.enclosing {
            //     environment = enclosing.clone();
            // }
            let enclosing = environment.borrow().enclosing.clone();
            environment = enclosing.expect("should find enclosing environment");
        }

        environment
    }

    pub fn assign(&mut self, name: &Token, value: ExprValue) -> Result<(), RuntimeError> {
        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme.clone(), value);
            Ok(())
        } else if let Some(enclosing) = &mut self.enclosing {
            enclosing.borrow_mut().assign(name, value)
        } else {
            Err(RuntimeError::RTE {
                token: name.lexeme.clone(),
                message: format!("Undefined variable '{}'", name.lexeme),
                line: name.line,
            })
        }
    }

    pub fn assign_at(&self, distance: usize, name: &Token, value: ExprValue) {
        self.ancestor(distance)
            .borrow_mut()
            .values
            .insert(name.lexeme.clone(), value);
    }
}
