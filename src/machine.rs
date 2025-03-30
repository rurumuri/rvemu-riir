use crate::{
    interp::exec_block_interp,
    mmu::mmu_t,
    reg::{fp_reg_t, fp_reg_type_t, gp_reg_type_t}, syscall::{syscall_t, OLD_SYSCALL_TABLE, OLD_SYSCALL_THRESHOLD, SYSCALL_TABLE},
};
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

#[repr(C)]
pub struct state_t {
    pub exit_reason: exit_reason_t,
    pub reenter_pc: u64,
    pub gp_regs: [u64; gp_reg_type_t::num_gp_regs as usize],
    pub fp_regs: [fp_reg_t; fp_reg_type_t::num_fp_regs as usize],
    pub pc: u64,
}

#[repr(C)]
pub struct machine_t {
    pub state: state_t,
    pub mmu: mmu_t,
}

impl machine_t {
    pub fn new() -> machine_t {
        machine_t {
            state: state_t {
                exit_reason: exit_reason_t::none,
                reenter_pc: 0,
                gp_regs: [0; gp_reg_type_t::num_gp_regs as usize],
                fp_regs: [fp_reg_t { v: 0 }; fp_reg_type_t::num_fp_regs as usize],
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
                }
                _ => break,
            }
        }

        self.state.pc = self.state.reenter_pc;
        assert_eq!(self.state.exit_reason, exit_reason_t::ecall);
        exit_reason_t::ecall
    }
    pub fn machine_setup(&mut self, argc: u64, argv: &[&str]) {
        println!("sysx {:#x}", self.mmu.alloc);

        let stack_size: usize = 32 * 1024 * 1024;
        let stack: u64 = self.mmu.mmu_alloc(stack_size as i64);

        // println!(">>>>111: {}", unsafe {*(0x088800012c68 as *const u64)});
        // println!(">>>>222: {}", unsafe {*(0x088802012fd8 as *const u64)});

        // println!(">>>>>Stack: {:#x}", stack);
        self.state.gp_regs[gp_reg_type_t::sp as usize] = stack + stack_size as u64; // goto stack bottom

        // println!(">>>>{:x}", self.state.gp_regs[gp_reg_type_t::sp as usize]);

        self.state.gp_regs[gp_reg_type_t::sp as usize] -= 8; // auxv
        self.state.gp_regs[gp_reg_type_t::sp as usize] -= 8; // envp
        self.state.gp_regs[gp_reg_type_t::sp as usize] -= 8; // argv end

        let args = argc - 1;

        for i in (1..=args).rev() {
            println!("sysx {}: {}", i, argv[i as usize]);
            let len: usize = argv[i as usize].len();
            println!("sysx {:#x}", self.mmu.alloc);

            let addr: u64 = self.mmu.mmu_alloc((len + 1) as i64);
            println!("sysx {:#x}", self.mmu.alloc);

            mmu_t::mmu_write(addr, argv[i as usize].as_bytes());
            self.state.gp_regs[gp_reg_type_t::sp as usize] -= 8; // argv[i]
            mmu_t::mmu_write(
                self.state.gp_regs[gp_reg_type_t::sp as usize],
                &(addr.to_le_bytes()),
            );
            // println!(">{:x}", self.state.gp_regs[gp_reg_type_t::sp as usize]);
        }

        self.state.gp_regs[gp_reg_type_t::sp as usize] -= 8; // argc
        mmu_t::mmu_write(
            self.state.gp_regs[gp_reg_type_t::sp as usize],
            &args.to_le_bytes(),
        );
        // println!(">>>>{:x}", self.state.gp_regs[gp_reg_type_t::sp as usize]);

        // println!(">>>>{:x}", self.state.gp_regs[gp_reg_type_t::sp as usize]);
    }

    pub fn do_syscall(&mut self, syscall_num: u64) -> u64 {
        println!("syscall: {}", syscall_num);

        let f: Option<syscall_t>;
        if syscall_num as u32 <= *(SYSCALL_TABLE.keys().max().unwrap()) {
            f = Some(SYSCALL_TABLE[&(syscall_num as u32)]);
        } else if (syscall_num as u32 - OLD_SYSCALL_THRESHOLD) <= *(OLD_SYSCALL_TABLE.keys().max().unwrap()) {
            f = Some(OLD_SYSCALL_TABLE[&(syscall_num as u32 - OLD_SYSCALL_THRESHOLD as u32)]);
        } else {
            f = None;
        }

        if let Some(f) = f {
            f(self)
        } else {
            panic!("Unknown syscall: {}", syscall_num);
        }
    }

    #[inline]
    pub fn machine_get_gp_reg(&mut self, reg: gp_reg_type_t) -> u64 {
        self.state.gp_regs[reg as usize]
    }

    #[inline]
    pub fn machine_set_gp_reg(&mut self, reg: gp_reg_type_t, val: u64) {
        self.state.gp_regs[reg as usize] = val;
    }
}
