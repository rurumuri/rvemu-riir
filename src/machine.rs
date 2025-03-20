use std::fs::File;
use crate::mmu::mmu_t;

const GUEST_MEMORY_OFFSET: u64 = 0x0888_0000_0000;

pub fn to_host_addr(addr: u64) -> u64 {
    addr + GUEST_MEMORY_OFFSET
}

pub fn to_guest_addr(addr: u64) -> u64 {
    addr - GUEST_MEMORY_OFFSET
}

struct state_t {
    gp_regs: [u64; 32],
    pc: u64,
}

pub struct machine_t {
    state: state_t,
    mmu: mmu_t,
}

impl machine_t {
    pub fn new() -> machine_t {
        machine_t {
            state: state_t {
                gp_regs: [0; 32],
                pc: 0,
            },
            mmu: mmu_t::new(),
        }
    }
    pub fn machine_load_program(&mut self, prog_path_str: &str){
        let mut elf_file = match File::open(prog_path_str) {
            Ok(elf_file) => elf_file,
            Err(e) => panic!("Error opening file: {}", e),
        };

        self.mmu.mmu_load_elf(&mut elf_file);

        self.state.pc = self.mmu.get_entry();
    }
    pub fn get_mmu_entry(&self) -> u64 {
        self.mmu.get_entry()
    }
}