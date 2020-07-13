#[derive(Debug)]
pub enum OpCode {
    Return,
    Constant(usize),
}

impl OpCode {
    fn description(&self, constants: &[Typed]) -> String {
        use OpCode::*;

        match self {
            Return => "RETURN".into(),
            Constant(idx) => format!("CONSTANT {:04} ; {}", idx, constants[*idx].description()),
        }
    }

    pub fn disassemble(&self, w: &mut dyn std::fmt::Write, prefix: String, constants: &[Typed]) {
        writeln!(w, "{}: {}", prefix, self.description(constants)).unwrap();
    }
}

pub struct CodeLocation {
    pub line: u16,
    pub op_count: u8,
}

impl CodeLocation {
    pub fn new(line: u16) -> Self {
        CodeLocation { line, op_count: 1 }
    }

    pub fn inc(&mut self) {
        self.op_count += 1;
    }
}

pub enum Typed {
    Int(isize),
}

impl Typed {
    pub fn description(&self) -> String {
        use Typed::*;

        match self {
            Int(v) => format!("(Int) {}", v),
        }
    }
}
