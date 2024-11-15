use crate::instruction::Instruction;

pub fn disassemble(bin: Vec<u8>) -> String {
    let mut dis = String::new();

    bin.chunks_exact(2).for_each(|c| {
        let c0: i16 = c[0].into();
        let c1: i16 = c[1].into();
        let word = c0 * 100 + c1;

        if let Some(insn) = Instruction::decode(word) {
            let mnem = match insn {
                Instruction::Load(x) => format!("load {}", x),
                Instruction::Store(x) => format!("store {}", x),
                Instruction::Add(x) => format!("add {}", x),
                Instruction::Subt(x) => format!("subt {}", x),
                Instruction::Input => "input".to_string(),
                Instruction::Output => "output".to_string(),
                Instruction::Halt => "halt".to_string(),
                Instruction::Skipcond(x) => format!("skipcond {}", x),
                Instruction::Jump(x) => format!("jump {}", x),
            };

            dis.push_str(&mnem);
        } else {
            dis.push_str(&word.to_string());
        }

        dis.push('\n');
    });

    dis
}
