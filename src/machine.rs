use crate::{interp::exec_block_interp, mmu::mmu_t, reg::{fp_reg_type_t, gp_reg_type_t}};
use core::panic;
use std::fs::File;

const GUEST_MEMORY_OFFSET: u64 = 0x0888_0000_0000;

#[inline]
pub fn to_host_addr(addr: u64) -> u64 {
    addr + GUEST_MEMORY_OFFSET
}

#[inline]
pub fn to_guest_addr(addr: u64) -> u64 {
    addr - GUEST_MEMORY_OFFSET
}

#[derive(PartialEq, Debug)]
pub enum exit_reason_t {
    none,
    direct_branch,
    indirect_branch,
    ecall,
}

pub struct state_t {
    pub exit_reason: exit_reason_t,
    pub reenter_pc: u64,
    pub gp_regs: [u64; gp_reg_type_t::num_gp_regs as usize],
    pub fp_regs: [f64; fp_reg_type_t::num_fp_regs as usize],
    pub pc: u64,
}

pub struct machine_t {
    state: state_t,
    mmu: mmu_t,
}

impl machine_t {
    pub fn new() -> machine_t {
        machine_t {
            state: state_t {
                exit_reason: exit_reason_t::none,
                reenter_pc: 0,
                gp_regs: [0; gp_reg_type_t::num_gp_regs as usize],
                fp_regs: [0.0; fp_reg_type_t::num_fp_regs as usize],
                pc: 0,
            },
            mmu: mmu_t::new(),
        }
    }
    pub fn machine_load_program(&mut self, prog_path_str: &str) {
        let mut elf_file = match File::open(prog_path_str) {
            Ok(elf_file) => elf_file,
            Err(e) => panic!("Error opening file: {}", e),
        };

        self.mmu.mmu_load_elf(&mut elf_file);

        self.state.pc = self.mmu.get_entry();
    }
    pub fn machine_step(&mut self) -> exit_reason_t {
        loop {
            self.state.exit_reason = exit_reason_t::none;
            exec_block_interp(&mut self.state);
            assert_ne!(self.state.exit_reason, exit_reason_t::none);

            match self.state.exit_reason {
                exit_reason_t::indirect_branch | exit_reason_t::direct_branch => {
                    self.state.pc = self.state.reenter_pc;
                    continue;
                },
                _ => break,
            }
        }

        self.state.pc = self.state.reenter_pc;
        assert_eq!(self.state.exit_reason, exit_reason_t::ecall);
        exit_reason_t::ecall
    }
}
