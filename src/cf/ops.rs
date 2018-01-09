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

/*
pub type BitPos = u8;

/// Operand form combinations.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum OperandForms {
    /// No operands.
    Nullary,

    /// One operand.
    Unary([OperandForm; 1]),

    /// Two operands.
    Binary([OperandForm; 2]),

    /// Three operands.
    Ternary([OperandForm; 3]),
}

// 1 .  4 4  6 2
//  1    8    8
//
// 1 1  4 4  12 2
//  2    8    14
//    10      14
//        24 bytes
//
// daipmdxnfDXI
// 0123456789AB = 12 bits + 4 bits bitpos => 17 bits
// 
// 0000mmmm mmmmmmmm .pppppPP PPPppppp
// 1....... ........
//
//

/// Operand forms.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum OperandForm {
    /// Modes daipmdxDXnfI (any) (6 bits)
    AnyMode(BitPos),

    /// Modes d_ipmdxDXnfI (any except addr reg) (6 bits)
    DataMode(BitPos),

    /// Modes daipmdx__nf_ (mutable) (6 bits)
    MutMode(BitPos),

    /// Modes __ipmdx__nf_ (mutable memory) (6 bits)
    MutMemMode(BitPos),

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

    /// Immediate (16 or 32 bits in extension words)
    Immediate,

    /// Quick immediate (3 bits; 0 => 8)
    Quick3(BitPos),

    /// Quick immediate (8 bits signed)
    Quick8(BitPos),
}

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

//
// 4 bits  \__ 8 ___
// 4 mask  /        \
// 1 mnem  \         > 16
// 1 arg0   \_ 4    /
// 1 arg1   /   \_ 8
// 1 arg2  /    /
// 2 pos   \__ 4
// 2 flag  /

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[repr(u8)]
pub enum Mnemonic {
    Nop,
    Remsl,
    Remul,
}
*/

/// A ColdFire opcode.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Op {
    /// Values of required bits in opword and extension word.
    pub bits: (u16, u16),

    /// Mask of required bits in opword and extension word.
    pub mask: (u16, u16),

/*
    /// Opcode mnemonic name.
    pub name: Mnemonic,

    /// Argument specification.
    pub arg_kinds: [u8; 3], // ArgSpec,

    pub arg_sites: u16, // 3x packed 5-bit

    /// Flags.
    pub flags: Flags,
*/
}

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

