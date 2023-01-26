enum OpCode {
    Constant,
    Return,
}

impl From<OpCode> for u8 {
    fn from(value: OpCode) -> Self {
        match value {
            OpCode::Constant => 0,
            OpCode::Return => 1,
        }
    }
}

impl TryFrom<u8> for OpCode {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(OpCode::Constant),
            1 => Ok(OpCode::Return),
            _ => Err(format!("Unknown opcode {value}")),
        }
    }
}

type Value = f64;

struct Chunk {
    code: Vec<u8>,
    constants: Vec<Value>,
    lines: Vec<u32>,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            code: Vec::new(),
            constants: Vec::new(),
            lines: Vec::new(),
        }
    }

    pub fn write(&mut self, byte: u8, line: u32) {
        self.code.push(byte);
        self.lines.push(line);
    }

    pub fn add_constant(&mut self, value: Value) -> u8 {
        self.constants.push(value);
        self.constants.len() as u8 - 1
    }

    pub fn disassemble(&self, name: &str) -> String {
        let mut f = format!("== {name} ==\n");

        let mut offset = 0;

        while offset < self.code.len() {
            offset = self.disassemble_instruction(&mut f, offset);
        }

        f
    }

    fn disassemble_instruction(&self, f: &mut String, offset: usize) -> usize {
        f.push_str(&format!("{offset:04} "));

        if offset > 0 && self.lines[offset] == self.lines[offset - 1] {
            f.push_str("   | ");
        } else {
            f.push_str(&format!("{line:4} ", line = self.lines[offset]));
        }

        let instruction = self.code[offset];

        match OpCode::try_from(instruction) {
            Ok(opcode) => match opcode {
                OpCode::Constant => self.constant_instruction(f, "OP_CONSTANT", offset),
                OpCode::Return => simple_instruction(f, "OP_RETURN", offset),
            },
            Err(_) => {
                f.push_str(&format!("Unknown opcode {instruction}\n"));
                offset + 1
            }
        }
    }

    fn constant_instruction(&self, f: &mut String, name: &str, offset: usize) -> usize {
        let constant_index = self.code[offset + 1] as usize;

        f.push_str(&format!(
            "{name:-16} {constant_index:4} '{constant}'\n",
            constant = self.constants.get(constant_index).unwrap()
        ));

        offset + 2
    }
}

fn simple_instruction(f: &mut String, name: &str, offset: usize) -> usize {
    f.push_str(&format!("{name}\n"));
    offset + 1
}

fn main() {
    let mut chunk = Chunk::new();
    let constant_index = chunk.add_constant(1.2);
    chunk.write(OpCode::Constant.into(), 123);
    chunk.write(constant_index, 123);
    chunk.write(OpCode::Return.into(), 123);
    println!("{}", chunk.disassemble("test chunk"));
}
