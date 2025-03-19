use crate::elfdef::*;

pub struct mmu_t {
    entry: u64
}

impl mmu_t {
    pub fn new() -> mmu_t {
        mmu_t {
            entry: 0
        }
    }
    pub fn mmu_load_elf(&mut self, elf: &Vec<u8>) {
        if elf.len() < ELFMAG.len() {
            panic!("File too short to be an ELF file");
        } else if &elf[0..ELFMAG.len()] != ELFMAG.as_bytes() {
            panic!("File is not an ELF file");
        }

        let ehdr: &elf64_ehdr_t = unsafe { &*(elf.as_ptr() as *const elf64_ehdr_t) };
        if ehdr.e_machine != EM_RISCV || ehdr.e_ident[EI_CLASS] != ELFCLASS64 {
            panic!("File is not a RISC-V ELF64 file");
        }

        self.entry = ehdr.e_entry;
    }
    pub fn get_entry(&self) -> u64 {
        self.entry
    }
}