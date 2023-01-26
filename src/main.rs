enum OpCode {
    Return,
}

impl From<OpCode> for u8 {
    fn from(value: OpCode) -> Self {
        match value {
            OpCode::Return => 0,
        }
    }
}

impl TryFrom<u8> for OpCode {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(OpCode::Return),
            _ => Err(format!("Unknown opcode {value}")),
        }
    }
}

struct Chunk {
    code: Vec<u8>,
}

impl Chunk {
    pub fn new() -> Self {
        Self { code: Vec::new() }
    }

    pub fn write(&mut self, byte: u8) {
        self.code.push(byte);
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
        let instruction = self.code[offset];
        match OpCode::try_from(instruction) {
            Ok(opcode) => match opcode {
                OpCode::Return => simple_instruction(f, "OP_RETURN", offset),
            },
            Err(_) => {
                f.push_str(&format!("Unknown opcode {instruction}\n"));
                offset + 1
            }
        }
    }
}

fn simple_instruction(f: &mut String, name: &str, offset: usize) -> usize {
    f.push_str(&format!("{name}\n"));
    offset + 1
}

fn main() {
    let mut chunk = Chunk::new();
    chunk.write(OpCode::Return.into());
    println!("{}", chunk.disassemble("test chunk"));
}
