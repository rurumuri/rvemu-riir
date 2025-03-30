use std::{collections::HashMap, sync::LazyLock};

use libc::{O_APPEND, O_CREAT, O_EXCL, O_RDONLY, O_RDWR, O_TRUNC, O_WRONLY};

use crate::{machine::{machine_t, to_host_addr}, reg::gp_reg_type_t};

// Copied from https://github.com/riscv-software-src/riscv-pk

pub const SYS_EXIT: u32 = 93;
pub const SYS_EXIT_GROUP: u32 = 94;
pub const SYS_GETPID: u32 = 172;
pub const SYS_KILL: u32 = 129;
pub const SYS_TGKILL: u32 = 131;
pub const SYS_READ: u32 = 63;
pub const SYS_WRITE: u32 = 64;
pub const SYS_OPENAT: u32 = 56;
pub const SYS_CLOSE: u32 = 57;
pub const SYS_LSEEK: u32 = 62;
pub const SYS_BRK: u32 = 214;
pub const SYS_LINKAT: u32 = 37;
pub const SYS_UNLINKAT: u32 = 35;
pub const SYS_MKDIRAT: u32 = 34;
pub const SYS_RENAMEAT: u32 = 38;
pub const SYS_CHDIR: u32 = 49;
pub const SYS_GETCWD: u32 = 17;
pub const SYS_FSTAT: u32 = 80;
pub const SYS_FSTATAT: u32 = 79;
pub const SYS_FACCESSAT: u32 = 48;
pub const SYS_PREAD: u32 = 67;
pub const SYS_PWRITE: u32 = 68;
pub const SYS_UNAME: u32 = 160;
pub const SYS_GETUID: u32 = 174;
pub const SYS_GETEUID: u32 = 175;
pub const SYS_GETGID: u32 = 176;
pub const SYS_GETEGID: u32 = 177;
pub const SYS_GETTID: u32 = 178;
pub const SYS_SYSINFO: u32 = 179;
pub const SYS_MMAP: u32 = 222;
pub const SYS_MUNMAP: u32 = 215;
pub const SYS_MREMAP: u32 = 216;
pub const SYS_MPROTECT: u32 = 226;
pub const SYS_PRLIMIT64: u32 = 261;
pub const SYS_GETMAINVARS: u32 = 2011;
pub const SYS_RT_SIGACTION: u32 = 134;
pub const SYS_WRITEV: u32 = 66;
pub const SYS_GETTIMEOFDAY: u32 = 169;
pub const SYS_TIMES: u32 = 153;
pub const SYS_FCNTL: u32 = 25;
pub const SYS_FTRUNCATE: u32 = 46;
pub const SYS_GETDENTS: u32 = 61;
pub const SYS_DUP: u32 = 23;
pub const SYS_DUP3: u32 = 24;
pub const SYS_READLINKAT: u32 = 78;
pub const SYS_RT_SIGPROCMASK: u32 = 135;
pub const SYS_IOCTL: u32 = 29;
pub const SYS_GETRLIMIT: u32 = 163;
pub const SYS_SETRLIMIT: u32 = 164;
pub const SYS_GETRUSAGE: u32 = 165;
pub const SYS_CLOCK_GETTIME: u32 = 113;
pub const SYS_SET_TID_ADDRESS: u32 = 96;
pub const SYS_SET_ROBUST_LIST: u32 = 99;
pub const SYS_MADVISE: u32 = 233;
pub const SYS_STATX: u32 = 291;

pub const OLD_SYSCALL_THRESHOLD: u32 = 1024;
pub const SYS_OPEN: u32 = 1024;
pub const SYS_LINK: u32 = 1025;
pub const SYS_UNLINK: u32 = 1026;
pub const SYS_MKDIR: u32 = 1030;
pub const SYS_ACCESS: u32 = 1033;
pub const SYS_STAT: u32 = 1038;
pub const SYS_LSTAT: u32 = 1039;
pub const SYS_TIME: u32 = 1062;

pub type syscall_t = fn(&mut machine_t) -> u64;

pub static SYSCALL_TABLE: LazyLock<HashMap<u32, syscall_t>> = LazyLock::new(|| {
    let mut table: HashMap<u32, syscall_t> = HashMap::new();
    
    table.insert(SYS_EXIT, sys_exit);
    table.insert(SYS_EXIT_GROUP, sys_exit);
    table.insert(SYS_READ, sys_read);
    table.insert(SYS_PREAD, sys_unimplemented);
    table.insert(SYS_WRITE, sys_write);
    table.insert(SYS_OPENAT, sys_openat);
    table.insert(SYS_CLOSE, sys_close);
    table.insert(SYS_FSTAT, sys_fstat);
    table.insert(SYS_STATX, sys_unimplemented);
    table.insert(SYS_LSEEK, sys_lseek);
    table.insert(SYS_FSTATAT, sys_unimplemented);
    table.insert(SYS_LINKAT, sys_unimplemented);
    table.insert(SYS_UNLINKAT, sys_unimplemented);
    table.insert(SYS_MKDIRAT, sys_unimplemented);
    table.insert(SYS_RENAMEAT, sys_unimplemented);
    table.insert(SYS_GETCWD, sys_unimplemented);
    table.insert(SYS_BRK, sys_brk);
    table.insert(SYS_UNAME, sys_unimplemented);
    table.insert(SYS_GETPID, sys_unimplemented);
    table.insert(SYS_GETUID, sys_unimplemented);
    table.insert(SYS_GETEUID, sys_unimplemented);
    table.insert(SYS_GETGID, sys_unimplemented);
    table.insert(SYS_GETEGID, sys_unimplemented);
    table.insert(SYS_GETTID, sys_unimplemented);
    table.insert(SYS_TGKILL, sys_unimplemented);
    table.insert(SYS_MMAP, sys_unimplemented);
    table.insert(SYS_MUNMAP, sys_unimplemented);
    table.insert(SYS_MREMAP, sys_unimplemented);
    table.insert(SYS_MPROTECT, sys_unimplemented);
    table.insert(SYS_RT_SIGACTION, sys_unimplemented);
    table.insert(SYS_GETTIMEOFDAY, sys_gettimeofday);
    table.insert(SYS_TIMES, sys_unimplemented);
    table.insert(SYS_WRITEV, sys_unimplemented);
    table.insert(SYS_FACCESSAT, sys_unimplemented);
    table.insert(SYS_FCNTL, sys_unimplemented);
    table.insert(SYS_FTRUNCATE, sys_unimplemented);
    table.insert(SYS_GETDENTS, sys_unimplemented);
    table.insert(SYS_DUP, sys_unimplemented);
    table.insert(SYS_DUP3, sys_unimplemented);
    table.insert(SYS_RT_SIGPROCMASK, sys_unimplemented);
    table.insert(SYS_CLOCK_GETTIME, sys_unimplemented);
    table.insert(SYS_CHDIR, sys_unimplemented);

    table
});


pub static OLD_SYSCALL_TABLE: LazyLock<HashMap<u32, syscall_t>> = LazyLock::new(|| {
    let mut table: HashMap<u32, syscall_t> = HashMap::new();
    table.insert(SYS_OPEN - OLD_SYSCALL_THRESHOLD, sys_open);
    table.insert(SYS_LINK - OLD_SYSCALL_THRESHOLD, sys_unimplemented);
    table.insert(SYS_UNLINK - OLD_SYSCALL_THRESHOLD, sys_unimplemented);
    table.insert(SYS_MKDIR - OLD_SYSCALL_THRESHOLD, sys_unimplemented);
    table.insert(SYS_ACCESS - OLD_SYSCALL_THRESHOLD, sys_unimplemented);
    table.insert(SYS_STAT - OLD_SYSCALL_THRESHOLD, sys_unimplemented);
    table.insert(SYS_LSTAT - OLD_SYSCALL_THRESHOLD, sys_unimplemented);
    table.insert(SYS_TIME - OLD_SYSCALL_THRESHOLD, sys_unimplemented);
    table
});

fn sys_unimplemented(m: &mut machine_t) -> u64 {
    panic!("Unimplemented syscall: {}", m.state.gp_regs[gp_reg_type_t::a7 as usize]);
}

fn sys_exit(m: &mut machine_t) -> u64 {
    let code: u64 = m.state.gp_regs[gp_reg_type_t::a0 as usize];

    unsafe { libc::exit(
        code as libc::c_int
    ) };
}

fn sys_close(m: &mut machine_t) -> u64 {
    let fd: u64 = m.state.gp_regs[gp_reg_type_t::a0 as usize];

    println!("sys_close, fd: {}", fd);

    if fd > 2 {
        return unsafe { libc::close(
            fd as libc::c_int
        ) as u64 }
    };
    return 0;
}

fn sys_write(m: &mut machine_t) -> u64 {
    let fd: u64 = m.state.gp_regs[gp_reg_type_t::a0 as usize];
    let ptr: u64 = m.state.gp_regs[gp_reg_type_t::a1 as usize];
    let len: u64 = m.state.gp_regs[gp_reg_type_t::a2 as usize];
    println!("sys_write, fd: {}, ptr: {:#x}, len: {}", fd, to_host_addr(ptr), len);

    return unsafe { libc::write(
        fd as libc::c_int,
        to_host_addr(ptr) as *const libc::c_void,
        len as libc::size_t
    ) } as u64;
}

fn sys_fstat(m: &mut machine_t) -> u64 {
    let fd: u64 = m.state.gp_regs[gp_reg_type_t::a0 as usize];
    let addr: u64 = m.state.gp_regs[gp_reg_type_t::a1 as usize];
    println!("sys_fstat, fd: {}, addr: {}", fd, addr);

    let ret = unsafe { libc::fstat(
        fd as libc::c_int,
        to_host_addr(addr) as *mut libc::stat
    ) };

    let stat_info: libc::stat = unsafe { *(to_host_addr(addr) as *mut libc::stat).as_mut().unwrap() };
    println!("sys_fstat Device: {}", stat_info.st_dev);
    println!("sys_fstat Inode: {}", stat_info.st_ino);
    println!("sys_fstat Mode: {}", stat_info.st_mode);
    println!("sys_fstat Number of hard links: {}", stat_info.st_nlink);
    println!("sys_fstat Owner's user ID: {}", stat_info.st_uid);
    println!("sys_fstat Owner's group ID: {}", stat_info.st_gid);
    println!("sys_fstat Total size, in bytes: {}", stat_info.st_size);
    println!("sys_fstat Last access time: {}", stat_info.st_atime);
    println!("sys_fstat Last modification time: {}", stat_info.st_mtime);
    println!("sys_fstat Last status change time: {}", stat_info.st_ctime);

    ret as u64

    // return unsafe { libc::fstat(
    //     fd as libc::c_int,
    //     to_host_addr(addr) as *mut libc::stat
    // ) } as u64;
}

fn sys_gettimeofday(m: &mut machine_t) -> u64 {
    let tv_addr: u64 = m.state.gp_regs[gp_reg_type_t::a0 as usize];
    let tz_addr: u64 = m.state.gp_regs[gp_reg_type_t::a1 as usize];
    let tv = unsafe { &mut *(to_host_addr(tv_addr) as *mut libc::timeval) };
    let tz: *mut libc::timezone = if tz_addr != 0 {
        unsafe { &mut *(to_host_addr(tz_addr) as *mut libc::timezone) }
    } else {
        std::ptr::null_mut()
    };

    return unsafe { libc::gettimeofday(
        tv,
        tz
    ) } as u64;
}

fn sys_brk(m: &mut machine_t) -> u64 {
    let mut addr: u64 = m.state.gp_regs[gp_reg_type_t::a0 as usize];
    if addr == 0 {
        addr = m.mmu.alloc;
    }
    assert!(addr >= m.mmu.base);
    let incr: i64 = addr as i64 - m.mmu.alloc as i64;

    println!("sys_brk, addr: {:#x}, incr: {} = {} - {}", to_host_addr(addr), incr, addr, m.mmu.alloc );

    m.mmu.mmu_alloc(incr);    
    return addr;
}


// the O_* macros is OS dependent.
// here is a workaround to convert newlib flags to the host.
pub const NEWLIB_O_RDONLY: i32 = 0x0;
pub const NEWLIB_O_WRONLY: i32 = 0x1;
pub const NEWLIB_O_RDWR: i32 = 0x2;
pub const NEWLIB_O_APPEND: i32 = 0x8;
pub const NEWLIB_O_CREAT: i32 = 0x200;
pub const NEWLIB_O_TRUNC: i32 = 0x400;
pub const NEWLIB_O_EXCL: i32 = 0x800;

fn convert_flags(flags: i32) -> i32 {
    let mut hostflags: i32 = 0;

    if flags & NEWLIB_O_RDONLY != 0 {
        hostflags |= O_RDONLY;
    }
    if flags & NEWLIB_O_WRONLY != 0 {
        hostflags |= O_WRONLY;
    }
    if flags & NEWLIB_O_RDWR != 0 {
        hostflags |= O_RDWR;
    }
    if flags & NEWLIB_O_APPEND != 0 {
        hostflags |= O_APPEND;
    }
    if flags & NEWLIB_O_CREAT != 0 {
        hostflags |= O_CREAT;
    }
    if flags & NEWLIB_O_TRUNC != 0 {
        hostflags |= O_TRUNC;
    }
    if flags & NEWLIB_O_EXCL != 0 {
        hostflags |= O_EXCL;
    }

    hostflags
}

fn sys_openat(m: &mut machine_t) -> u64 {
    let dirfd: u64 = m.state.gp_regs[gp_reg_type_t::a0 as usize];
    let nameptr: u64 = m.state.gp_regs[gp_reg_type_t::a1 as usize];
    let flags: u64 = m.state.gp_regs[gp_reg_type_t::a2 as usize];
    let mode: u64 = m.state.gp_regs[gp_reg_type_t::a3 as usize];

    return unsafe { libc::openat(
        dirfd as libc::c_int,
        to_host_addr(nameptr) as *const libc::c_char,
        convert_flags(flags as i32) as libc::c_int,
        mode as libc::c_int,
    ) } as u64;
}

fn sys_open(m: &mut machine_t) -> u64 {
    let nameptr: u64 = m.state.gp_regs[gp_reg_type_t::a0 as usize];
    let flags: u64 = m.state.gp_regs[gp_reg_type_t::a1 as usize];
    let mode: u64 = m.state.gp_regs[gp_reg_type_t::a2 as usize];

    return unsafe { libc::open(
        to_host_addr(nameptr) as *const libc::c_char,
        convert_flags(flags as i32) as libc::c_int,
        mode as libc::c_int,
    ) } as u64;
}

fn sys_lseek(m: &mut machine_t) -> u64 {
    let fd: u64 = m.state.gp_regs[gp_reg_type_t::a0 as usize];
    let offset: u64 = m.state.gp_regs[gp_reg_type_t::a1 as usize];
    let whence: u64 = m.state.gp_regs[gp_reg_type_t::a2 as usize];

    return unsafe { libc::lseek(
        fd as libc::c_int,
        offset as libc::off_t,
        whence as libc::c_int
    ) } as u64;
}

fn sys_read(m: &mut machine_t) -> u64 {
    let fd: u64 = m.state.gp_regs[gp_reg_type_t::a0 as usize];
    let bufptr: u64 = m.state.gp_regs[gp_reg_type_t::a1 as usize];
    let count: u64 = m.state.gp_regs[gp_reg_type_t::a2 as usize];

    return unsafe { libc::read(
        fd as libc::c_int,
        to_host_addr(bufptr) as *mut libc::c_void,
        count as libc::size_t
    ) } as u64;
}