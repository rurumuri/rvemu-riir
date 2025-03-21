use crate::insn::*;

fn quadrant(data: u32) -> u32 {
    (data >> 0) & 0x3
}

pub fn insn_decode(insn: &insn_t, data: u32) {
    match quadrant(data) {
        0x0 => panic!("Not implemented"),
        0x1 => panic!("Not implemented"),
        0x2 => panic!("Not implemented"),
        0x3 => panic!("Not implemented"),
        _ => unreachable!(),
    }
}
