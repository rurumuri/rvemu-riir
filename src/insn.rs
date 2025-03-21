pub struct insn_t {
    pub rd: i8,
    pub rs1: i8,
    pub rs2: i8,
    pub imm: i32,
    pub type_: insn_type_t,
    pub rvc: bool,
    pub cont: bool
}

pub enum insn_type_t {
    insn_addi,
    num_insns
}