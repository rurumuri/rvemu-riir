pub enum gp_reg_type_t {
    zero,
    ra,
    sp,
    gp,
    tp,
    t0,
    t1,
    t2,
    s0,
    s1,
    a0,
    a1,
    a2,
    a3,
    a4,
    a5,
    a6,
    a7,
    s2,
    s3,
    s4,
    s5,
    s6,
    s7,
    s8,
    s9,
    s10,
    s11,
    t3,
    t4,
    t5,
    t6,
    num_gp_regs,
}

#[derive(Debug)]
pub enum fp_reg_type_t {
    ft0,
    ft1,
    ft2,
    ft3,
    ft4,
    ft5,
    ft6,
    ft7,
    fs0,
    fs1,
    fa0,
    fa1,
    fa2,
    fa3,
    fa4,
    fa5,
    fa6,
    fa7,
    fs2,
    fs3,
    fs4,
    fs5,
    fs6,
    fs7,
    fs8,
    fs9,
    fs10,
    fs11,
    ft8,
    ft9,
    ft10,
    ft11,
    num_fp_regs,
}

#[derive(Copy, Clone)]
pub union fp_reg_t {
    pub v: u64,
    pub w: u32,
    pub d: f64,
    pub f: f32,
}

pub enum csr_t {
    fflags = 0x001,
    frm = 0x002,
    fcsr = 0x003,
}

impl From<u16> for csr_t {
    fn from(val: u16) -> Self {
        match val {
            0x001 => csr_t::fflags,
            0x002 => csr_t::frm,
            0x003 => csr_t::fcsr,
            _ => panic!("Invalid CSR value: {:#06x}", val),
        }
    }
}
