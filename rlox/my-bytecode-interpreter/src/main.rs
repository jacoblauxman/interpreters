use my_bytecode_interpreter::{disassemble, Chunk, OpCode, Value, Vm};

fn main() {
    let mut vm = Vm::new();

    vm.init();
    // let mut vm = Vm::init_vm();

    let mut chunk = Chunk::new();
    let line = 123;
    let mut constant = chunk.add_constant(Value::Number(1.2));

    chunk.write_chunk(OpCode::Constant as u8, line);
    chunk.write_chunk(constant as u8, line);
    constant = chunk.add_constant(Value::Number(3.4));
    chunk.write_chunk(OpCode::Constant as u8, line);
    chunk.write_chunk(constant, line);
    chunk.write_chunk(OpCode::Add as u8, line);
    constant = chunk.add_constant(Value::Number(5.6));
    chunk.write_chunk(OpCode::Constant as u8, line);
    chunk.write_chunk(constant, line);
    chunk.write_chunk(OpCode::Divide as u8, line);
    chunk.write_chunk(OpCode::Negate as u8, line);
    chunk.write_chunk(OpCode::Return as u8, line);
    disassemble(&chunk, "test chunk");

    if let Err(e) = vm.interpret(&chunk) {
        eprintln!("Error interpreting with VM: {e:?}");
    }

    std::process::exit(0)
}
