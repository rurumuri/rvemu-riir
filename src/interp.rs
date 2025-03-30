use core::panic;
use std::mem::{self};

use crate::{
    decode::insn_decode,
    insn::{insn_t, insn_type_t},
    machine::{exit_reason_t, state_t, to_host_addr},
    reg::{csr_t, gp_reg_type_t},
};

type interp_func_t = fn(&mut state_t, &mut insn_t);

/*
    load instructions
*/
fn func_empty(state: &mut state_t, insn: &mut insn_t) {
    panic!("Unhandled instruction type: {:?}", insn.type_);
}

fn func_load_template<T: Into<i64> + Copy>(state: &mut state_t, insn: &mut insn_t) {
    // println!();
    // println!("func_load_template invoked: state@{:x} insn@{:x}", state as *const state_t as u64, insn as *const insn_t as u64);
    let addr: u64 = (state.gp_regs[insn.rs1 as usize] as i64 + insn.imm as i64) as u64; // I'm not sure if this is correct
    // println!("addr{} = {} + {}", addr, state.gp_regs[insn.rs1 as usize], insn.imm);
    // println!("{}", std::any::type_name::<T>());

    // for i in (0..32).rev() {
    //     let bit = (insn.imm >> i) & 1;
    //     print!("-{}", bit);
    // }
    // println!();

    // println!("{}", unsafe { (*(to_host_addr(addr) as *const i8))});
    // println!("{}", unsafe { (*(to_host_addr(addr) as *const i16))});
    // println!("{}", unsafe { (*(to_host_addr(addr) as *const i32))});
    // println!("{}", unsafe { (*(to_host_addr(addr) as *const i64))});
    state.gp_regs[insn.rd as usize] =
        unsafe { std::ptr::read_unaligned::<T>(to_host_addr(addr) as *const T).into() } as u64;
    // state.gp_regs[insn.rd as usize] = unsafe { (*(to_host_addr(addr) as *const T)).into() as u64 };
    // println!("loaded {} from addr={:x}({})", unsafe { (*(to_host_addr(addr) as *const T)).into() as u64 }, to_host_addr(addr), addr);
}

fn func_loadu_template<T: Into<u64> + Copy>(state: &mut state_t, insn: &mut insn_t) {
    let addr: u64 = (state.gp_regs[insn.rs1 as usize] as i64 + insn.imm as i64) as u64; // I'm not sure if this is correct
    state.gp_regs[insn.rd as usize] = unsafe { (*(to_host_addr(addr) as *const T)).into() };
}

fn func_lb(state: &mut state_t, insn: &mut insn_t) {
    func_load_template::<i8>(state, insn);
}

fn func_lh(state: &mut state_t, insn: &mut insn_t) {
    func_load_template::<i16>(state, insn);
}

fn func_lw(state: &mut state_t, insn: &mut insn_t) {
    func_load_template::<i32>(state, insn);
}

fn func_ld(state: &mut state_t, insn: &mut insn_t) {
    func_load_template::<i64>(state, insn);
}

fn func_lbu(state: &mut state_t, insn: &mut insn_t) {
    func_loadu_template::<u8>(state, insn);
}

fn func_lhu(state: &mut state_t, insn: &mut insn_t) {
    func_loadu_template::<u16>(state, insn);
}

fn func_lwu(state: &mut state_t, insn: &mut insn_t) {
    func_loadu_template::<u32>(state, insn);
}

fn func_ldu(state: &mut state_t, insn: &mut insn_t) {
    func_loadu_template::<u64>(state, insn);
}

/*
    arithmetic instructions
*/
fn func_addi(state: &mut state_t, insn: &mut insn_t) {
    // println!("func_addi: [rs1]{} imm{} [rd]{}", state.gp_regs[insn.rs1 as usize], insn.imm, state.gp_regs[insn.rd as usize]);
    state.gp_regs[insn.rd as usize] =
        state.gp_regs[insn.rs1 as usize].wrapping_add(insn.imm as u64);
    // println!("func_addi: [rs1]{} imm{} [rd]{}", state.gp_regs[insn.rs1 as usize], insn.imm, state.gp_regs[insn.rd as usize]);
}

fn func_slti(state: &mut state_t, insn: &mut insn_t) {
    state.gp_regs[insn.rd as usize] =
        ((state.gp_regs[insn.rs1 as usize] as i64) < (insn.imm as i64)) as u64;
}

fn func_sltiu(state: &mut state_t, insn: &mut insn_t) {
    state.gp_regs[insn.rd as usize] = (state.gp_regs[insn.rs1 as usize] < (insn.imm as u64)) as u64;
}

fn func_xori(state: &mut state_t, insn: &mut insn_t) {
    state.gp_regs[insn.rd as usize] = state.gp_regs[insn.rs1 as usize] ^ (insn.imm as u64);
}

fn func_ori(state: &mut state_t, insn: &mut insn_t) {
    state.gp_regs[insn.rd as usize] = state.gp_regs[insn.rs1 as usize] | (insn.imm as u64);
}

fn func_andi(state: &mut state_t, insn: &mut insn_t) {
    state.gp_regs[insn.rd as usize] = state.gp_regs[insn.rs1 as usize] & (insn.imm as u64);
}

fn func_slli(state: &mut state_t, insn: &mut insn_t) {
    state.gp_regs[insn.rd as usize] = state.gp_regs[insn.rs1 as usize] << (insn.imm & 0x3f);
}

fn func_srli(state: &mut state_t, insn: &mut insn_t) {
    state.gp_regs[insn.rd as usize] = state.gp_regs[insn.rs1 as usize] >> (insn.imm & 0x3f);
}

fn func_srai(state: &mut state_t, insn: &mut insn_t) {
    state.gp_regs[insn.rd as usize] =
        ((state.gp_regs[insn.rs1 as usize] as i64) >> (insn.imm & 0x3f)) as u64;
}

fn func_addiw(state: &mut state_t, insn: &mut insn_t) {
    state.gp_regs[insn.rd as usize] =
        ((state.gp_regs[insn.rs1 as usize] as i32).wrapping_add(insn.imm as i32)) as u64;
}

fn func_slliw(state: &mut state_t, insn: &mut insn_t) {
    state.gp_regs[insn.rd as usize] =
        ((state.gp_regs[insn.rs1 as usize] as i32) << (insn.imm & 0x1f)) as u64;
}

fn func_srliw(state: &mut state_t, insn: &mut insn_t) {
    state.gp_regs[insn.rd as usize] =
        ((state.gp_regs[insn.rs1 as usize] as u32) >> (insn.imm & 0x1f)) as u64;
}

fn func_sraiw(state: &mut state_t, insn: &mut insn_t) {
    state.gp_regs[insn.rd as usize] =
        ((state.gp_regs[insn.rs1 as usize] as i32) >> (insn.imm & 0x1f)) as u64;
}

fn func_add(state: &mut state_t, insn: &mut insn_t) {
    state.gp_regs[insn.rd as usize] =
        state.gp_regs[insn.rs1 as usize].wrapping_add(state.gp_regs[insn.rs2 as usize]);
}

fn func_sll(state: &mut state_t, insn: &mut insn_t) {
    state.gp_regs[insn.rd as usize] =
        state.gp_regs[insn.rs1 as usize] << (state.gp_regs[insn.rs2 as usize] & 0x3f);
}

fn func_slt(state: &mut state_t, insn: &mut insn_t) {
    state.gp_regs[insn.rd as usize] =
        if (state.gp_regs[insn.rs1 as usize] as i64) < (state.gp_regs[insn.rs2 as usize] as i64) {
            1
        } else {
            0
        };
}

fn func_sltu(state: &mut state_t, insn: &mut insn_t) {
    state.gp_regs[insn.rd as usize] =
        if state.gp_regs[insn.rs1 as usize] < state.gp_regs[insn.rs2 as usize] {
            1
        } else {
            0
        };
}

fn func_xor(state: &mut state_t, insn: &mut insn_t) {
    state.gp_regs[insn.rd as usize] =
        state.gp_regs[insn.rs1 as usize] ^ state.gp_regs[insn.rs2 as usize];
}

fn func_srl(state: &mut state_t, insn: &mut insn_t) {
    state.gp_regs[insn.rd as usize] =
        state.gp_regs[insn.rs1 as usize] >> (state.gp_regs[insn.rs2 as usize] & 0x3f);
}

fn func_or(state: &mut state_t, insn: &mut insn_t) {
    state.gp_regs[insn.rd as usize] =
        state.gp_regs[insn.rs1 as usize] | state.gp_regs[insn.rs2 as usize];
}

fn func_and(state: &mut state_t, insn: &mut insn_t) {
    state.gp_regs[insn.rd as usize] =
        state.gp_regs[insn.rs1 as usize] & state.gp_regs[insn.rs2 as usize];
}

fn func_mul(state: &mut state_t, insn: &mut insn_t) {
    state.gp_regs[insn.rd as usize] =
        state.gp_regs[insn.rs1 as usize].wrapping_mul(state.gp_regs[insn.rs2 as usize]);
}

fn func_mulh(state: &mut state_t, insn: &mut insn_t) {
    let rs1 = state.gp_regs[insn.rs1 as usize] as i64;
    let rs2 = state.gp_regs[insn.rs2 as usize] as i64;
    state.gp_regs[insn.rd as usize] = ((rs1 as i128 * rs2 as i128) >> 64) as u64;
}

fn func_mulhsu(state: &mut state_t, insn: &mut insn_t) {
    let rs1 = state.gp_regs[insn.rs1 as usize] as i64;
    let rs2 = state.gp_regs[insn.rs2 as usize] as u64;
    state.gp_regs[insn.rd as usize] = ((rs1 as i128 * rs2 as i128) >> 64) as u64;
}

fn func_mulhu(state: &mut state_t, insn: &mut insn_t) {
    let rs1 = state.gp_regs[insn.rs1 as usize] as u64;
    let rs2 = state.gp_regs[insn.rs2 as usize] as u64;
    state.gp_regs[insn.rd as usize] = ((rs1 as u128 * rs2 as u128) >> 64) as u64;
}

fn func_div(state: &mut state_t, insn: &mut insn_t) {
    let rs1 = state.gp_regs[insn.rs1 as usize] as i64;
    let rs2 = state.gp_regs[insn.rs2 as usize] as i64;
    state.gp_regs[insn.rd as usize] = if rs2 == 0 {
        u64::MAX
    } else if rs1 == i64::MIN && rs2 == -1 {
        rs1 as u64
    } else {
        (rs1 / rs2) as u64
    };
}

fn func_divu(state: &mut state_t, insn: &mut insn_t) {
    let rs1 = state.gp_regs[insn.rs1 as usize];
    let rs2 = state.gp_regs[insn.rs2 as usize];
    state.gp_regs[insn.rd as usize] = if rs2 == 0 { u64::MAX } else { rs1 / rs2 };
}

fn func_rem(state: &mut state_t, insn: &mut insn_t) {
    let rs1 = state.gp_regs[insn.rs1 as usize] as i64;
    let rs2 = state.gp_regs[insn.rs2 as usize] as i64;
    state.gp_regs[insn.rd as usize] = if rs2 == 0 {
        rs1 as u64
    } else if rs1 == i64::MIN && rs2 == -1 {
        0
    } else {
        (rs1 % rs2) as u64
    };
}

fn func_remu(state: &mut state_t, insn: &mut insn_t) {
    let rs1 = state.gp_regs[insn.rs1 as usize];
    let rs2 = state.gp_regs[insn.rs2 as usize];
    state.gp_regs[insn.rd as usize] = if rs2 == 0 { rs1 } else { rs1 % rs2 };
}

fn func_sub(state: &mut state_t, insn: &mut insn_t) {
    state.gp_regs[insn.rd as usize] =
        state.gp_regs[insn.rs1 as usize].wrapping_sub(state.gp_regs[insn.rs2 as usize]);
}

fn func_sra(state: &mut state_t, insn: &mut insn_t) {
    state.gp_regs[insn.rd as usize] = (state.gp_regs[insn.rs1 as usize] as i64
        >> (state.gp_regs[insn.rs2 as usize] & 0x3f)) as u64;
}

fn func_addw(state: &mut state_t, insn: &mut insn_t) {
    state.gp_regs[insn.rd as usize] = ((state.gp_regs[insn.rs1 as usize] as i32)
        .wrapping_add(state.gp_regs[insn.rs2 as usize] as i32))
        as u64;
}

fn func_sllw(state: &mut state_t, insn: &mut insn_t) {
    state.gp_regs[insn.rd as usize] = ((state.gp_regs[insn.rs1 as usize] as i32)
        << (state.gp_regs[insn.rs2 as usize] & 0x1f)) as u64;
}

fn func_srlw(state: &mut state_t, insn: &mut insn_t) {
    state.gp_regs[insn.rd as usize] = ((state.gp_regs[insn.rs1 as usize] as u32)
        >> (state.gp_regs[insn.rs2 as usize] & 0x1f)) as u64;
}

fn func_mulw(state: &mut state_t, insn: &mut insn_t) {
    state.gp_regs[insn.rd as usize] = ((state.gp_regs[insn.rs1 as usize] as i32)
        .wrapping_mul(state.gp_regs[insn.rs2 as usize] as i32))
        as u64;
}

fn func_divw(state: &mut state_t, insn: &mut insn_t) {
    let rs1 = state.gp_regs[insn.rs1 as usize] as i32;
    let rs2 = state.gp_regs[insn.rs2 as usize] as i32;
    state.gp_regs[insn.rd as usize] = if rs2 == 0 {
        u64::MAX
    } else if rs1 == i32::MIN && rs2 == -1 {
        rs1 as u64
    } else {
        (rs1 / rs2) as u64
    };
}

fn func_divuw(state: &mut state_t, insn: &mut insn_t) {
    let rs1 = state.gp_regs[insn.rs1 as usize] as u32;
    let rs2 = state.gp_regs[insn.rs2 as usize] as u32;
    state.gp_regs[insn.rd as usize] = if rs2 == 0 {
        u64::MAX
    } else {
        (rs1 / rs2) as u64
    };
}

fn func_remw(state: &mut state_t, insn: &mut insn_t) {
    let rs1 = state.gp_regs[insn.rs1 as usize] as i32;
    let rs2 = state.gp_regs[insn.rs2 as usize] as i32;
    state.gp_regs[insn.rd as usize] = if rs2 == 0 {
        rs1 as u64
    } else if rs1 == i32::MIN && rs2 == -1 {
        0
    } else {
        (rs1 % rs2) as u64
    };
}

fn func_remuw(state: &mut state_t, insn: &mut insn_t) {
    let rs1 = state.gp_regs[insn.rs1 as usize] as u32;
    let rs2 = state.gp_regs[insn.rs2 as usize] as u32;
    state.gp_regs[insn.rd as usize] = if rs2 == 0 {
        rs1 as u64
    } else {
        (rs1 % rs2) as u64
    };
}

fn func_subw(state: &mut state_t, insn: &mut insn_t) {
    state.gp_regs[insn.rd as usize] = ((state.gp_regs[insn.rs1 as usize] as i32)
        .wrapping_sub(state.gp_regs[insn.rs2 as usize] as i32))
        as u64;
}

fn func_sraw(state: &mut state_t, insn: &mut insn_t) {
    state.gp_regs[insn.rd as usize] = ((state.gp_regs[insn.rs1 as usize] as i32)
        >> (state.gp_regs[insn.rs2 as usize] & 0x1f)) as u64;
}

/*
    store instructions
*/
fn func_sb(state: &mut state_t, insn: &mut insn_t) {
    let addr: u64 = (state.gp_regs[insn.rs1 as usize] as i64 + insn.imm as i64) as u64;
    let value: u8 = (state.gp_regs[insn.rs2 as usize] & 0xFF) as u8;
    // println!("saved {} to addr={:x}({})", value, to_host_addr(addr), addr);
    unsafe {
        let ptr = to_host_addr(addr) as *mut u8;
        std::ptr::write(ptr, value);
    }
}

fn func_sh(state: &mut state_t, insn: &mut insn_t) {
    let addr: u64 = (state.gp_regs[insn.rs1 as usize] as i64 + insn.imm as i64) as u64;
    let value: u16 = (state.gp_regs[insn.rs2 as usize] & 0xFFFF) as u16;
    // println!("saved {} to addr={:x}({})", value, to_host_addr(addr), addr);
    unsafe {
        let ptr = to_host_addr(addr) as *mut u16;
        std::ptr::write(ptr, value);
    }
}

fn func_sw(state: &mut state_t, insn: &mut insn_t) {
    let addr: u64 = (state.gp_regs[insn.rs1 as usize] as i64 + insn.imm as i64) as u64;
    let value: u32 = (state.gp_regs[insn.rs2 as usize] & 0xFFFFFFFF) as u32;
    // println!("saved {} to addr={:x}({})", value, to_host_addr(addr), addr);
    unsafe {
        let ptr = to_host_addr(addr) as *mut u32;
        std::ptr::write(ptr, value);
    }
}

fn func_sd(state: &mut state_t, insn: &mut insn_t) {
    let addr: u64 = (state.gp_regs[insn.rs1 as usize] as i64 + insn.imm as i64) as u64;
    let value: u64 = state.gp_regs[insn.rs2 as usize];
    // println!("saved {} to addr={:x}({})", value, to_host_addr(addr), addr);
    unsafe {
        let ptr = to_host_addr(addr) as *mut u64;
        std::ptr::write(ptr, value);
    }
}

/*
    branch instructions
*/
fn func_beq(state: &mut state_t, insn: &mut insn_t) {
    if state.gp_regs[insn.rs1 as usize] == state.gp_regs[insn.rs2 as usize] {
        state.pc = (state.pc as i64 + insn.imm as i64) as u64;
        state.reenter_pc = state.pc;
        state.exit_reason = exit_reason_t::direct_branch;
        insn.cont = true;
    }
}

fn func_bne(state: &mut state_t, insn: &mut insn_t) {
    if state.gp_regs[insn.rs1 as usize] != state.gp_regs[insn.rs2 as usize] {
        state.pc = (state.pc as i64 + insn.imm as i64) as u64;
        state.reenter_pc = state.pc;
        state.exit_reason = exit_reason_t::direct_branch;
        insn.cont = true;
    }
}

fn func_blt(state: &mut state_t, insn: &mut insn_t) {
    if (state.gp_regs[insn.rs1 as usize] as i64) < (state.gp_regs[insn.rs2 as usize] as i64) {
        state.pc = (state.pc as i64 + insn.imm as i64) as u64;
        state.reenter_pc = state.pc;
        state.exit_reason = exit_reason_t::direct_branch;
        insn.cont = true;
    }
}

fn func_bge(state: &mut state_t, insn: &mut insn_t) {
    if (state.gp_regs[insn.rs1 as usize] as i64) >= (state.gp_regs[insn.rs2 as usize] as i64) {
        state.pc = (state.pc as i64 + insn.imm as i64) as u64;
        state.reenter_pc = state.pc;
        state.exit_reason = exit_reason_t::direct_branch;
        insn.cont = true;
    }
}

fn func_bltu(state: &mut state_t, insn: &mut insn_t) {
    if state.gp_regs[insn.rs1 as usize] < state.gp_regs[insn.rs2 as usize] {
        state.pc = (state.pc as i64 + insn.imm as i64) as u64;
        state.reenter_pc = state.pc;
        state.exit_reason = exit_reason_t::direct_branch;
        insn.cont = true;
    }
}

fn func_bgeu(state: &mut state_t, insn: &mut insn_t) {
    if state.gp_regs[insn.rs1 as usize] >= state.gp_regs[insn.rs2 as usize] {
        state.pc = (state.pc as i64 + insn.imm as i64) as u64;
        state.reenter_pc = state.pc;
        state.exit_reason = exit_reason_t::direct_branch;
        insn.cont = true;
    }
}

/*
    jump instructions
*/
fn func_jal(state: &mut state_t, insn: &mut insn_t) {
    state.gp_regs[insn.rd as usize] = state.pc
        + match insn.rvc {
            true => 2,
            false => 4,
        };
    state.pc = (state.pc as i64 + insn.imm as i64) as u64;
    state.reenter_pc = state.pc;
    state.exit_reason = exit_reason_t::direct_branch;
}

fn func_jalr(state: &mut state_t, insn: &mut insn_t) {
    state.gp_regs[insn.rd as usize] = state.pc
        + match insn.rvc {
            true => 2,
            false => 4,
        };
    let target = (state.gp_regs[insn.rs1 as usize] as i64 + insn.imm as i64) as u64;
    // println!("rs1={} [rs1]={:x} insn.imm={:x}", insn.rs1 as usize, state.gp_regs[insn.rs1 as usize], insn.imm as i64);
    // println!("func_jalr: target={}", target);
    state.reenter_pc = target & !1;
    // println!("reenter_pc={}", state.reenter_pc);
    state.exit_reason = exit_reason_t::indirect_branch;
}

/*
    csr instructions
*/
fn func_csr_handler(state: &mut state_t, insn: &mut insn_t) {
    match csr_t::from(insn.csr) {
        csr_t::fflags => {}
        csr_t::frm => {}
        csr_t::fcsr => {}
        _ => panic!("unsupported csr"),
    }
    state.gp_regs[insn.rd as usize] = 0;
}

fn func_csrrw(state: &mut state_t, insn: &mut insn_t) {
    func_csr_handler(state, insn);
}

fn func_csrrs(state: &mut state_t, insn: &mut insn_t) {
    func_csr_handler(state, insn);
}

fn func_csrrc(state: &mut state_t, insn: &mut insn_t) {
    func_csr_handler(state, insn);
}

fn func_csrrwi(state: &mut state_t, insn: &mut insn_t) {
    func_csr_handler(state, insn);
}

fn func_csrrsi(state: &mut state_t, insn: &mut insn_t) {
    func_csr_handler(state, insn);
}

fn func_csrrci(state: &mut state_t, insn: &mut insn_t) {
    func_csr_handler(state, insn);
}

/*
    other instructions
*/
fn func_lui(state: &mut state_t, insn: &mut insn_t) {
    state.gp_regs[insn.rd as usize] = insn.imm as u64;
}

fn func_auipc(state: &mut state_t, insn: &mut insn_t) {
    state.gp_regs[insn.rd as usize] = state.pc + insn.imm as u64;
}

fn func_ecall(state: &mut state_t, insn: &mut insn_t) {
    state.exit_reason = exit_reason_t::ecall;
    state.reenter_pc = state.pc + 4;
}

/*
    floating point instructions
*/
fn func_fadd_s(state: &mut state_t, insn: &mut insn_t) {
    let rs1 = unsafe { state.fp_regs[insn.rs1 as usize].f };
    let rs2 = unsafe { state.fp_regs[insn.rs2 as usize].f };
    state.fp_regs[insn.rd as usize].f = rs1 + rs2;
}

fn func_fsub_s(state: &mut state_t, insn: &mut insn_t) {
    let rs1 = unsafe { state.fp_regs[insn.rs1 as usize].f };
    let rs2 = unsafe { state.fp_regs[insn.rs2 as usize].f };
    state.fp_regs[insn.rd as usize].f = rs1 - rs2;
}

fn func_fmul_s(state: &mut state_t, insn: &mut insn_t) {
    let rs1 = unsafe { state.fp_regs[insn.rs1 as usize].f };
    let rs2 = unsafe { state.fp_regs[insn.rs2 as usize].f };
    state.fp_regs[insn.rd as usize].f = rs1 * rs2;
}

fn func_fdiv_s(state: &mut state_t, insn: &mut insn_t) {
    let rs1 = unsafe { state.fp_regs[insn.rs1 as usize].f };
    let rs2 = unsafe { state.fp_regs[insn.rs2 as usize].f };
    state.fp_regs[insn.rd as usize].f = rs1 / rs2;
}

fn func_fsqrt_s(state: &mut state_t, insn: &mut insn_t) {
    let rs1 = unsafe { state.fp_regs[insn.rs1 as usize].f };
    state.fp_regs[insn.rd as usize].f = rs1.sqrt();
}

fn func_fmin_s(state: &mut state_t, insn: &mut insn_t) {
    let rs1 = unsafe { state.fp_regs[insn.rs1 as usize].f };
    let rs2 = unsafe { state.fp_regs[insn.rs2 as usize].f };
    state.fp_regs[insn.rd as usize].f = rs1.min(rs2);
}

fn func_fmax_s(state: &mut state_t, insn: &mut insn_t) {
    let rs1 = unsafe { state.fp_regs[insn.rs1 as usize].f };
    let rs2 = unsafe { state.fp_regs[insn.rs2 as usize].f };
    state.fp_regs[insn.rd as usize].f = rs1.max(rs2);
}

fn func_flw(state: &mut state_t, insn: &mut insn_t) {
    let addr = (state.gp_regs[insn.rs1 as usize] as i64 + insn.imm as i64) as u64;
    let value = unsafe { *(to_host_addr(addr) as *const u32) };
    state.fp_regs[insn.rd as usize].v = (value as u64) | (u64::MAX << 32);
}

fn func_fld(state: &mut state_t, insn: &mut insn_t) {
    let addr = (state.gp_regs[insn.rs1 as usize] as i64 + insn.imm as i64) as u64;
    let value = unsafe { *(to_host_addr(addr) as *const u64) };
    state.fp_regs[insn.rd as usize].v = value;
}

fn func_fsw(state: &mut state_t, insn: &mut insn_t) {
    let addr = (state.gp_regs[insn.rs1 as usize] as i64 + insn.imm as i64) as u64;
    let value = unsafe { state.fp_regs[insn.rs2 as usize].v } as u32;
    unsafe { *(to_host_addr(addr) as *mut u32) = value };
}

fn func_fsd(state: &mut state_t, insn: &mut insn_t) {
    let addr = (state.gp_regs[insn.rs1 as usize] as i64 + insn.imm as i64) as u64;
    let value = unsafe { state.fp_regs[insn.rs2 as usize].v };
    unsafe { *(to_host_addr(addr) as *mut u64) = value };
}

fn func_fmadd_s(state: &mut state_t, insn: &mut insn_t) {
    let rs1 = unsafe { state.fp_regs[insn.rs1 as usize].f };
    let rs2 = unsafe { state.fp_regs[insn.rs2 as usize].f };
    let rs3 = unsafe { state.fp_regs[insn.rs3 as usize].f };
    state.fp_regs[insn.rd as usize].f = rs1 * rs2 + rs3;
}

fn func_fmsub_s(state: &mut state_t, insn: &mut insn_t) {
    let rs1 = unsafe { state.fp_regs[insn.rs1 as usize].f };
    let rs2 = unsafe { state.fp_regs[insn.rs2 as usize].f };
    let rs3 = unsafe { state.fp_regs[insn.rs3 as usize].f };
    state.fp_regs[insn.rd as usize].f = rs1 * rs2 - rs3;
}

fn func_fnmsub_s(state: &mut state_t, insn: &mut insn_t) {
    let rs1 = unsafe { state.fp_regs[insn.rs1 as usize].f };
    let rs2 = unsafe { state.fp_regs[insn.rs2 as usize].f };
    let rs3 = unsafe { state.fp_regs[insn.rs3 as usize].f };
    state.fp_regs[insn.rd as usize].f = -(rs1 * rs2) + rs3;
}

fn func_fnmadd_s(state: &mut state_t, insn: &mut insn_t) {
    let rs1 = unsafe { state.fp_regs[insn.rs1 as usize].f };
    let rs2 = unsafe { state.fp_regs[insn.rs2 as usize].f };
    let rs3 = unsafe { state.fp_regs[insn.rs3 as usize].f };
    state.fp_regs[insn.rd as usize].f = -(rs1 * rs2) - rs3;
}
/*
    floating point conversion instructions
*/
fn func_fcvt_w_s(state: &mut state_t, insn: &mut insn_t) {
    let rs1 = unsafe { state.fp_regs[insn.rs1 as usize].f };
    state.gp_regs[insn.rd as usize] = rs1 as i32 as u64; // TODO: check if this is correct
}

fn func_fcvt_wu_s(state: &mut state_t, insn: &mut insn_t) {
    let rs1 = unsafe { state.fp_regs[insn.rs1 as usize].f };
    state.gp_regs[insn.rd as usize] = rs1 as u32 as u64; // TODO: check if this is correct
}

fn func_fcvt_s_w(state: &mut state_t, insn: &mut insn_t) {
    let rs1 = state.gp_regs[insn.rs1 as usize] as i32;
    state.fp_regs[insn.rd as usize].f = rs1 as f32;
}

fn func_fcvt_s_wu(state: &mut state_t, insn: &mut insn_t) {
    let rs1 = state.gp_regs[insn.rs1 as usize] as u32;
    state.fp_regs[insn.rd as usize].f = rs1 as f32;
}

/*
    floating point comparison instructions
*/
fn func_feq_s(state: &mut state_t, insn: &mut insn_t) {
    let rs1 = unsafe { state.fp_regs[insn.rs1 as usize].f };
    let rs2 = unsafe { state.fp_regs[insn.rs2 as usize].f };
    state.gp_regs[insn.rd as usize] = (rs1 == rs2) as u64;
}

fn func_flt_s(state: &mut state_t, insn: &mut insn_t) {
    let rs1 = unsafe { state.fp_regs[insn.rs1 as usize].f };
    let rs2 = unsafe { state.fp_regs[insn.rs2 as usize].f };
    state.gp_regs[insn.rd as usize] = (rs1 < rs2) as u64;
}

fn func_fle_s(state: &mut state_t, insn: &mut insn_t) {
    let rs1 = unsafe { state.fp_regs[insn.rs1 as usize].f };
    let rs2 = unsafe { state.fp_regs[insn.rs2 as usize].f };
    state.gp_regs[insn.rd as usize] = (rs1 <= rs2) as u64;
}

/*
    floating point sign manipulation instructions
*/
fn func_fsgnj_s(state: &mut state_t, insn: &mut insn_t) {
    let rs1 = unsafe { state.fp_regs[insn.rs1 as usize].f };
    let rs2 = unsafe { state.fp_regs[insn.rs2 as usize].f };
    state.fp_regs[insn.rd as usize].f = rs1.copysign(rs2);
}

fn func_fsgnjn_s(state: &mut state_t, insn: &mut insn_t) {
    let rs1 = unsafe { state.fp_regs[insn.rs1 as usize].f };
    let rs2 = unsafe { state.fp_regs[insn.rs2 as usize].f };
    state.fp_regs[insn.rd as usize].f = rs1.copysign(-rs2);
}

fn func_fsgnjx_s(state: &mut state_t, insn: &mut insn_t) {
    let rs1 = unsafe { state.fp_regs[insn.rs1 as usize].f };
    let rs2 = unsafe { state.fp_regs[insn.rs2 as usize].f };
    state.fp_regs[insn.rd as usize].f = rs1.copysign(rs2.abs());
}

/*
    floating point classification instructions
*/
fn func_fclass_s(state: &mut state_t, insn: &mut insn_t) {
    let rs1 = unsafe { state.fp_regs[insn.rs1 as usize].f };
    state.gp_regs[insn.rd as usize] = rs1.classify() as u64;
}

/*
    floating point double precision instructions
*/
fn func_fadd_d(state: &mut state_t, insn: &mut insn_t) {
    let rs1 = unsafe { state.fp_regs[insn.rs1 as usize].d };
    let rs2 = unsafe { state.fp_regs[insn.rs2 as usize].d };
    state.fp_regs[insn.rd as usize].d = rs1 + rs2;
}

fn func_fsub_d(state: &mut state_t, insn: &mut insn_t) {
    let rs1 = unsafe { state.fp_regs[insn.rs1 as usize].d };
    let rs2 = unsafe { state.fp_regs[insn.rs2 as usize].d };
    state.fp_regs[insn.rd as usize].d = rs1 - rs2;
}

fn func_fmul_d(state: &mut state_t, insn: &mut insn_t) {
    let rs1 = unsafe { state.fp_regs[insn.rs1 as usize].d };
    let rs2 = unsafe { state.fp_regs[insn.rs2 as usize].d };
    state.fp_regs[insn.rd as usize].d = rs1 * rs2;
}

fn func_fdiv_d(state: &mut state_t, insn: &mut insn_t) {
    let rs1 = unsafe { state.fp_regs[insn.rs1 as usize].d };
    let rs2 = unsafe { state.fp_regs[insn.rs2 as usize].d };
    state.fp_regs[insn.rd as usize].d = rs1 / rs2;
}

fn func_fsqrt_d(state: &mut state_t, insn: &mut insn_t) {
    let rs1 = unsafe { state.fp_regs[insn.rs1 as usize].d };
    state.fp_regs[insn.rd as usize].d = rs1.sqrt();
}

fn func_fmin_d(state: &mut state_t, insn: &mut insn_t) {
    let rs1 = unsafe { state.fp_regs[insn.rs1 as usize].d };
    let rs2 = unsafe { state.fp_regs[insn.rs2 as usize].d };
    state.fp_regs[insn.rd as usize].d = rs1.min(rs2);
}

fn func_fmax_d(state: &mut state_t, insn: &mut insn_t) {
    let rs1 = unsafe { state.fp_regs[insn.rs1 as usize].d };
    let rs2 = unsafe { state.fp_regs[insn.rs2 as usize].d };
    state.fp_regs[insn.rd as usize].d = rs1.max(rs2);
}

fn func_fmadd_d(state: &mut state_t, insn: &mut insn_t) {
    let rs1 = unsafe { state.fp_regs[insn.rs1 as usize].d };
    let rs2 = unsafe { state.fp_regs[insn.rs2 as usize].d };
    let rs3 = unsafe { state.fp_regs[insn.rs3 as usize].d };
    state.fp_regs[insn.rd as usize].d = rs1 * rs2 + rs3;
}

fn func_fmsub_d(state: &mut state_t, insn: &mut insn_t) {
    let rs1 = unsafe { state.fp_regs[insn.rs1 as usize].d };
    let rs2 = unsafe { state.fp_regs[insn.rs2 as usize].d };
    let rs3 = unsafe { state.fp_regs[insn.rs3 as usize].d };
    state.fp_regs[insn.rd as usize].d = rs1 * rs2 - rs3;
}

fn func_fnmsub_d(state: &mut state_t, insn: &mut insn_t) {
    let rs1 = unsafe { state.fp_regs[insn.rs1 as usize].d };
    let rs2 = unsafe { state.fp_regs[insn.rs2 as usize].d };
    let rs3 = unsafe { state.fp_regs[insn.rs3 as usize].d };
    state.fp_regs[insn.rd as usize].d = -(rs1 * rs2) + rs3;
}

fn func_fnmadd_d(state: &mut state_t, insn: &mut insn_t) {
    let rs1 = unsafe { state.fp_regs[insn.rs1 as usize].d };
    let rs2 = unsafe { state.fp_regs[insn.rs2 as usize].d };
    let rs3 = unsafe { state.fp_regs[insn.rs3 as usize].d };
    state.fp_regs[insn.rd as usize].d = -(rs1 * rs2) - rs3;
}

/*
    floating point double precision comparison instructions
*/
fn func_feq_d(state: &mut state_t, insn: &mut insn_t) {
    let rs1 = unsafe { state.fp_regs[insn.rs1 as usize].d };
    let rs2 = unsafe { state.fp_regs[insn.rs2 as usize].d };
    state.gp_regs[insn.rd as usize] = (rs1 == rs2) as u64;
}

fn func_flt_d(state: &mut state_t, insn: &mut insn_t) {
    let rs1 = unsafe { state.fp_regs[insn.rs1 as usize].d };
    let rs2 = unsafe { state.fp_regs[insn.rs2 as usize].d };
    state.gp_regs[insn.rd as usize] = (rs1 < rs2) as u64;
}

fn func_fle_d(state: &mut state_t, insn: &mut insn_t) {
    let rs1 = unsafe { state.fp_regs[insn.rs1 as usize].d };
    let rs2 = unsafe { state.fp_regs[insn.rs2 as usize].d };
    state.gp_regs[insn.rd as usize] = (rs1 <= rs2) as u64;
}

/*
    floating point double precision sign manipulation instructions
*/
fn func_fsgnj_d(state: &mut state_t, insn: &mut insn_t) {
    let rs1 = unsafe { state.fp_regs[insn.rs1 as usize].d };
    let rs2 = unsafe { state.fp_regs[insn.rs2 as usize].d };
    state.fp_regs[insn.rd as usize].d = rs1.copysign(rs2);
}

fn func_fsgnjn_d(state: &mut state_t, insn: &mut insn_t) {
    let rs1 = unsafe { state.fp_regs[insn.rs1 as usize].d };
    let rs2 = unsafe { state.fp_regs[insn.rs2 as usize].d };
    state.fp_regs[insn.rd as usize].d = rs1.copysign(-rs2);
}

fn func_fsgnjx_d(state: &mut state_t, insn: &mut insn_t) {
    let rs1 = unsafe { state.fp_regs[insn.rs1 as usize].d };
    let rs2 = unsafe { state.fp_regs[insn.rs2 as usize].d };
    state.fp_regs[insn.rd as usize].d = rs1.copysign(rs2.abs());
}

/*
    floating point double precision classification instructions
*/
fn func_fclass_d(state: &mut state_t, insn: &mut insn_t) {
    let rs1 = unsafe { state.fp_regs[insn.rs1 as usize].d };
    state.gp_regs[insn.rd as usize] = rs1.classify() as u64;
}

/*
    floating point double precision conversion instructions
*/
fn func_fcvt_d_s(state: &mut state_t, insn: &mut insn_t) {
    let rs1 = unsafe { state.fp_regs[insn.rs1 as usize].f };
    state.fp_regs[insn.rd as usize].d = rs1 as f64;
}

fn func_fcvt_s_d(state: &mut state_t, insn: &mut insn_t) {
    let rs1 = unsafe { state.fp_regs[insn.rs1 as usize].d };
    state.fp_regs[insn.rd as usize].f = rs1 as f32;
}

fn func_fcvt_w_d(state: &mut state_t, insn: &mut insn_t) {
    let rs1 = unsafe { state.fp_regs[insn.rs1 as usize].d };
    state.gp_regs[insn.rd as usize] = rs1 as i32 as u64; // TODO: check if this is correct
}

fn func_fcvt_wu_d(state: &mut state_t, insn: &mut insn_t) {
    let rs1 = unsafe { state.fp_regs[insn.rs1 as usize].d };
    state.gp_regs[insn.rd as usize] = rs1 as u32 as u64; // TODO: check if this is correct
}

fn func_fcvt_d_w(state: &mut state_t, insn: &mut insn_t) {
    let rs1 = state.gp_regs[insn.rs1 as usize] as i32;
    state.fp_regs[insn.rd as usize].d = rs1 as f64;
}

fn func_fcvt_d_wu(state: &mut state_t, insn: &mut insn_t) {
    let rs1 = state.gp_regs[insn.rs1 as usize] as u32;
    state.fp_regs[insn.rd as usize].d = rs1 as f64;
}

fn func_fcvt_l_d(state: &mut state_t, insn: &mut insn_t) {
    let rs1 = unsafe { state.fp_regs[insn.rs1 as usize].d };
    state.gp_regs[insn.rd as usize] = rs1 as i64 as u64; // TODO: check if this is correct
}

fn func_fcvt_lu_d(state: &mut state_t, insn: &mut insn_t) {
    let rs1 = unsafe { state.fp_regs[insn.rs1 as usize].d };
    state.gp_regs[insn.rd as usize] = rs1 as u64; // TODO: check if this is correct
}

fn func_fcvt_d_l(state: &mut state_t, insn: &mut insn_t) {
    let rs1 = state.gp_regs[insn.rs1 as usize] as i64;
    state.fp_regs[insn.rd as usize].d = rs1 as f64;
}

fn func_fcvt_d_lu(state: &mut state_t, insn: &mut insn_t) {
    let rs1 = state.gp_regs[insn.rs1 as usize] as u64;
    state.fp_regs[insn.rd as usize].d = rs1 as f64;
}

fn func_fmv_x_d(state: &mut state_t, insn: &mut insn_t) {
    state.gp_regs[insn.rd as usize] = unsafe { state.fp_regs[insn.rs1 as usize].v };
}

fn func_fmv_d_x(state: &mut state_t, insn: &mut insn_t) {
    state.fp_regs[insn.rd as usize].v = state.gp_regs[insn.rs1 as usize];
}

fn func_fmv_x_w(state: &mut state_t, insn: &mut insn_t) {
    state.gp_regs[insn.rd as usize] = unsafe { state.fp_regs[insn.rs1 as usize].w } as u64;
}

fn func_fmv_w_x(state: &mut state_t, insn: &mut insn_t) {
    state.fp_regs[insn.rd as usize].w = state.gp_regs[insn.rs1 as usize] as u32;
}

fn func_fcvt_l_s(state: &mut state_t, insn: &mut insn_t) {
    let rs1 = unsafe { state.fp_regs[insn.rs1 as usize].f };
    state.gp_regs[insn.rd as usize] = rs1.round() as i64 as u64; // TODO: check if this is correct
}

fn func_fcvt_lu_s(state: &mut state_t, insn: &mut insn_t) {
    let rs1 = unsafe { state.fp_regs[insn.rs1 as usize].f };
    state.gp_regs[insn.rd as usize] = rs1.round() as u64; // TODO: check if this is correct
}

fn func_fcvt_s_l(state: &mut state_t, insn: &mut insn_t) {
    let rs1 = state.gp_regs[insn.rs1 as usize] as i64;
    state.fp_regs[insn.rd as usize].f = rs1 as f32;
}

fn func_fcvt_s_lu(state: &mut state_t, insn: &mut insn_t) {
    let rs1 = state.gp_regs[insn.rs1 as usize] as u64;
    state.fp_regs[insn.rd as usize].f = rs1 as f32;
}

static interp_funcs: [interp_func_t; insn_type_t::num_insns as usize] = [
    func_lb,
    func_lh,
    func_lw,
    func_ld,
    func_lbu,
    func_lhu,
    func_lwu,
    func_empty, // fence
    func_empty, // fence_i
    func_addi,
    func_slli,
    func_slti,
    func_sltiu,
    func_xori,
    func_srli,
    func_srai,
    func_ori,
    func_andi,
    func_auipc,
    func_addiw,
    func_slliw,
    func_srliw,
    func_sraiw,
    func_sb,
    func_sh,
    func_sw,
    func_sd,
    func_add,
    func_sll,
    func_slt,
    func_sltu,
    func_xor,
    func_srl,
    func_or,
    func_and,
    func_mul,
    func_mulh,
    func_mulhsu,
    func_mulhu,
    func_div,
    func_divu,
    func_rem,
    func_remu,
    func_sub,
    func_sra,
    func_lui,
    func_addw,
    func_sllw,
    func_srlw,
    func_mulw,
    func_divw,
    func_divuw,
    func_remw,
    func_remuw,
    func_subw,
    func_sraw,
    func_beq,
    func_bne,
    func_blt,
    func_bge,
    func_bltu,
    func_bgeu,
    func_jalr,
    func_jal,
    func_ecall,
    func_csrrw,
    func_csrrs,
    func_csrrc,
    func_csrrwi,
    func_csrrsi,
    func_csrrci,
    func_flw,
    func_fsw,
    func_fmadd_s,
    func_fmsub_s,
    func_fnmsub_s,
    func_fnmadd_s,
    func_fadd_s,
    func_fsub_s,
    func_fmul_s,
    func_fdiv_s,
    func_fsqrt_s,
    func_fsgnj_s,
    func_fsgnjn_s,
    func_fsgnjx_s,
    func_fmin_s,
    func_fmax_s,
    func_fcvt_w_s,
    func_fcvt_wu_s,
    func_fmv_x_w,
    func_feq_s,
    func_flt_s,
    func_fle_s,
    func_fclass_s,
    func_fcvt_s_w,
    func_fcvt_s_wu,
    func_fmv_w_x,
    func_fcvt_l_s,
    func_fcvt_lu_s,
    func_fcvt_s_l,
    func_fcvt_s_lu,
    func_fld,
    func_fsd,
    func_fmadd_d,
    func_fmsub_d,
    func_fnmsub_d,
    func_fnmadd_d,
    func_fadd_d,
    func_fsub_d,
    func_fmul_d,
    func_fdiv_d,
    func_fsqrt_d,
    func_fsgnj_d,
    func_fsgnjn_d,
    func_fsgnjx_d,
    func_fmin_d,
    func_fmax_d,
    func_fcvt_s_d,
    func_fcvt_d_s,
    func_feq_d,
    func_flt_d,
    func_fle_d,
    func_fclass_d,
    func_fcvt_w_d,
    func_fcvt_wu_d,
    func_fcvt_d_w,
    func_fcvt_d_wu,
    func_fcvt_l_d,
    func_fcvt_lu_d,
    func_fmv_x_d,
    func_fcvt_d_l,
    func_fcvt_d_lu,
    func_fmv_d_x,
];

pub fn exec_block_interp(state: &mut state_t) {
    loop {
        // println!("pc: {:#x}", state.pc);
        let mut insn: insn_t = unsafe { mem::zeroed() };
        let insn_data = unsafe { std::ptr::read_unaligned(to_host_addr(state.pc) as *const u32) };
        insn_decode(&mut insn, insn_data);

        // println!(">>>0x08880201bbc0: {}", unsafe {*(0x08880201bbc0 as *const u64)});
        // println!(">>>222: {}", unsafe {*(0x088802012fd8 as *const u64)});

        interp_funcs[insn.type_ as usize](state, &mut insn);

        state.gp_regs[gp_reg_type_t::zero as usize] = 0;

        // print!("state.gp_regs: ");
        // for i in 0..32 {
        //     print!(" x{}: {}", i, state.gp_regs[i]);
        // }
        // println!();
        // println!("reenter_pc: {:#x}", state.reenter_pc);
        // println!();

        if insn.cont {
            break;
        }

        state.pc += match insn.rvc {
            true => 2,
            false => 4,
        };
    }
}
