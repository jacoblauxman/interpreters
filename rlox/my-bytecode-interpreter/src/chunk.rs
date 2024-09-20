use crate::Value;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum OpCode {
    Return,
    Constant,
    Negate,
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl From<u8> for OpCode {
    fn from(value: u8) -> Self {
        match value {
            0 => OpCode::Return,
            1 => OpCode::Constant,
            2 => OpCode::Negate,
            3 => OpCode::Add,
            4 => OpCode::Subtract,
            5 => OpCode::Multiply,
            6 => OpCode::Divide,
            _ => panic!("Unknown OpCode: {}", value),
        }
    }
}

#[derive(Debug)]
pub struct Chunk {
    pub code: Vec<u8>, // Vec handles 'count' and 'capacity'
    pub constants: Vec<Value>,
    pub lines: Vec<usize>,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            code: Vec::with_capacity(8),
            constants: Vec::with_capacity(8),
            lines: Vec::with_capacity(8),
        }
    }

    pub fn write_chunk(&mut self, byte: u8, line: usize) {
        self.code.push(byte);
        self.lines.push(line);
    }

    // returns const's idx in vec
    pub fn add_constant(&mut self, value: Value) -> u8 {
        self.constants.push(value);
        let idx = (self.constants.len() - 1) as u8;
        idx
    }
}
