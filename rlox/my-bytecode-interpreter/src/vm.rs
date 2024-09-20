//vm.rs
use crate::{disassemble_instruction, print_value, Chunk, OpCode, Value, DEBUG_TRACE_EXECUTION};

pub const STACK_MAX: usize = 256;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}

pub struct Vm<'a> {
    chunk: Option<&'a Chunk>,
    ip: *const u8,
    stack: [Value; STACK_MAX],
    stack_top: *mut Value,
}

impl<'a> Vm<'a> {
    pub fn new() -> Self {
        Self {
            chunk: None,
            ip: std::ptr::null(),
            stack: [Value::default(); STACK_MAX],
            stack_top: std::ptr::null_mut(),
        }
    }

    pub fn init(&mut self) {
        self.reset_stack();
        println!("VM init'd: stack_top after reset: {:?}", self.stack_top);
    }

    pub fn reset_stack(&mut self) {
        self.stack_top = self.stack.as_mut_ptr();
    }

    fn push(&mut self, value: Value) -> Result<(), InterpretResult> {
        let stack_size = unsafe { self.stack_top.offset_from(self.stack.as_ptr()) };
        if stack_size >= STACK_MAX as isize {
            return Err(InterpretResult::RuntimeError);
        }
        unsafe {
            *self.stack_top = value;
            self.stack_top = self.stack_top.add(1);
        }

        Ok(())
    }

    fn pop(&mut self) -> Result<Value, InterpretResult> {
        let stack_size = unsafe { self.stack_top.offset_from(self.stack.as_ptr()) };
        if stack_size <= 0 {
            return Err(InterpretResult::RuntimeError);
        }
        unsafe {
            self.stack_top = self.stack_top.sub(1);
            let value = *self.stack_top;

            // Ok(*self.stack_top)
            Ok(value)
        }
    }

    pub fn interpret(&mut self, chunk: &'a Chunk) -> Result<InterpretResult, InterpretResult> {
        self.chunk = Some(chunk);
        self.ip = chunk.code.as_ptr();

        let res = self.run();
        match res {
            Ok(res) => Ok(res),
            Err(e) => {
                eprintln!("Runtime error: {:?}", e);
                Err(e)
            }
        }
    }

    fn run(&mut self) -> Result<InterpretResult, InterpretResult> {
        loop {
            if DEBUG_TRACE_EXECUTION {
                print!("          ");

                let stack_size = unsafe { self.stack_top.offset_from(self.stack.as_ptr()) };

                if stack_size >= 0 && stack_size <= STACK_MAX as isize {
                    for slot in &self.stack[..stack_size as usize] {
                        print!("[ ");
                        print_value(slot);
                        print!(" ]");
                    }
                    println!("");
                } else {
                    println!("STACK ERROR (size: {}", stack_size);
                }

                if let Some(chunk) = self.chunk {
                    let offset = unsafe { self.ip.offset_from(chunk.code.as_ptr()) } as usize;
                    disassemble_instruction(chunk, offset);
                }
                // if let Some(chunk) = self.chunk {
                //     let code_start = chunk.code.as_ptr();
                //     if code_start.is_null() {
                //         return Err(InterpretResult::RuntimeError);
                //     }
                //     // SAFETY: `code_end` is calculated with the `len` of the `code` Vec, this ensures it's a valid location in `chunk.code`'s bounds
                //     // and guarantees `code_end` points to a valid memory addr withink `chunk.code`.
                //     let code_end = unsafe { code_start.add(chunk.code.len()) };
                //     if self.ip.is_null() || self.ip < chunk.code.as_ptr() || self.ip >= code_end {
                //         return Err(InterpretResult::RuntimeError);
                //     }
                //     // SAFETY: `offset_from` calculates diff/offset of instruction ptr `ip` relative to start of (chunk's) `code` Vec.
                //     // We have confimed `ip` is not null and points within the valid range of chunk's `code` Vec.
                //     let offset = unsafe { self.ip.offset_from(chunk.code.as_ptr()) as usize };
                //     disassemble_instruction(chunk, offset);
                // }
            }
            let instruction = self.read_byte();

            match OpCode::from(instruction) {
                OpCode::Return => {
                    print_value(&self.pop()?);
                    println!("");
                    return Ok(InterpretResult::Ok);
                }
                OpCode::Constant => {
                    let constant = self.read_constant()?;
                    self.push(constant)?;
                }
                OpCode::Negate => {
                    let value = self.pop()?;
                    match value {
                        Value::Number(n) => {
                            self.push(Value::Number(-n))?;
                        }
                    }
                }
                OpCode::Add => {
                    self.binary_operation('+')?;
                }
                OpCode::Subtract => {
                    self.binary_operation('-')?;
                }
                OpCode::Multiply => {
                    self.binary_operation('*')?;
                }
                OpCode::Divide => {
                    self.binary_operation('/')?;
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

    fn read_constant(&mut self) -> Result<Value, InterpretResult> {
        let const_idx = self.read_byte();
        if const_idx as usize
            >= self
                .chunk
                .as_ref()
                .expect("`Vm` should have assigned `Chunk`")
                .constants
                .len()
        {
            return Err(InterpretResult::RuntimeError);
        }
        Ok(self
            .chunk
            .expect("`Vm` should have assigned `Chunk`")
            .constants[const_idx as usize])
    }

    fn binary_operation(&mut self, op: char) -> Result<InterpretResult, InterpretResult> {
        let b = self.pop()?;
        let a = self.pop()?;

        match (a, b) {
            (Value::Number(a_val), Value::Number(b_val)) => {
                let op_res = match op {
                    '+' => a_val + b_val,
                    '-' => a_val - b_val,
                    '*' => a_val * b_val,
                    '/' => a_val / b_val,
                    _ => return Err(InterpretResult::RuntimeError),
                };

                self.push(Value::Number(op_res))?;
                Ok(InterpretResult::Ok)
            }
        }
    }
}
