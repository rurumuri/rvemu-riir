use crate::insn::*;

#[inline]
fn quadrant(data: u32) -> u32 {
    (data >> 0) & 0x3
}

pub fn insn_decode(insn: &mut insn_t, data: u32) {
    let quadrant = quadrant(data);
    match quadrant {
        0x0 => {
            let copcode = copcode(data as u16);
            match copcode {
                0x0 => {
                    // C.ADDI4SPN
                    insn.insn_ciwtype_read(data as u16);
                    insn.rs1 = 2; // sp
                    insn.type_ = insn_type_t::insn_addi;
                    assert!(insn.imm != 0);
                }
                0x1 => {
                    // C.FLD
                    insn.insn_cltype_read2(data as u16);
                    insn.type_ = insn_type_t::insn_fld;
                }
                0x2 => {
                    // C.LW
                    insn.insn_cltype_read(data as u16);
                    insn.type_ = insn_type_t::insn_lw;
                }
                0x3 => {
                    // C.LD
                    insn.insn_cltype_read2(data as u16);
                    insn.type_ = insn_type_t::insn_ld;
                }
                0x5 => {
                    // C.FSD
                    insn.insn_cstype_read(data as u16);
                    insn.type_ = insn_type_t::insn_fsd;
                }
                0x6 => {
                    // C.SW
                    insn.insn_cstype_read2(data as u16);
                    insn.type_ = insn_type_t::insn_sw;
                }
                0x7 => {
                    // C.SD
                    insn.insn_cstype_read(data as u16);
                    insn.type_ = insn_type_t::insn_sd;
                }
                _ => panic!("unimplemented copcode: {}", copcode),
            }
        }
        0x1 => {
            let copcode = copcode(data as u16);
            match copcode {
                0x0 => {
                    // C.ADDI
                    insn.insn_citype_read(data as u16);
                    insn.rs1 = insn.rd;
                    insn.type_ = insn_type_t::insn_addi;
                }
                0x1 => {
                    // C.ADDIW
                    insn.insn_citype_read(data as u16);
                    assert!(insn.rd != 0);
                    insn.rs1 = insn.rd;
                    insn.type_ = insn_type_t::insn_addiw;
                }
                0x2 => {
                    // C.LI
                    insn.insn_citype_read(data as u16);
                    insn.rs1 = 0; // zero
                    insn.type_ = insn_type_t::insn_addi;
                }
                0x3 => {
                    let rd = rc1(data as u16);
                    if rd == 2 {
                        // C.ADDI16SP
                        insn.insn_citype_read3(data as u16);
                        assert!(insn.imm != 0);
                        insn.rs1 = insn.rd;
                        insn.type_ = insn_type_t::insn_addi;
                    } else {
                        // C.LUI
                        insn.insn_citype_read5(data as u16);
                        assert!(insn.imm != 0);
                        insn.type_ = insn_type_t::insn_lui;
                    }
                }
                0x4 => {
                    let cfunct2high = cfunct2high(data as u16);
                    match cfunct2high {
                        0x0 => {
                            // C.SRLI
                            insn.insn_cbtype_read2(data as u16);
                            insn.rs1 = insn.rd;
                            insn.type_ = insn_type_t::insn_srli;
                        }
                        0x1 => {
                            // C.SRAI
                            insn.insn_cbtype_read2(data as u16);
                            insn.rs1 = insn.rd;
                            insn.type_ = insn_type_t::insn_srai;
                        }
                        0x2 => {
                            // C.ANDI
                            insn.insn_cbtype_read2(data as u16);
                            insn.rs1 = insn.rd;
                            insn.type_ = insn_type_t::insn_andi;
                        }
                        0x3 => {
                            let cfunct1 = cfunct1(data as u16);
                            match cfunct1 {
                                0x0 => {
                                    let cfunct2low = cfunct2low(data as u16);
                                    insn.insn_catype_read(data as u16);
                                    insn.rs1 = insn.rd;
                                    match cfunct2low {
                                        0x0 => insn.type_ = insn_type_t::insn_sub,
                                        0x1 => insn.type_ = insn_type_t::insn_xor,
                                        0x2 => insn.type_ = insn_type_t::insn_or,
                                        0x3 => insn.type_ = insn_type_t::insn_and,
                                        _ => unreachable!(),
                                    }
                                }
                                0x1 => {
                                    let cfunct2low = cfunct2low(data as u16);
                                    insn.insn_catype_read(data as u16);
                                    insn.rs1 = insn.rd;
                                    match cfunct2low {
                                        0x0 => insn.type_ = insn_type_t::insn_subw,
                                        0x1 => insn.type_ = insn_type_t::insn_addw,
                                        _ => unreachable!(),
                                    }
                                }
                                _ => unreachable!(),
                            }
                        }
                        _ => unreachable!(),
                    }
                }
                0x5 => {
                    // C.J
                    insn.insn_cjtype_read(data as u16);
                    insn.rd = 0; // zero
                    insn.type_ = insn_type_t::insn_jal;
                    insn.cont = true;
                }
                0x6 => {
                    // C.BEQZ
                    insn.insn_cbtype_read(data as u16);
                    insn.rs2 = 0; // zero
                    insn.type_ = insn_type_t::insn_beq;
                }
                0x7 => {
                    // C.BNEZ
                    insn.insn_cbtype_read(data as u16);
                    insn.rs2 = 0; // zero
                    insn.type_ = insn_type_t::insn_bne;
                }
                _ => panic!("unrecognized copcode: {}", copcode),
            }
        }
        0x2 => {
            let copcode = copcode(data as u16);
            match copcode {
                0x0 => {
                    // C.SLLI
                    insn.insn_citype_read(data as u16);
                    insn.rs1 = insn.rd;
                    insn.type_ = insn_type_t::insn_slli;
                }
                0x1 => {
                    // C.FLDSP
                    insn.insn_citype_read2(data as u16);
                    insn.rs1 = 2; // sp
                    insn.type_ = insn_type_t::insn_fld;
                }
                0x2 => {
                    // C.LWSP
                    insn.insn_citype_read4(data as u16);
                    assert!(insn.rd != 0);
                    insn.rs1 = 2; // sp
                    insn.type_ = insn_type_t::insn_lw;
                }
                0x3 => {
                    // C.LDSP
                    insn.insn_citype_read2(data as u16);
                    assert!(insn.rd != 0);
                    insn.rs1 = 2; // sp
                    insn.type_ = insn_type_t::insn_ld;
                }
                0x4 => {
                    let cfunct1 = cfunct1(data as u16);
                    match cfunct1 {
                        0x0 => {
                            insn.insn_crtype_read(data as u16);
                            if insn.rs2 == 0 {
                                // C.JR
                                assert!(insn.rs1 != 0);
                                insn.rd = 0; // zero
                                insn.type_ = insn_type_t::insn_jalr;
                                insn.cont = true;
                            } else {
                                // C.MV
                                insn.rd = insn.rs1;
                                insn.rs1 = 0; // zero
                                insn.type_ = insn_type_t::insn_add;
                            }
                        }
                        0x1 => {
                            insn.insn_crtype_read(data as u16);
                            if insn.rs1 == 0 && insn.rs2 == 0 {
                                // C.EBREAK
                                panic!("unimplemented C.EBREAK");
                            } else if insn.rs2 == 0 {
                                // C.JALR
                                insn.rd = 1; // ra
                                insn.type_ = insn_type_t::insn_jalr;
                                insn.cont = true;
                            } else {
                                // C.ADD
                                insn.rd = insn.rs1;
                                insn.type_ = insn_type_t::insn_add;
                            }
                        }
                        _ => unreachable!(),
                    }
                }
                0x5 => {
                    // C.FSDSP
                    insn.insn_csstype_read(data as u16);
                    insn.rs1 = 2; // sp
                    insn.type_ = insn_type_t::insn_fsd;
                }
                0x6 => {
                    // C.SWSP
                    insn.insn_csstype_read2(data as u16);
                    insn.rs1 = 2; // sp
                    insn.type_ = insn_type_t::insn_sw;
                }
                0x7 => {
                    // C.SDSP
                    insn.insn_csstype_read(data as u16);
                    insn.rs1 = 2; // sp
                    insn.type_ = insn_type_t::insn_sd;
                }
                _ => panic!("unrecognized copcode: {}", copcode),
            }
        }
        0x3 => {
            let opcode = opcode(data);
            match opcode {
                0x0 => {
                    let funct3 = funct3(data);
                    insn.insn_itype_read(data);
                    match funct3 {
                        0x0 => insn.type_ = insn_type_t::insn_lb,
                        0x1 => insn.type_ = insn_type_t::insn_lh,
                        0x2 => insn.type_ = insn_type_t::insn_lw,
                        0x3 => insn.type_ = insn_type_t::insn_ld,
                        0x4 => insn.type_ = insn_type_t::insn_lbu,
                        0x5 => insn.type_ = insn_type_t::insn_lhu,
                        0x6 => insn.type_ = insn_type_t::insn_lwu,
                        _ => unreachable!(),
                    }
                }
                0x1 => {
                    let funct3 = funct3(data);
                    insn.insn_itype_read(data);
                    match funct3 {
                        0x2 => insn.type_ = insn_type_t::insn_flw,
                        0x3 => insn.type_ = insn_type_t::insn_fld,
                        _ => unreachable!(),
                    }
                }
                0x3 => {
                    let funct3 = funct3(data);
                    match funct3 {
                        0x0 => {
                            // FENCE
                            *insn = insn_t {
                                type_: insn_type_t::insn_fence,
                                ..Default::default()
                            };
                        }
                        0x1 => {
                            // FENCE.I
                            *insn = insn_t {
                                type_: insn_type_t::insn_fence_i,
                                ..Default::default()
                            };
                        }
                        _ => unreachable!(),
                    }
                }
                0x4 => {
                    let funct3 = funct3(data);
                    insn.insn_itype_read(data);
                    match funct3 {
                        0x0 => insn.type_ = insn_type_t::insn_addi,
                        0x1 => {
                            let imm116 = imm116(data);
                            if imm116 == 0 {
                                insn.type_ = insn_type_t::insn_slli;
                            } else {
                                unreachable!();
                            }
                        }
                        0x2 => insn.type_ = insn_type_t::insn_slti,
                        0x3 => insn.type_ = insn_type_t::insn_sltiu,
                        0x4 => insn.type_ = insn_type_t::insn_xori,
                        0x5 => {
                            let imm116 = imm116(data);
                            if imm116 == 0x0 {
                                insn.type_ = insn_type_t::insn_srli;
                            } else if imm116 == 0x10 {
                                insn.type_ = insn_type_t::insn_srai;
                            } else {
                                unreachable!();
                            }
                        }
                        0x6 => insn.type_ = insn_type_t::insn_ori,
                        0x7 => insn.type_ = insn_type_t::insn_andi,
                        _ => panic!("unrecognized funct3"),
                    }
                }
                0x5 => {
                    // AUIPC
                    insn.insn_utype_read(data);
                    insn.type_ = insn_type_t::insn_auipc;
                }
                0x6 => {
                    let funct3 = funct3(data);
                    let funct7 = funct7(data);
                    insn.insn_itype_read(data);
                    match funct3 {
                        0x0 => insn.type_ = insn_type_t::insn_addiw,
                        0x1 => {
                            assert!(funct7 == 0);
                            insn.type_ = insn_type_t::insn_slliw;
                        }
                        0x5 => match funct7 {
                            0x0 => insn.type_ = insn_type_t::insn_srliw,
                            0x20 => insn.type_ = insn_type_t::insn_sraiw,
                            _ => unreachable!(),
                        },
                        _ => panic!("unimplemented"),
                    }
                }
                0x8 => {
                    let funct3 = funct3(data);
                    insn.insn_stype_read(data);
                    match funct3 {
                        0x0 => insn.type_ = insn_type_t::insn_sb,
                        0x1 => insn.type_ = insn_type_t::insn_sh,
                        0x2 => insn.type_ = insn_type_t::insn_sw,
                        0x3 => insn.type_ = insn_type_t::insn_sd,
                        _ => unreachable!(),
                    }
                }
                0x9 => {
                    let funct3 = funct3(data);
                    insn.insn_stype_read(data);
                    match funct3 {
                        0x2 => insn.type_ = insn_type_t::insn_fsw,
                        0x3 => insn.type_ = insn_type_t::insn_fsd,
                        _ => unreachable!(),
                    }
                }
                0xc => {
                    insn.insn_rtype_read(data);
                    let funct3 = funct3(data);
                    let funct7 = funct7(data);
                    match funct7 {
                        0x0 => match funct3 {
                            0x0 => insn.type_ = insn_type_t::insn_add,
                            0x1 => insn.type_ = insn_type_t::insn_sll,
                            0x2 => insn.type_ = insn_type_t::insn_slt,
                            0x3 => insn.type_ = insn_type_t::insn_sltu,
                            0x4 => insn.type_ = insn_type_t::insn_xor,
                            0x5 => insn.type_ = insn_type_t::insn_srl,
                            0x6 => insn.type_ = insn_type_t::insn_or,
                            0x7 => insn.type_ = insn_type_t::insn_and,
                            _ => unreachable!(),
                        },
                        0x1 => match funct3 {
                            0x0 => insn.type_ = insn_type_t::insn_mul,
                            0x1 => insn.type_ = insn_type_t::insn_mulh,
                            0x2 => insn.type_ = insn_type_t::insn_mulhsu,
                            0x3 => insn.type_ = insn_type_t::insn_mulhu,
                            0x4 => insn.type_ = insn_type_t::insn_div,
                            0x5 => insn.type_ = insn_type_t::insn_divu,
                            0x6 => insn.type_ = insn_type_t::insn_rem,
                            0x7 => insn.type_ = insn_type_t::insn_remu,
                            _ => unreachable!(),
                        },
                        0x20 => match funct3 {
                            0x0 => insn.type_ = insn_type_t::insn_sub,
                            0x5 => insn.type_ = insn_type_t::insn_sra,
                            _ => unreachable!(),
                        },
                        _ => unreachable!(),
                    }
                }
                0xd => {
                    // LUI
                    insn.insn_utype_read(data);
                    insn.type_ = insn_type_t::insn_lui;
                }
                0xe => {
                    insn.insn_rtype_read(data);
                    let funct3 = funct3(data);
                    let funct7 = funct7(data);
                    match funct7 {
                        0x0 => match funct3 {
                            0x0 => insn.type_ = insn_type_t::insn_addw,
                            0x1 => insn.type_ = insn_type_t::insn_sllw,
                            0x5 => insn.type_ = insn_type_t::insn_srlw,
                            _ => unreachable!(),
                        },
                        0x1 => match funct3 {
                            0x0 => insn.type_ = insn_type_t::insn_mulw,
                            0x4 => insn.type_ = insn_type_t::insn_divw,
                            0x5 => insn.type_ = insn_type_t::insn_divuw,
                            0x6 => insn.type_ = insn_type_t::insn_remw,
                            0x7 => insn.type_ = insn_type_t::insn_remuw,
                            _ => unreachable!(),
                        },
                        0x20 => match funct3 {
                            0x0 => insn.type_ = insn_type_t::insn_subw,
                            0x5 => insn.type_ = insn_type_t::insn_sraw,
                            _ => unreachable!(),
                        },
                        _ => unreachable!(),
                    }
                }
                0x10 => {
                    let funct2 = funct2(data);
                    insn.insn_fprtype_read(data);
                    match funct2 {
                        0x0 => insn.type_ = insn_type_t::insn_fmadd_s,
                        0x1 => insn.type_ = insn_type_t::insn_fmadd_d,
                        _ => unreachable!(),
                    }
                }
                0x11 => {
                    let funct2 = funct2(data);
                    insn.insn_fprtype_read(data);
                    match funct2 {
                        0x0 => insn.type_ = insn_type_t::insn_fmsub_s,
                        0x1 => insn.type_ = insn_type_t::insn_fmsub_d,
                        _ => unreachable!(),
                    }
                }
                0x12 => {
                    let funct2 = funct2(data);
                    insn.insn_fprtype_read(data);
                    match funct2 {
                        0x0 => insn.type_ = insn_type_t::insn_fnmsub_s,
                        0x1 => insn.type_ = insn_type_t::insn_fnmsub_d,
                        _ => unreachable!(),
                    }
                }
                0x13 => {
                    let funct2 = funct2(data);
                    insn.insn_fprtype_read(data);
                    match funct2 {
                        0x0 => insn.type_ = insn_type_t::insn_fnmadd_s,
                        0x1 => insn.type_ = insn_type_t::insn_fnmadd_d,
                        _ => unreachable!(),
                    }
                }
                0x14 => {
                    let funct7 = funct7(data);
                    insn.insn_rtype_read(data);
                    match funct7 {
                        0x0 => insn.type_ = insn_type_t::insn_fadd_s,
                        0x1 => insn.type_ = insn_type_t::insn_fadd_d,
                        0x4 => insn.type_ = insn_type_t::insn_fsub_s,
                        0x5 => insn.type_ = insn_type_t::insn_fsub_d,
                        0x8 => insn.type_ = insn_type_t::insn_fmul_s,
                        0x9 => insn.type_ = insn_type_t::insn_fmul_d,
                        0xc => insn.type_ = insn_type_t::insn_fdiv_s,
                        0xd => insn.type_ = insn_type_t::insn_fdiv_d,
                        0x10 => match funct3(data) {
                            0x0 => insn.type_ = insn_type_t::insn_fsgnj_s,
                            0x1 => insn.type_ = insn_type_t::insn_fsgnjn_s,
                            0x2 => insn.type_ = insn_type_t::insn_fsgnjx_s,
                            _ => unreachable!(),
                        },
                        0x11 => match funct3(data) {
                            0x0 => insn.type_ = insn_type_t::insn_fsgnj_d,
                            0x1 => insn.type_ = insn_type_t::insn_fsgnjn_d,
                            0x2 => insn.type_ = insn_type_t::insn_fsgnjx_d,
                            _ => unreachable!(),
                        },
                        0x14 => match funct3(data) {
                            0x0 => insn.type_ = insn_type_t::insn_fmin_s,
                            0x1 => insn.type_ = insn_type_t::insn_fmax_s,
                            _ => unreachable!(),
                        },
                        0x15 => match funct3(data) {
                            0x0 => insn.type_ = insn_type_t::insn_fmin_d,
                            0x1 => insn.type_ = insn_type_t::insn_fmax_d,
                            _ => unreachable!(),
                        },
                        0x20 => {
                            assert!(rs2(data) == 1);
                            insn.type_ = insn_type_t::insn_fcvt_s_d;
                        }
                        0x21 => {
                            assert!(rs2(data) == 0);
                            insn.type_ = insn_type_t::insn_fcvt_d_s;
                        }
                        0x2c => {
                            assert!(insn.rs2 == 0);
                            insn.type_ = insn_type_t::insn_fsqrt_s;
                        }
                        0x2d => {
                            assert!(insn.rs2 == 0);
                            insn.type_ = insn_type_t::insn_fsqrt_d;
                        }
                        0x50 => match funct3(data) {
                            0x0 => insn.type_ = insn_type_t::insn_fle_s,
                            0x1 => insn.type_ = insn_type_t::insn_flt_s,
                            0x2 => insn.type_ = insn_type_t::insn_feq_s,
                            _ => unreachable!(),
                        },
                        0x51 => match funct3(data) {
                            0x0 => insn.type_ = insn_type_t::insn_fle_d,
                            0x1 => insn.type_ = insn_type_t::insn_flt_d,
                            0x2 => insn.type_ = insn_type_t::insn_feq_d,
                            _ => unreachable!(),
                        },
                        0x60 => match rs2(data) {
                            0x0 => insn.type_ = insn_type_t::insn_fcvt_w_s,
                            0x1 => insn.type_ = insn_type_t::insn_fcvt_wu_s,
                            0x2 => insn.type_ = insn_type_t::insn_fcvt_l_s,
                            0x3 => insn.type_ = insn_type_t::insn_fcvt_lu_s,
                            _ => unreachable!(),
                        },
                        0x61 => match rs2(data) {
                            0x0 => insn.type_ = insn_type_t::insn_fcvt_w_d,
                            0x1 => insn.type_ = insn_type_t::insn_fcvt_wu_d,
                            0x2 => insn.type_ = insn_type_t::insn_fcvt_l_d,
                            0x3 => insn.type_ = insn_type_t::insn_fcvt_lu_d,
                            _ => unreachable!(),
                        },
                        0x68 => match rs2(data) {
                            0x0 => insn.type_ = insn_type_t::insn_fcvt_s_w,
                            0x1 => insn.type_ = insn_type_t::insn_fcvt_s_wu,
                            0x2 => insn.type_ = insn_type_t::insn_fcvt_s_l,
                            0x3 => insn.type_ = insn_type_t::insn_fcvt_s_lu,
                            _ => unreachable!(),
                        },
                        0x69 => match rs2(data) {
                            0x0 => insn.type_ = insn_type_t::insn_fcvt_d_w,
                            0x1 => insn.type_ = insn_type_t::insn_fcvt_d_wu,
                            0x2 => insn.type_ = insn_type_t::insn_fcvt_d_l,
                            0x3 => insn.type_ = insn_type_t::insn_fcvt_d_lu,
                            _ => unreachable!(),
                        },
                        0x70 => {
                            assert!(rs2(data) == 0);
                            match funct3(data) {
                                0x0 => insn.type_ = insn_type_t::insn_fmv_x_w,
                                0x1 => insn.type_ = insn_type_t::insn_fclass_s,
                                _ => unreachable!(),
                            }
                        }
                        0x71 => {
                            assert!(rs2(data) == 0);
                            match funct3(data) {
                                0x0 => insn.type_ = insn_type_t::insn_fmv_x_d,
                                0x1 => insn.type_ = insn_type_t::insn_fclass_d,
                                _ => unreachable!(),
                            }
                        }
                        0x78 => {
                            assert!(rs2(data) == 0 && funct3(data) == 0);
                            insn.type_ = insn_type_t::insn_fmv_w_x;
                        }
                        0x79 => {
                            assert!(rs2(data) == 0 && funct3(data) == 0);
                            insn.type_ = insn_type_t::insn_fmv_d_x;
                        }
                        _ => unreachable!(),
                    }
                }
                0x18 => {
                    insn.insn_btype_read(data);
                    let funct3 = funct3(data);
                    match funct3 {
                        0x0 => insn.type_ = insn_type_t::insn_beq,
                        0x1 => insn.type_ = insn_type_t::insn_bne,
                        0x4 => insn.type_ = insn_type_t::insn_blt,
                        0x5 => insn.type_ = insn_type_t::insn_bge,
                        0x6 => insn.type_ = insn_type_t::insn_bltu,
                        0x7 => insn.type_ = insn_type_t::insn_bgeu,
                        _ => unreachable!(),
                    }
                }
                0x19 => {
                    // JALR
                    insn.insn_itype_read(data);
                    insn.type_ = insn_type_t::insn_jalr;
                    insn.cont = true;
                }
                0x1b => {
                    // JAL
                    insn.insn_jtype_read(data);
                    insn.type_ = insn_type_t::insn_jal;
                    insn.cont = true;
                }
                0x1c => {
                    if data == 0x73 {
                        // ECALL
                        insn.type_ = insn_type_t::insn_ecall;
                        insn.cont = true;
                    } else {
                        let funct3 = funct3(data);
                        insn.insn_csrtype_read(data);
                        match funct3 {
                            0x1 => insn.type_ = insn_type_t::insn_csrrw,
                            0x2 => insn.type_ = insn_type_t::insn_csrrs,
                            0x3 => insn.type_ = insn_type_t::insn_csrrc,
                            0x5 => insn.type_ = insn_type_t::insn_csrrwi,
                            0x6 => insn.type_ = insn_type_t::insn_csrrsi,
                            0x7 => insn.type_ = insn_type_t::insn_csrrci,
                            _ => unreachable!(),
                        }
                    }
                }
                _ => unreachable!(),
            }
        }
        _ => unreachable!(),
    }
}
