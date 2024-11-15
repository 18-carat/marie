#[derive(PartialEq, Eq)]
pub enum Instruction {
    Load(i16),
    Store(i16),
    Add(i16),
    Subt(i16),
    Input,
    Output,
    Halt,
    Skipcond(i16),
    Jump(i16),
}

impl Instruction {
    pub fn decode(word: i16) -> Option<Self> {
        let opcode = word / 1000;
        let addr = word % 1000;

        match opcode {
            1 => Some(Instruction::Load(addr)),
            2 => Some(Instruction::Store(addr)),
            3 => Some(Instruction::Add(addr)),
            4 => Some(Instruction::Subt(addr)),
            5 => Some(Instruction::Input),
            6 => Some(Instruction::Output),
            7 => Some(Instruction::Halt),
            8 => Some(Instruction::Skipcond(addr)),
            9 => Some(Instruction::Jump(addr)),
            _ => None,
        }
    }

    pub fn assemble(mnem: &str, addr: i16) -> Option<Self> {
        match mnem {
            "load" => Some(Instruction::Load(addr)),
            "store" => Some(Instruction::Store(addr)),
            "add" => Some(Instruction::Add(addr)),
            "subt" => Some(Instruction::Subt(addr)),
            "input" => Some(Instruction::Input),
            "output" => Some(Instruction::Output),
            "halt" => Some(Instruction::Halt),
            "skipcond" => Some(Instruction::Skipcond(addr)),
            "jump" => Some(Instruction::Jump(addr)),
            _ => None,
        }
    }

    pub fn machine_code(&self) -> i16 {
        match self {
            Instruction::Load(addr) => 1000 + addr,
            Instruction::Store(addr) => 2000 + addr,
            Instruction::Add(addr) => 3000 + addr,
            Instruction::Subt(addr) => 4000 + addr,
            Instruction::Input => 5000,
            Instruction::Output => 6000,
            Instruction::Halt => 7000,
            Instruction::Skipcond(cond) => 8000 + cond,
            Instruction::Jump(addr) => 9000 + addr,
        }
    }
}
