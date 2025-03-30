#![allow(non_camel_case_types)]

use machine::exit_reason_t;
use reg::gp_reg_type_t;

mod decode;
mod elf;
mod insn;
mod interp;
mod machine;
mod mmu;
mod reg;
mod syscall;
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
    println!("sysx {:#x}", machine.mmu.alloc);
    machine.machine_setup(args.len() as u64, &args_str);
    println!("sysx {:#x}", machine.mmu.alloc);


    loop {
        let reason: exit_reason_t = machine.machine_step();
        assert_eq!(reason, exit_reason_t::ecall);

        let syscall_num: u64 = machine.machine_get_gp_reg(reg::gp_reg_type_t::a7);
        let ret: u64 = machine.do_syscall(syscall_num);
        machine.machine_set_gp_reg(gp_reg_type_t::a0, ret);
    }
}
