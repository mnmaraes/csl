mod vm;

use vm::{Chunk, InterpretError, OpCode, Typed, VM};

fn main() -> Result<(), InterpretError> {
    let mut vm = VM::default();
    let mut c = Chunk::default();

    c.push_constant_op(Typed::Int(15), 1);
    c.push_op(OpCode::Return, 1);

    vm.interpret(&c)?;

    Ok(())
}
