use my_bytecode_interpreter::{disassemble, Chunk, OpCode, Value};

fn main() {
    let mut chunk = Chunk::new();

    let line = 123;
    let constant = chunk.add_constant(Value::Number(1.2));
    chunk.write_chunk(OpCode::Constant as u8, line);

    chunk.write_chunk(constant as u8, line);

    chunk.write_chunk(OpCode::Return as u8, line);

    disassemble(&chunk, "test chunk");
}
