use std::fmt::{Display, Formatter};

mod debug;
use debug::disassemble_instruction;
pub use debug::{disassemble, print_value};

const DEBUG_TRACE_EXECUTION: bool = true;

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

#[derive(Debug)]
pub struct Chunk {
    pub code: Vec<u8>, // Vec handles 'count' and 'capacity'
    pub constants: Vec<Value>,
    pub lines: Vec<usize>,
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
    pub fn add_constant(&mut self, value: Value) -> u8 {
        self.constants.push(value);
        (self.constants.len() - 1) as u8
    }
}

pub struct VM<'a> {
    chunk: Option<&'a Chunk>,
    // ip: usize,
    ip: *const u8,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}

impl<'a> VM<'a> {
    pub fn new() -> Self {
        Self {
            chunk: None,
            ip: std::ptr::null(),
        }
    }

    pub fn interpret(&mut self, chunk: &'a Chunk) -> InterpretResult {
        self.chunk = Some(chunk);
        self.ip = chunk.code.as_ptr();

        self.run()
    }

    fn run(&mut self) -> InterpretResult {
        loop {
            if DEBUG_TRACE_EXECUTION {
                if let Some(chunk) = self.chunk {
                    let code_start = chunk.code.as_ptr();
                    // SAFETY: `code_end` is calculated with the `len` of the `code` Vec, this ensures it's a valid location in `chunk.code`'s bounds
                    // and guarantees `code_end` points to a valid memory addr withink `chunk.code`.
                    let code_end = unsafe { code_start.add(chunk.code.len()) };
                    if self.ip.is_null() || self.ip < chunk.code.as_ptr() || self.ip >= code_end {
                        return InterpretResult::RuntimeError;
                    }
                    // SAFETY: `offset_from` calculates diff/offset of instruction ptr `ip` relative to start of (chunk's) `code` Vec.
                    // We have confimed `ip` is not null and points within the valid range of chunk's `code` Vec.
                    let offset = unsafe { self.ip.offset_from(chunk.code.as_ptr()) as usize };
                    disassemble_instruction(chunk, offset);
                }
            }
            let instruction = self.read_byte();

            match OpCode::from(instruction) {
                OpCode::Return => return InterpretResult::Ok,
                OpCode::Constant => {
                    let constant = self.read_constant();
                    print_value(&constant);
                    println!("");
                    return InterpretResult::Ok;
                }
            }
        }
    }

    fn read_byte(&mut self) -> u8 {
        unsafe {
            let byte = *self.ip;
            self.ip = self.ip.add(1);

            byte
        }
    }

    fn read_constant(&mut self) -> Value {
        let const_idx = self.read_byte();
        self.chunk
            .expect("`VM` should have assigned `Chunk`")
            .constants[const_idx as usize]
    }
}
