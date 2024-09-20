mod chunk;
mod debug;
mod value;
mod vm;

pub use chunk::{Chunk, OpCode};
pub use debug::{disassemble, disassemble_instruction, print_value};
pub use value::Value;
pub use vm::Vm;

pub const DEBUG_TRACE_EXECUTION: bool = true;
