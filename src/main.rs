#![allow(non_camel_case_types)]

use machine::exit_reason_t;

mod decode;
mod elf;
mod insn;
mod interp;
mod machine;
mod mmu;
mod reg;
mod utils;

fn main() {
    if std::env::args().len() < 2 {
        println!("Usage: {} <program>", std::env::args().nth(0).unwrap());
        std::process::exit(1);
    }
    let args: Vec<String> = std::env::args().collect();
    let args_str: Vec<&str> = args.iter().map(|s| s.as_str()).collect();

            let mut machine: machine::machine_t = machine::machine_t::new();
    machine.machine_load_program(args_str[1]);
    machine.machine_setup(args.len() as u64, &args_str);

            loop {
                let reason: exit_reason_t = machine.machine_step();
                assert_eq!(reason, exit_reason_t::ecall);

        
    }
}
