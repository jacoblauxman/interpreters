use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Copy)]
pub enum Value {
    Number(f64),
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
        }
    }
}

impl Default for Value {
    fn default() -> Self {
        Value::Number(0.0)
    }
}
