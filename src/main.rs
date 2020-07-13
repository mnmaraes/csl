mod vm;

use vm::{OpCode, Typed, VM};

fn main() {
    let mut vm = VM::default();

    vm.push_constant_op(Typed::Int(3), 1000);
    vm.push_op(OpCode::Return, 1000);

    let mut output = String::default();
    vm.disassemble(&mut output, "TEST");
    println!("{}", output);
}
