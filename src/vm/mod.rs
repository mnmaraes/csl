mod chunk;
mod op;

pub use chunk::Chunk;
pub use op::{OpCode, Typed};

#[derive(Debug)]
pub enum InterpretError {
    Compile,
    Runtime(String),
}

#[derive(Default)]
pub struct VM;

impl VM {
    pub fn interpret(&self, chunk: &Chunk) -> Result<(), InterpretError> {
        for instruction in chunk.op_iter() {
            use OpCode::*;

            match instruction {
                Return => return Ok(()),
                Constant(addr) => {
                    let constant = chunk.get_const(*addr);
                }
                _ => {
                    return Err(InterpretError::Runtime(format!(
                        "Unknown operation: {:?}",
                        instruction,
                    )))
                }
            }
        }

        Err(InterpretError::Runtime(
            "Unexpected end of program".to_string(),
        ))
    }
}
