#![forbid(unsafe_code)]

mod assembler;
mod binout;
mod disassembler;
mod instruction;
mod machine;

use crate::assembler::Assembler;
use crate::binout::BinaryOutput;
use crate::disassembler::disassemble;
use crate::machine::Machine;
use pico_args::Arguments;
use std::fs::{read, read_to_string, remove_file, write};

fn main() {
    let mut args = Arguments::from_env();

    let input: Option<String> = args.value_from_str(["-i", "--input"]).ok();
    let asm: bool = args.contains(["-a", "--assemble"]);
    let dis: bool = args.contains(["-d", "--disassemble"]);
    let exec: bool = args.contains(["-x", "--execute"]);
    let help: bool = args.contains(["-h", "--help"]);

    if help || input.is_none() || (!asm && !dis && !exec) {
        print_help();
    }

    let mut input = input.unwrap();

    if asm {
        let src = read_to_string(&input).unwrap();
        let mut asm = Assembler::new();
        let compiled = asm.assemble(&src);

        write("out.o", compiled.bytes).unwrap();
        input = "out.o".to_string();
    }

    if dis {
        let binary = read(&input).unwrap();
        let dis = disassemble(binary);

        if input == "out.o" {
            remove_file("out.o").unwrap();
        }

        println!("{}", dis);
        return;
    }

    if exec {
        let file = read(&input).unwrap();
        let bin = BinaryOutput::from_bytes(file);
        let mut machine = Machine::new(bin.code);

        if input == "out.o" {
            remove_file("out.o").unwrap();
        }

        machine.run();
        return;
    }

    print_help();
}

fn print_help() -> ! {
    println!("Usage: ./marie [options] [-i, --input] <file>");
    println!();
    println!("Options:");
    println!("  -a, --assemble        Assemble MARIE++ code or text file");
    println!("  -d, --disassemble     Disassemble compiled machine code");
    println!("  -x, --execute         Execute compiled machine code");
    println!("  -h, --help            Display this message");

    std::process::exit(0);
}
