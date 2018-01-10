// This file is part of cfda, a fun little disassembler project.
// Copyright (C) 2017 Jeffrey Sharp
//
// cfda is free software: you can redistribute it and/or modify it
// under the terms of the GNU General Public License as published
// by the Free Software Foundation, either version 3 of the License,
// or (at your option) any later version.
//
// cfda is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See
// the GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with cfda.  If not, see <http://www.gnu.org/licenses/>.

/// ColdFire opcode and operands specification.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Op {
    /// Mnemonic name.
    pub name: &'static str,

    /// Values of required bits in opword and extension word.
    pub bits: (u16, u16),

    /// Mask of required bits in opword and extension word.
    pub mask: (u16, u16),

    /// Operand specifications.
    pub operands: [Operand; 3],

    /// Flags.
    pub flags: u16, // Flags (arch, arity, extension word,)

    /// Performs assembly pass 0 (resolve aliases, validate, collect symbols).
    pub asm0: fn(),

    /// Performs assembly pass 1 (generate binary).
    pub asm1: fn(),

    /// Performs disassembly (validate, apply symbols/aliases).
    pub disasm: fn(),

    /// Formats as code.
    pub fmt: fn(),

    /// Runs in simulation.
    pub run: fn(),
}

pub type BitPos = u8;

/// Specifies the bit position and accepted kinds of an operand.
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Operand {
    // General addressing modes
    //
    // d a i p m d x n f D X I
    //
    // d: data register    direct
    // a: address register direct
    // i: address register indirect
    // p: address register indirect, post-increment
    // m: address register indirect, pre-decrement
    // d: address register indirect + displacement
    // x: address register indirect + displacement + scaled index
    // n: absolute near
    // f: absolute far
    // D: pc-relative + displacement
    // X: pc-relative + displacement + scaled index
    // I: immediate

    /// Any addressing mode (6 bits)
    MdaipmdxnfDXI(BitPos),

    /// Readable data addressing modes (6 bits)
    Md_ipmdxnfDXI(BitPos),

    /// Writable addressing modes (6 bits)
    Mdaipmdxnf___(BitPos),

    /// Writable memory addressing modes (6 bits)
    M__ipmdxnf___(BitPos),

    /// Data register (3 bits)
    DataReg(BitPos),

    /// Address register (3 bits)
    AddrReg(BitPos),

    /// Data or address register (4 bits)
    NormalReg(BitPos),

    /// Control register (12 bits)
    CtlReg(BitPos),

    /// Debug control register (5 bits)
    DbgReg(BitPos),

    /// Condition code register (implicit)
    Ccr,

    /// Condition code register (implicit)
    Sr,

    /// Data/address register list (16 bits in extension word)
    RegList,

    /// Condition code (4 bits),
    Cond(BitPos),

    /// Cache selector (2 bits)
    CacheSel(BitPos),

    /// Immediate (16 or 32 bits following opword)
    Immediate,

    /// Quick immediate (3 bits unsigned; 0 => 8)
    Quick3(BitPos),

    /// Quick immediate (8 bits signed)
    Quick8(BitPos),
}

/*
/// Opcode flags.
pub type Flags = u16;

pub const ISA_A:  Flags = 1 <<  0; // Present in ColdFire ISA_A
pub const ISA_A2: Flags = 1 <<  1; // Present in ColdFire ISA_A+
pub const ISA_B:  Flags = 1 <<  2; // Present in ColdFire ISA_B
pub const ISA_C:  Flags = 1 <<  3; // Present in ColdFire ISA_C
pub const HWDIV:  Flags = 1 <<  4; // Present in ColdFire hardware divide
pub const FPU:    Flags = 1 <<  5; // Present in ColdFire FPU
pub const MAC:    Flags = 1 <<  6; // Present in ColdFire MAC
pub const EMAC:   Flags = 1 <<  7; // Present in ColdFire EMAC
pub const EMAC_B: Flags = 1 <<  8; // Present in ColdFire EMAC_B
pub const MMU:    Flags = 1 <<  9; // Present in ColdFire MMU
pub const USP:    Flags = 1 << 10; // Present in ColdFire user stack pointer

pub const ISA_A_UP:   Flags = ISA_A | ISA_A2 | ISA_B | ISA_C;
pub const ISA_A2_UP:  Flags =         ISA_A2 | ISA_B | ISA_C;
pub const ISA_B_UP:   Flags =                  ISA_B | ISA_C;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[repr(u8)]
pub enum Mnemonic {
    Nop,
    Remsl,
    Remul,
}
*/

/*
static NOP: Op = Op {
    name:  "nop",
    bits:  (0x4E71, 0),
    mask:  (0xFFFF, 0),
    args:  OperandForms::Nullary,
    flags: ISA_A_UP,
};

static REMSL: Op = Op {
    name:  "rems.l",
    bits:  (0o046100, 0o004000),
    mask:  (0o177700, 0o107770),
    args:  OperandForms::Ternary([
        OperandForm::AnyMode(00+00),    // x dividend (TODO)
        OperandForm::DataReg(16+00),    // w remainder
        OperandForm::DataReg(16+12),    // y divisor
    ]),
    flags: HWDIV,
};

static REMUL: Op = Op {
    name:  "remu.l",
    bits:  (0o046100, 0o000000),
    mask:  (0o177700, 0o107770),
    args:  OperandForms::Ternary([
        OperandForm::AnyMode(00+00),    // x dividend (TODO)
        OperandForm::DataReg(16+00),    // w remainder
        OperandForm::DataReg(16+12),    // y divisor
    ]),
    flags: HWDIV,
};
*/

// // Integer user instructions
// Addl,
// Addal,
// Addil,
// Addql,
// Addxl,
// Andl,
// Andil,
// Asll,
// Asrl,
// 
// Beqs,
// Beqw,
// 
// Bges,
// Bgew,
// 
// Bgts,
// Bgtw,
// 
// Bhis,
// Bhiw,
// 
// Bhss,
// Bhsw,
// 
// Bles,
// Blew,
// 
// Blos,
// Blow,
// 
// Blss,
// Blsw,
// 
// Blts,
// Bltw,
// 
// Bmis,
// Bmiw,
// 
// Bnes,
// Bnew,
// 
// Bpls,
// Bplw,
// 
// Bvcs,
// Bvcw,
// 
// Bvss,
// Bvsw,
// 
// Bchgb,
// Bchgl,
// 
// Bclrb,
// Bclrl,
// 
// Bsetb,
// Bsetl,
// 
// Btstb,
// Btstl,
// 
// Bras,
// Braw,
// 
// Bsrs,
// Bsrw,
// 
// Clrb,
// Clrw,
// Clrl,
// 
// Cmpl,
// Cmpal,
// Cmpil,
// 
// Divsw,
// Divsl,
// 
// Divuw,
// Divul,
// 
// Eorl,
// Eoril,
// 
// Extw,
// Extl,
// Extbl,
// 
// Jmp,
// 
// Jsr,
// 
// Leal,
// 
// Linkw,
// 
// Lsll,
// Lsrl,
// 
// Moveb,
// Movew,
// Movel,
// Moveaw,
// Moveal,
// Moveml,
// Moveql,
// 
// Mulsw,
// Mulsl,
// 
// Muluw,
// Mulul,
// 
// Negl,
// 
// Negxl,
// 
// Nop,
// 
// Notl,
// 
// Orl,
// Oril,
// 
// Peal,
// 
// Pulse,
// 
// Rts,
// 
// Seqb,
// Sfb,
// Sgeb,
// Sgtb,
// Shib,
// Shsb,
// Sleb,
// Slob,
// Slsb,
// Sltb,
// Smib,
// Sneb,
// Splb,
// Stb,
// Svcb,
// Svsb,
// 
// Subl,
// Subal,
// Subil,
// Subql,
// Subxl,
// 
// Swapw,
// 
// Remsl,
// 
// Remul,
// 
// Tpf,
// Tpfw,
// Tpfl,
// 
// Trap,
// 
// Tstb,
// Tstw,
// Tstl,
// 
// Unlk,
// 
// Wddatab,
// Wddataw,
// Wddatal,
// 
// // TODO: MAC, EMAC, FPU, ISA_A+...
// 
// // Supervisor instructions
// Cpushl,
// 
// Halt,
// 
// Movecl,
// 
// Rte,
// 
// Stop,
// 
// Wdebugl,

