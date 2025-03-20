use std::{fs::File, io::{Read, Seek}, mem};

use libc::{PROT_READ, PROT_WRITE, PROT_EXEC};

pub const EI_NIDENT: usize = 16;
pub const ELFMAG: &str = "\x7FELF";

pub const EM_RISCV: u16 = 243;

pub const EI_CLASS: usize = 4;
pub const ELFCLASSNONE: u8 = 0;
pub const ELFCLASS32: u8 = 1;
pub const ELFCLASS64: u8 = 2;
pub const ELFCLASSNUM: u8 = 3;

pub const PT_LOAD: u32 = 1;

pub const PF_X: u32 = 0x1;
pub const PF_W: u32 = 0x2;
pub const PF_R: u32 = 0x4;

#[repr(C)]
pub struct elf64_ehdr_t {
    pub e_ident: [u8; EI_NIDENT],
    pub e_type: u16,
    pub e_machine: u16,
    pub e_version: u32,
    pub e_entry: u64,
    pub e_phoff: u64,
    pub e_shoff: u64,
    pub e_flags: u32,
    pub e_ehsize: u16,
    pub e_phentsize: u16,
    pub e_phnum: u16,
    pub e_shentsize: u16,
    pub e_shnum: u16,
    pub e_shstrndx: u16
}

#[repr(C)]
pub struct elf64_phdr_t {
    pub p_type: u32,
    pub p_flags: u32,
    pub p_offset: u64,
    pub p_vaddr: u64,
    pub p_paddr: u64,
    pub p_filesz: u64,
    pub p_memsz: u64,
    pub p_align: u64
}

pub fn flags_to_mmap_prot(flags: u32) -> i32 {
    (if flags & PF_R != 0 { PROT_READ } else { 0 }) |
    (if flags & PF_W != 0 { PROT_WRITE } else { 0 }) |
    (if flags & PF_X != 0 { PROT_EXEC } else { 0 })
}

impl elf64_phdr_t {
    pub fn load_phdr(&mut self, ehdr_t: &elf64_ehdr_t, phdr_index: i64, elf: &mut File) {
        elf.seek_relative(ehdr_t.e_phoff as i64 + phdr_index * ehdr_t.e_phentsize as i64).unwrap();
    
        let mut buf_phdr_t = [0; mem::size_of::<elf64_phdr_t>()];
        elf.read_exact(&mut buf_phdr_t).unwrap();
        *self = unsafe { mem::transmute(buf_phdr_t) };
    }
}