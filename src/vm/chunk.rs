use super::op::{CodeLocation, OpCode, Typed};

#[derive(Default)]
pub struct Chunk {
    code: Vec<OpCode>,
    constants: Vec<Typed>,
    loc: Vec<CodeLocation>,
}

impl Chunk {
    pub fn push_op(&mut self, op: OpCode, line: u16) {
        match self.loc.last_mut() {
            Some(loc) if loc.line == line => loc.inc(),
            _ => self.loc.push(CodeLocation::new(line)),
        }

        self.code.push(op);
    }

    pub fn push_constant_op(&mut self, constant: Typed, line: u16) {
        self.constants.push(constant);
        self.push_op(OpCode::Constant(self.constants.len() - 1), line);
    }

    pub fn disassemble(&self, w: &mut dyn std::fmt::Write, name: &str) {
        writeln!(w, "== {} ==", name).unwrap();

        for (line, is_first, op) in self.disassembler() {
            let prefix = if is_first {
                format!("{:>4}", line)
            } else {
                "   |".into()
            };
            op.disassemble(w, prefix, &self.constants);
        }
    }

    pub fn get_const(&self, addr: usize) -> &Typed {
        &self.constants[addr]
    }

    #[cfg(debug_assertions)]
    pub fn op_iter(&self) -> impl Iterator<Item = &OpCode> {
        self.disassembler().map(move |(line, is_first, op)| {
            let prefix = if is_first {
                format!("{:>4}", line)
            } else {
                "   |".into()
            };

            let mut w = String::default();
            op.disassemble(&mut w, prefix, &self.constants);
            print!("{}", w);

            op
        })
    }

    #[cfg(not(debug_assertions))]
    pub fn op_iter(&self) -> impl Iterator<Item = &OpCode> {
        OpIter::new(&self.code)
    }

    fn disassembler(&self) -> Disassembler {
        Disassembler::new(&self.code, &self.loc)
    }
}

pub struct OpIter<'a> {
    op_index: usize,

    code: &'a [OpCode],
}

impl<'a> OpIter<'a> {
    fn new(code: &'a [OpCode]) -> Self {
        OpIter { op_index: 0, code }
    }
}

impl<'a> Iterator for OpIter<'a> {
    type Item = &'a OpCode;

    fn next(&mut self) -> Option<Self::Item> {
        if self.code.len() <= self.op_index {
            None
        } else {
            let op_code = &self.code[self.op_index];
            self.op_index += 1;
            Some(op_code)
        }
    }
}

struct Disassembler<'a> {
    op_index: usize,
    loc_index: usize,
    loc_curr_count: u8,

    code: &'a [OpCode],
    loc: &'a [CodeLocation],
}

impl<'a> Disassembler<'a> {
    pub fn new(code: &'a [OpCode], loc: &'a [CodeLocation]) -> Self {
        Disassembler {
            op_index: 0,
            loc_index: 0,
            loc_curr_count: 0,

            code,
            loc,
        }
    }
}

impl<'a> Iterator for Disassembler<'a> {
    type Item = (u16, bool, &'a OpCode);

    fn next(&mut self) -> Option<Self::Item> {
        if self.code.len() <= self.op_index {
            return None;
        }

        // Get Information
        let loc = &self.loc[self.loc_index];
        let is_first = self.loc_curr_count == 0;
        let op_code = &self.code[self.op_index];

        // Update Indexes
        self.op_index += 1;
        self.loc_curr_count += 1;

        if self.loc_curr_count == loc.op_count {
            self.loc_index += 1;
            self.loc_curr_count = 0;
        }

        Some((loc.line, is_first, op_code))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_disassemble_simple() {
        let mut chunk = Chunk::default();

        chunk.push_constant_op(Typed::Int(3), 1);
        chunk.push_op(OpCode::Return, 1);

        let mut output = String::default();
        chunk.disassemble(&mut output, "TEST");

        assert_eq!(
            output,
            "== TEST ==\n   \
            1: CONSTANT 0000 ; (Int) 3\n   \
            |: RETURN\n"
        );
    }

    #[test]
    fn can_disassemble_multiline() {
        let mut chunk = Chunk::default();

        chunk.push_constant_op(Typed::Int(3), 1);
        chunk.push_op(OpCode::Return, 1);
        chunk.push_op(OpCode::Return, 2);
        chunk.push_op(OpCode::Return, 2);
        chunk.push_op(OpCode::Return, 2);
        chunk.push_op(OpCode::Return, 3);
        chunk.push_op(OpCode::Return, 4);

        let mut output = String::default();
        chunk.disassemble(&mut output, "TEST");

        assert_eq!(
            output,
            "== TEST ==\n   \
            1: CONSTANT 0000 ; (Int) 3\n   \
            |: RETURN\n   \
            2: RETURN\n   \
            |: RETURN\n   \
            |: RETURN\n   \
            3: RETURN\n   \
            4: RETURN\n"
        );
    }
}
