use std::mem;

use crate::{
    decode::insn_decode,
    insn::{insn_t, insn_type_t},
    machine::{state_t, to_host_addr},
    reg::gp_reg_type_t,
};

pub fn exec_block_interp(state: &mut state_t) {
    static insn: insn_t = unsafe { mem::zeroed() };
    loop {
        let insn_data = unsafe { *(to_host_addr(state.pc) as *const u64) as u32 };
        insn_decode(&insn, insn_data);

        match insn.type_ {
            insn_type_t::insn_addi => panic!("Not implemented"),
            insn_type_t::num_insns => panic!("Not implemented"),
            _ => (),
        }

        state.gp_regs[gp_reg_type_t::zero as usize] = 0;

        if insn.cont {
            break;
        }

        state.pc += match insn.rvc {
            true => 2,
            false => 4,
        };
    }
}
