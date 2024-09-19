use crate::{Chunk, OpCode, Value};

pub fn disassemble(chunk: &Chunk, name: &str) {
    println!("== {} ==\n", name);

    let mut offset = 0;
    while offset < chunk.code.len() {
        offset = disassemble_instruction(chunk, offset);
    }
}

pub fn disassemble_instruction(chunk: &Chunk, offset: usize) -> usize {
    print!("{:04} ", offset);

    if offset > 0 && chunk.lines[offset] == chunk.lines[offset - 1] {
        print!("  | ")
    } else {
        print!("{:4} ", chunk.lines[offset]);
    }

    let instruction = OpCode::from(chunk.code[offset]);

    match instruction {
        OpCode::Return => simple_instruction(&"OP_RETURN", offset),
        OpCode::Constant => constant_instruction(&"OP_CONSTANT", chunk, offset),
    }
}

fn simple_instruction(name: &str, offset: usize) -> usize {
    println!("{}", name);
    offset + 1
}

fn constant_instruction(name: &str, chunk: &Chunk, offset: usize) -> usize {
    let constant = chunk.code[offset + 1];
    print!("{:-16} {:4} '", name, constant);
    print_value(&chunk.constants[constant as usize]);
    println!("'");
    offset + 2
}

pub fn print_value(value: &Value) {
    match value {
        Value::Number(n) => print!("{}", n),
    }
}
