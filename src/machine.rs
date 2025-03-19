use std::{fs, path::Path};
use crate::mmu::mmu_t;

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
        let path = Path::new(prog_path_str);
        let elf = match fs::read(path) {
            Ok(elf) => elf,
            Err(e) => panic!("Error reading file: {}", e),
        };
        
        self.mmu.mmu_load_elf(&elf);

        self.state.pc = self.mmu.get_entry();
    }
    pub fn get_mmu_entry(&self) -> u64 {
        self.mmu.get_entry()
    }
}