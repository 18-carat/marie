use crate::binout::BinaryOutput;
use crate::instruction::Instruction;
use std::collections::HashMap;

pub struct Assembler {
    addrs: HashMap<String, usize>,
    bin: [i16; 1000],
    pos: usize,
    vars: HashMap<String, (Vec<usize>, i16)>,
}

impl Assembler {
    pub fn new() -> Self {
        Self {
            addrs: HashMap::new(),
            bin: [0; 1000],
            pos: 0,
            vars: HashMap::new(),
        }
    }

    pub fn assemble(&mut self, source: &str) -> BinaryOutput {
        for line in source.lines() {
            if line.is_empty() || line.starts_with(';') {
                continue;
            }

            let line = line.to_lowercase();
            let mut parts = line.split_whitespace();
            let mnem = parts.next().unwrap();

            if let Ok(x) = mnem.parse::<i16>() {
                self.bin[self.pos] = x;
                self.pos += 1;
                continue;
            }

            if mnem.ends_with(':') {
                self.add_label(mnem);
                continue;
            }

            if let Some(x) = parts.next() {
                self.add_insn_with_operand(mnem, x);
            } else {
                self.add_insn(mnem, None);
            }

            self.pos += 1;
        }

        self.addr_vars();
        self.resolve_vars();

        BinaryOutput::new(self.bin, self.pos)
    }

    fn add_var(&mut self, var: &str, pos: usize, val: i16) {
        if let Some(p) = self.vars.get_mut(var) {
            p.0.push(pos);
        } else {
            self.vars.insert(var.to_string(), (vec![pos], val));
        }
    }

    fn add_insn(&mut self, mnem: &str, addr: Option<i16>) {
        let addr = addr.unwrap_or_default();
        let insn = Instruction::assemble(mnem, addr).unwrap();

        self.bin[self.pos] = insn.machine_code();
    }

    fn add_insn_with_operand(&mut self, mnem: &str, x: &str) {
        if let Ok(addr) = x.parse::<i16>() {
            self.add_insn(mnem, Some(addr));
            return;
        }

        if x.starts_with(';') {
            self.add_insn(mnem, None);
            return;
        }

        if x.starts_with('#') {
            let x = x[1..x.len()].to_owned();
            let val = x.parse::<i16>().unwrap();

            self.add_insn(mnem, None);
            self.add_var(&x, self.pos, val);

            return;
        }

        self.add_insn(mnem, None);
        self.add_var(x, self.pos, 0);
    }

    fn add_label(&mut self, label: &str) {
        let var = label[..label.len() - 1].to_owned();

        if self.addrs.get(&var).is_none() {
            self.addrs.insert(var, self.pos);
        } else {
            panic!("Duplicate label: {}", label);
        }
    }

    fn addr_vars(&mut self) {
        self.vars.iter().for_each(|(var, (_, val))| {
            if self.addrs.get(var).is_none() {
                self.addrs.insert(var.to_string(), self.pos);
            }

            self.bin[self.pos] = *val;
            self.pos += 1;
        });
    }

    fn resolve_vars(&mut self) {
        self.vars.iter().for_each(|(var, pos)| {
            pos.0.iter().for_each(|p| {
                let addr = self.addrs.get(var).unwrap();
                self.bin[*p] += *addr as i16;
            });
        });
    }
}
