pub struct insn_t {
    pub rd: u8,
    pub rs1: u8,
    pub rs2: u8,
    pub rs3: u8,
    pub imm: i32, // Immediate value
    pub csr: u16, // Control and status register
    pub type_: insn_type_t,
    pub rvc: bool,  // If is a compressed instruction
    pub cont: bool, // If is a continuation
}

#[derive(Copy, Clone, Debug)]
pub enum insn_type_t {
    insn_lb,
    insn_lh,
    insn_lw,
    insn_ld,
    insn_lbu,
    insn_lhu,
    insn_lwu,
    insn_fence,
    insn_fence_i,
    insn_addi,
    insn_slli,
    insn_slti,
    insn_sltiu,
    insn_xori,
    insn_srli,
    insn_srai,
    insn_ori,
    insn_andi,
    insn_auipc,
    insn_addiw,
    insn_slliw,
    insn_srliw,
    insn_sraiw,
    insn_sb,
    insn_sh,
    insn_sw,
    insn_sd,
    insn_add,
    insn_sll,
    insn_slt,
    insn_sltu,
    insn_xor,
    insn_srl,
    insn_or,
    insn_and,
    insn_mul,
    insn_mulh,
    insn_mulhsu,
    insn_mulhu,
    insn_div,
    insn_divu,
    insn_rem,
    insn_remu,
    insn_sub,
    insn_sra,
    insn_lui,
    insn_addw,
    insn_sllw,
    insn_srlw,
    insn_mulw,
    insn_divw,
    insn_divuw,
    insn_remw,
    insn_remuw,
    insn_subw,
    insn_sraw,
    insn_beq,
    insn_bne,
    insn_blt,
    insn_bge,
    insn_bltu,
    insn_bgeu,
    insn_jalr,
    insn_jal,
    insn_ecall,
    insn_csrrc,
    insn_csrrci,
    insn_csrrs,
    insn_csrrsi,
    insn_csrrw,
    insn_csrrwi,
    insn_flw,
    insn_fsw,
    insn_fmadd_s,
    insn_fmsub_s,
    insn_fnmsub_s,
    insn_fnmadd_s,
    insn_fadd_s,
    insn_fsub_s,
    insn_fmul_s,
    insn_fdiv_s,
    insn_fsqrt_s,
    insn_fsgnj_s,
    insn_fsgnjn_s,
    insn_fsgnjx_s,
    insn_fmin_s,
    insn_fmax_s,
    insn_fcvt_w_s,
    insn_fcvt_wu_s,
    insn_fmv_x_w,
    insn_feq_s,
    insn_flt_s,
    insn_fle_s,
    insn_fclass_s,
    insn_fcvt_s_w,
    insn_fcvt_s_wu,
    insn_fmv_w_x,
    insn_fcvt_l_s,
    insn_fcvt_lu_s,
    insn_fcvt_s_l,
    insn_fcvt_s_lu,
    insn_fld,
    insn_fsd,
    insn_fmadd_d,
    insn_fmsub_d,
    insn_fnmsub_d,
    insn_fnmadd_d,
    insn_fadd_d,
    insn_fsub_d,
    insn_fmul_d,
    insn_fdiv_d,
    insn_fsqrt_d,
    insn_fsgnj_d,
    insn_fsgnjn_d,
    insn_fsgnjx_d,
    insn_fmin_d,
    insn_fmax_d,
    insn_fcvt_s_d,
    insn_fcvt_d_s,
    insn_feq_d,
    insn_flt_d,
    insn_fle_d,
    insn_fclass_d,
    insn_fcvt_w_d,
    insn_fcvt_wu_d,
    insn_fcvt_d_w,
    insn_fcvt_d_wu,
    insn_fcvt_l_d,
    insn_fcvt_lu_d,
    insn_fmv_x_d,
    insn_fcvt_d_l,
    insn_fcvt_d_lu,
    insn_fmv_d_x,
    num_insns,
}

impl Default for insn_t {
    fn default() -> Self {
        insn_t {
            rd: 0,
            rs1: 0,
            rs2: 0,
            rs3: 0,
            imm: 0,
            csr: 0,
            type_: insn_type_t::num_insns, // Use a default variant
            rvc: false,
            cont: false,
        }
    }
}

/*
    get normal types from raw instruction data
*/
pub fn opcode(data: u32) -> u32 {
    (data >> 2) & 0x1f
}
pub fn rd(data: u32) -> u8 {
    ((data >> 7) & 0x1f) as u8
}
pub fn rs1(data: u32) -> u8 {
    ((data >> 15) & 0x1f) as u8
}
pub fn rs2(data: u32) -> u8 {
    ((data >> 20) & 0x1f) as u8
}
pub fn rs3(data: u32) -> u8 {
    ((data >> 27) & 0x1f) as u8
}
pub fn funct2(data: u32) -> u32 {
    (data >> 25) & 0x3
}
pub fn funct3(data: u32) -> u32 {
    (data >> 12) & 0x7
}
pub fn funct7(data: u32) -> u32 {
    (data >> 25) & 0x7f
}
pub fn imm116(data: u32) -> u32 {
    (data >> 26) & 0x3f
}

impl insn_t {
    #[inline]
    pub fn insn_utype_read(&mut self, data: u32) {
        self.imm = (data & 0xfffff000) as i32;
        self.rd = rd(data);
    }

    #[inline]
    pub fn insn_itype_read(&mut self, data: u32) {
        self.imm = (data >> 20) as i32;
        self.rs1 = rs1(data);
        self.rd = rd(data);
    }

    #[inline]
    pub fn insn_jtype_read(&mut self, data: u32) {
        let imm20 = (data >> 31) & 0x1;
        let imm101 = (data >> 21) & 0x3ff;
        let imm11 = (data >> 20) & 0x1;
        let imm1912 = (data >> 12) & 0xff;

        let imm = (imm20 << 20) | (imm1912 << 12) | (imm11 << 11) | (imm101 << 1);
        let imm = (imm << 11) >> 11; // Sign extend

        self.imm = imm as i32;
        self.rd = rd(data);
    }

    #[inline]
    pub fn insn_btype_read(&mut self, data: u32) {
        let imm12 = (data >> 31) & 0x1;
        let imm105 = (data >> 25) & 0x3f;
        let imm41 = (data >> 8) & 0xf;
        let imm11 = (data >> 7) & 0x1;

        let imm = (imm12 << 12) | (imm11 << 11) | (imm105 << 5) | (imm41 << 1);
        let imm = (imm << 19) >> 19; // Sign extend

        self.imm = imm as i32;
        self.rs1 = rs1(data);
        self.rs2 = rs2(data);
    }

    #[inline]
    pub fn insn_rtype_read(&mut self, data: u32) {
        self.rs1 = rs1(data);
        self.rs2 = rs2(data);
        self.rd = rd(data);
    }

    #[inline]
    pub fn insn_stype_read(&mut self, data: u32) {
        let imm115 = (data >> 25) & 0x7f;
        let imm40 = (data >> 7) & 0x1f;

        let imm = (imm115 << 5) | imm40;
        let imm = (imm << 20) >> 20; // Sign extend

        self.imm = imm as i32;
        self.rs1 = rs1(data);
        self.rs2 = rs2(data);
    }

    #[inline]
    pub fn insn_csrtype_read(&mut self, data: u32) {
        self.csr = (data >> 20) as u16;
        self.rs1 = rs1(data);
        self.rd = rd(data);
    }

    #[inline]
    pub fn insn_fprtype_read(&mut self, data: u32) {
        self.rs1 = rs1(data);
        self.rs2 = rs2(data);
        self.rs3 = rs3(data);
        self.rd = rd(data);
    }
}

/*
    get compressed types from raw instruction data
*/
pub fn copcode(data: u16) -> u8 {
    (data >> 13) as u8 & 0x7
}

pub fn cfunct1(data: u16) -> u8 {
    (data >> 12) as u8 & 0x1
}

pub fn cfunct2low(data: u16) -> u8 {
    (data >> 5) as u8 & 0x3
}

pub fn cfunct2high(data: u16) -> u8 {
    (data >> 10) as u8 & 0x3
}

pub fn rp1(data: u16) -> u8 {
    (data >> 7) as u8 & 0x7
}

pub fn rp2(data: u16) -> u8 {
    (data >> 2) as u8 & 0x7
}

pub fn rc1(data: u16) -> u8 {
    (data >> 7) as u8 & 0x7
}

pub fn rc2(data: u16) -> u8 {
    (data >> 2) as u8 & 0x7
}

impl insn_t {
    #[inline]
    pub fn insn_catype_read(&mut self, data: u16) {
        self.rd = rp1(data) + 8;
        self.rs2 = rp2(data) + 8;
        self.rvc = true;
    }

    #[inline]
    pub fn insn_crtype_read(&mut self, data: u16) {
        self.rs1 = rc1(data);
        self.rs2 = rc2(data);
        self.rvc = true;
    }

    #[inline]
    pub fn insn_citype_read(&mut self, data: u16) {
        let imm40 = (data >> 2) & 0x1f;
        let imm5 = (data >> 12) & 0x1;
        let imm = (imm5 << 5) | imm40;
        let imm = ((imm as u32) << 26) >> 26;

        self.imm = imm as i32;
        self.rd = rc1(data);
        self.rvc = true;
    }

    #[inline]
    pub fn insn_citype_read2(&mut self, data: u16) {
        let imm86 = (data >> 2) & 0x7;
        let imm43 = (data >> 5) & 0x3;
        let imm5 = (data >> 12) & 0x1;

        let imm = (imm86 << 6) | (imm43 << 3) | (imm5 << 5);

        self.imm = imm as i32;
        self.rd = rc1(data);
        self.rvc = true;
    }

    #[inline]
    pub fn insn_citype_read3(&mut self, data: u16) {
        let imm5 = (data >> 2) & 0x1;
        let imm87 = (data >> 3) & 0x3;
        let imm6 = (data >> 5) & 0x1;
        let imm4 = (data >> 6) & 0x1;
        let imm9 = (data >> 12) & 0x1;

        let imm = (imm5 << 5) | (imm87 << 7) | (imm6 << 6) | (imm4 << 4) | (imm9 << 9);
        let imm = ((imm as u32) << 22) >> 22;

        self.imm = imm as i32;
        self.rd = rc1(data);
        self.rvc = true;
    }

    #[inline]
    pub fn insn_citype_read4(&mut self, data: u16) {
        let imm5 = (data >> 12) & 0x1;
        let imm42 = (data >> 4) & 0x7;
        let imm76 = (data >> 2) & 0x3;

        let imm = (imm5 << 5) | (imm42 << 2) | (imm76 << 6);

        self.imm = imm as i32;
        self.rd = rc1(data);
        self.rvc = true;
    }

    #[inline]
    pub fn insn_citype_read5(&mut self, data: u16) {
        let imm1612 = (data >> 2) & 0x1f;
        let imm17 = (data >> 12) & 0x1;

        let imm = ((imm1612 as u32) << 12) | ((imm17 as u32) << 17);
        let imm = (imm << 14) >> 14;

        self.imm = imm as i32;
        self.rd = rc1(data);
        self.rvc = true;
    }

    #[inline]
    pub fn insn_cbtype_read(&mut self, data: u16) {
        let imm5 = (data >> 2) & 0x1;
        let imm21 = (data >> 3) & 0x3;
        let imm76 = (data >> 5) & 0x3;
        let imm43 = (data >> 10) & 0x3;
        let imm8 = (data >> 12) & 0x1;

        let imm = (imm8 << 8) | (imm76 << 6) | (imm5 << 5) | (imm43 << 3) | (imm21 << 1);
        let imm = ((imm as u32) << 23) >> 23;

        self.imm = imm as i32;
        self.rs1 = rp1(data) + 8;
        self.rvc = true;
    }

    #[inline]
    pub fn insn_cbtype_read2(&mut self, data: u16) {
        let imm40 = (data >> 2) & 0x1f;
        let imm5 = (data >> 12) & 0x1;
        let imm = (imm5 << 5) | imm40;
        let imm = ((imm as u32) << 26) >> 26;

        self.imm = imm as i32;
        self.rd = rp1(data) + 8;
        self.rvc = true;
    }

    #[inline]
    pub fn insn_cstype_read(&mut self, data: u16) {
        let imm76 = (data >> 5) & 0x3;
        let imm53 = (data >> 10) & 0x7;

        let imm = (imm76 << 6) | (imm53 << 3);

        self.imm = imm as i32;
        self.rs1 = rp1(data) + 8;
        self.rs2 = rp2(data) + 8;
        self.rvc = true;
    }

    #[inline]
    pub fn insn_cstype_read2(&mut self, data: u16) {
        let imm6 = (data >> 5) & 0x1;
        let imm2 = (data >> 6) & 0x1;
        let imm53 = (data >> 10) & 0x7;

        let imm = (imm6 << 6) | (imm2 << 2) | (imm53 << 3);

        self.imm = imm as i32;
        self.rs1 = rp1(data) + 8;
        self.rs2 = rp2(data) + 8;
        self.rvc = true;
    }

    #[inline]
    pub fn insn_cjtype_read(&mut self, data: u16) {
        let imm5 = (data >> 2) & 0x1;
        let imm31 = (data >> 3) & 0x7;
        let imm7 = (data >> 6) & 0x1;
        let imm6 = (data >> 7) & 0x1;
        let imm10 = (data >> 8) & 0x1;
        let imm98 = (data >> 9) & 0x3;
        let imm4 = (data >> 11) & 0x1;
        let imm11 = (data >> 12) & 0x1;

        let imm = (imm5 << 5)
            | (imm31 << 1)
            | (imm7 << 7)
            | (imm6 << 6)
            | (imm10 << 10)
            | (imm98 << 8)
            | (imm4 << 4)
            | (imm11 << 11);
        let imm = ((imm as u32) << 20) >> 20;

        self.imm = imm as i32;
        self.rvc = true;
    }

    #[inline]
    pub fn insn_cltype_read(&mut self, data: u16) {
        let imm6 = (data >> 5) & 0x1;
        let imm2 = (data >> 6) & 0x1;
        let imm53 = (data >> 10) & 0x7;

        let imm = (imm6 << 6) | (imm2 << 2) | (imm53 << 3);

        self.imm = imm as i32;
        self.rs1 = rp1(data) + 8;
        self.rd = rp2(data) + 8;
        self.rvc = true;
    }

    #[inline]
    pub fn insn_cltype_read2(&mut self, data: u16) {
        let imm76 = (data >> 5) & 0x3;
        let imm53 = (data >> 10) & 0x7;

        let imm = (imm76 << 6) | (imm53 << 3);

        self.imm = imm as i32;
        self.rs1 = rp1(data) + 8;
        self.rd = rp2(data) + 8;
        self.rvc = true;
    }

    #[inline]
    pub fn insn_csstype_read(&mut self, data: u16) {
        let imm86 = (data >> 7) & 0x7;
        let imm53 = (data >> 10) & 0x7;

        let imm = (imm86 << 6) | (imm53 << 3);

        self.imm = imm as i32;
        self.rs2 = rc2(data);
        self.rvc = true;
    }

    #[inline]
    pub fn insn_csstype_read2(&mut self, data: u16) {
        let imm76 = (data >> 7) & 0x3;
        let imm52 = (data >> 9) & 0xf;

        let imm = (imm76 << 6) | (imm52 << 2);

        self.imm = imm as i32;
        self.rs2 = rc2(data);
        self.rvc = true;
    }

    #[inline]
    pub fn insn_ciwtype_read(&mut self, data: u16) {
        let imm3 = (data >> 5) & 0x1;
        let imm2 = (data >> 6) & 0x1;
        let imm96 = (data >> 7) & 0xf;
        let imm54 = (data >> 11) & 0x3;

        let imm = (imm3 << 3) | (imm2 << 2) | (imm96 << 6) | (imm54 << 4);

        self.imm = imm as i32;
        self.rd = rp2(data) + 8;
        self.rvc = true;
    }
}
