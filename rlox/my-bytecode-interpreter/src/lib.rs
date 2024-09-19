use std::fmt::{Display, Formatter};

mod debug;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum OpCode {
    Return,
    Constant,
}

impl From<u8> for OpCode {
    fn from(value: u8) -> Self {
        match value {
            0 => OpCode::Constant,
            1 => OpCode::Return,
            _ => panic!("Unknown OpCode: {}", value),
        }
    }
}

#[derive(Debug)]
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

#[derive(Debug)]
pub struct Chunk {
    code: Vec<u8>, // Vec handles 'count' and 'capacity'
    constants: Vec<Value>,
    lines: Vec<usize>,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            code: Vec::new(),
            constants: Vec::new(),
            lines: Vec::new(),
        }
    }

    pub fn write_chunk(&mut self, byte: u8, line: usize) {
        self.code.push(byte);
        self.lines.push(line);
    }

    // returns const's idx in vec
    pub fn add_constant(&mut self, value: Value) -> usize {
        self.constants.push(value);
        self.constants.len() - 1
    }
}

// pub fn disassemble(chunk: &Chunk, name: &str) {
//     println!("== {} ==\n", name);

//     let mut offset = 0;
//     while offset < chunk.code.len() {
//         offset = disassemble_instruction(chunk, offset);
//     }
// }

// fn disassemble_instruction(chunk: &Chunk, offset: usize) -> usize {
//     println!("{:04}", offset);

//     let instruction = chunk.code[offset];

//     match OpCode::from(instruction) {
//         OpCode::Return => simple_instruction(&"OP_RETURN", offset),
//         OpCode::Constant => todo!(),
//     }

//     // let instruction = OpCode::from(chunk.code[offset]);
// }

// fn simple_instruction(name: &str, offset: usize) -> usize {
//     println!("{}", name);
//     offset + 1
// }

// fn constant_instruction(name: &str, chunk: &Chunk, offset: usize) -> usize {
//     // let constant = chunk
//     //     .code
//     //     .get(offset + 1)
//     //     .expect("should find constant value within code chunk");
//     let constant = chunk.code[offset + 1];
//     println!("{:-16} {:4}", name, constant);
//     print_value(&chunk.constants[constant as usize]);
//     println!("'");

//     offset + 2
// }

// fn print_value(value: &Value) {
//     match value {
//         Value::Number(n) => println!("{}", n),
//     }
// }
