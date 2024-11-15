use crate::instruction::Instruction;

pub struct Machine {
    mem: [i16; 1000],
    ac: i16,
    ir: i16,
    mar: i16,
    mbr: i16,
    pc: i16,
}

impl Machine {
    pub fn new(bin: [i16; 1000]) -> Self {
        Self {
            mem: bin,
            ac: 0,
            ir: 0,
            mar: 0,
            mbr: 0,
            pc: 0,
        }
    }

    pub fn run(&mut self) {
        loop {
            self.fetch();
            let insn = self.decode();
            self.execute(insn);
        }
    }

    fn fetch(&mut self) {
        self.ir = self.mem[self.pc as usize];
        self.pc += 1;
    }

    fn decode(&mut self) -> Instruction {
        Instruction::decode(self.ir).unwrap()
    }

    fn execute(&mut self, insn: Instruction) {
        match insn {
            Instruction::Load(addr) => self.load(addr),
            Instruction::Store(addr) => self.store(addr),
            Instruction::Add(addr) => self.add(addr),
            Instruction::Subt(addr) => self.subt(addr),
            Instruction::Input => self.input(),
            Instruction::Output => self.output(),
            Instruction::Halt => self.halt(),
            Instruction::Skipcond(cond) => self.skipcond(cond),
            Instruction::Jump(addr) => self.jump(addr),
        }
    }

    fn halt(&mut self) {
        std::process::exit(0);
    }

    fn load(&mut self, addr: i16) {
        self.mar = addr;
        self.mbr = self.mem[self.mar as usize];
        self.ac = self.mbr;
    }

    fn store(&mut self, addr: i16) {
        self.mar = addr;
        self.mbr = self.ac;
        self.mem[self.mar as usize] = self.mbr;
    }

    fn add(&mut self, addr: i16) {
        self.mar = addr;
        self.mbr = self.mem[self.mar as usize];
        self.ac += self.mbr;
    }

    fn subt(&mut self, addr: i16) {
        self.mar = addr;
        self.mbr = self.mem[self.mar as usize];
        self.ac -= self.mbr;
    }

    fn input(&mut self) {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        self.ac = input.trim().parse().unwrap();
    }

    fn output(&mut self) {
        println!("{}", self.ac);
    }

    fn skipcond(&mut self, cond: i16) {
        let cond = cond / 100;
        let ac = self.ac.clamp(-1, 1) + 1;

        if ac == cond {
            self.pc += 1;
        }
    }

    fn jump(&mut self, addr: i16) {
        self.pc = addr;
    }
}
