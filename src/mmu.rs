use crate::{
    machine::{to_guest_addr, to_host_addr},
    utils::{round_down, round_up},
};
use std::{
    cmp,
    fs::File,
    io::{Read, Seek},
    mem,
    os::fd::AsRawFd,
};

use crate::elf::*;

pub struct mmu_t {
    pub entry: u64,
    pub host_alloc: u64,
    pub alloc: u64,
    pub base: u64,
}

impl mmu_t {
    pub fn new() -> mmu_t {
        mmu_t {
            entry: 0,
            host_alloc: 0,
            alloc: 0,
            base: 0,
        }
    }
    pub fn mmu_load_elf(&mut self, elf: &mut File) {
        if elf.metadata().unwrap().len() < ELFMAG.len() as u64 {
            panic!("File too short to be an ELF file");
        }

        let mut buf_elfmag = [0; ELFMAG.len()];
        elf.read_exact(&mut buf_elfmag).unwrap();
        if buf_elfmag != ELFMAG.as_bytes() {
            panic!("File is not an ELF file");
        }

        let mut buf_ehdr_t = [0; mem::size_of::<elf64_ehdr_t>()];
        elf.seek(std::io::SeekFrom::Start(0)).unwrap();
        elf.read_exact(&mut buf_ehdr_t).unwrap();
        let ehdr: &elf64_ehdr_t = unsafe { &mem::transmute(buf_ehdr_t) };
        if ehdr.e_machine != EM_RISCV || ehdr.e_ident[EI_CLASS] != ELFCLASS64 {
            panic!("File is not a RISC-V ELF64 file");
        }

        self.entry = ehdr.e_entry;

        let mut phdr_t: elf64_phdr_t = unsafe { mem::zeroed() };
        for i in 0..ehdr.e_phnum {
            elf.seek(std::io::SeekFrom::Start(0)).unwrap();
            phdr_t.load_phdr(ehdr, i as i64, elf);
            if phdr_t.p_type == PT_LOAD {
                self.mmu_load_segment(&phdr_t, elf);
            }
        }
    }
    pub fn get_entry(&self) -> u64 {
        self.entry
    }

    fn mmu_load_segment(&mut self, phdr_t: &elf64_phdr_t, elf: &File) {
        let page_size: usize = page_size::get();
        let offset = phdr_t.p_offset;
        let vaddr = to_host_addr(phdr_t.p_vaddr);
        let aligned_vaddr = round_down(vaddr, page_size as u64);
        let filesz = phdr_t.p_filesz + (vaddr - aligned_vaddr);
        let memsz = phdr_t.p_memsz + (vaddr - aligned_vaddr);
        let prot = flags_to_mmap_prot(phdr_t.p_flags);

        let addr: u64 = unsafe {
            libc::mmap(
                aligned_vaddr as *mut libc::c_void,
                memsz as usize,
                prot,
                libc::MAP_PRIVATE | libc::MAP_FIXED,
                elf.as_raw_fd() as i32,
                round_down(offset, page_size as u64) as libc::off_t,
            ) as u64
        };
        assert_eq!(addr, aligned_vaddr);

        let remaining_bss = round_up(memsz, page_size as u64);
        if remaining_bss as isize > 0 {
            let addr: usize = unsafe {
                libc::mmap(
                    (aligned_vaddr + round_up(filesz, page_size as u64)) as *mut libc::c_void,
                    remaining_bss as usize,
                    prot,
                    libc::MAP_PRIVATE | libc::MAP_ANONYMOUS | libc::MAP_FIXED,
                    -1,
                    0,
                ) as usize
            };
            assert_eq!(
                addr,
                aligned_vaddr as usize + round_up(filesz, page_size as u64) as usize
            );
        }

        self.host_alloc = cmp::max(
            self.host_alloc,
            aligned_vaddr + round_up(memsz, page_size as u64) as u64,
        );
        self.alloc = to_guest_addr(self.host_alloc);
        self.base = to_guest_addr(self.host_alloc);
    }

    pub fn mmu_alloc(&mut self, size: i64) -> u64 {
        let page_size: usize = page_size::get();
        let base: u64 = self.alloc;
        assert!(base >= self.base);

        self.alloc = self.alloc.wrapping_add_signed(size);
        assert!(self.alloc >= self.base);

        if size > 0 && self.alloc > to_guest_addr(self.host_alloc) {
            if unsafe {
                libc::mmap(
                    self.host_alloc as *mut libc::c_void,
                    round_up(size.abs() as u64, page_size as u64) as usize,
                    libc::PROT_READ | libc::PROT_WRITE,
                    libc::MAP_PRIVATE | libc::MAP_ANONYMOUS | libc::MAP_FIXED,
                    -1,
                    0,
                )
            } == libc::MAP_FAILED
            {
                panic!("mmap failed");
            }

            self.host_alloc += round_up(size.abs() as u64, page_size as u64);
        } else if size < 0
            && round_up(self.alloc, page_size as u64) < to_guest_addr(self.host_alloc)
        {
            let len: u64 = to_guest_addr(self.host_alloc) - round_up(self.alloc, page_size as u64);
            if unsafe { libc::munmap(self.host_alloc as *mut libc::c_void, len as usize) } == -1 {
                panic!("munmap failed");
            }
            self.host_alloc -= len;
        }
        assert_eq!(self.alloc, base + size.abs() as u64);
        base
    }

    #[inline]
    pub fn mmu_write(addr: u64, data: &[u8]) {
        unsafe {
            let res = libc::memcpy(
                to_host_addr(addr) as *mut libc::c_void,
                data.as_ptr() as *const libc::c_void,
                data.len(),
            );
        };
    }
}
