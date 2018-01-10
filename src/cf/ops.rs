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

/// Valid operand combinations.
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum OperandKind {
    /// Any addressing mode (6 bits)
    MdaipmdxnfDXI,

    /// Readable data addressing modes (6 bits)
    Md_ipmdxnfDXI,

    /// Writable addressing modes (6 bits)
    Mdaipmdxnf___,

    /// Writable memory addressing modes (6 bits)
    M__ipmdxnf___,

    /// Data register (3 bits)
    DataReg,

    /// Address register (3 bits)
    AddrReg,

    /// Data or address register (4 bits)
    NormalReg,

    /// Control register (12 bits)
    CtlReg,

    /// Debug control register (5 bits)
    DbgReg,

    /// Condition code register (implicit)
    Ccr,

    /// Condition code register (implicit)
    Sr,

    /// Data/address register list (16 bits in extension word)
    RegList,

    /// Condition code (4 bits),
    Cond,

    /// Cache selector (2 bits)
    CacheSel,

    /// Immediate (16 or 32 bits in extension words)
    Immediate,

    /// Quick immediate (3 bits unsigned; 0 => 8)
    Quick3,

    /// Quick immediate (8 bits signed)
    Quick8,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct OperandPos (u16);

// pos: F EDCBA 98765 43210
// bit: e ccccc bbbbb aaaaa
//      |   |     |     |__ arg[0] bit position
//      |   |     |________ arg[1] bit position
//      |   |______________ arg[2] bit position
//      |__________________ extension word? 0=no 1=yes

macro_rules! pos {
    ($ext:expr) => {
        OperandPos(($ext & 1) << 15)
    };
    ($pos0:expr, $ext:expr) => {
        OperandPos(($ext & 1) << 15 | ($pos0 & 31))
    };
    ($pos0:expr, $pos1:expr, $ext:expr) => {
        OperandPos(($ext & 1) << 15 | ($pos0 & 31) | ($pos1 & 31) << 5)
    };
    ($pos0:expr, $pos1:expr, $pos2:expr, $ext:expr) => {
        OperandPos(($ext & 1) << 15 | ($pos0 & 31) | ($pos1 & 31) << 5 | ($pos2 & 31) << 10)
    };
}

impl OperandPos {
    pub fn has_extension_word(self) -> bool {
        (self.0 as i16).is_negative()
    }

    pub fn operand_pos(self, i: u8) -> u8 {
        match i {
            0 => (self.0       & 31) as u8,
            1 => (self.0 >>  5 & 31) as u8,
            2 => (self.0 >> 10 & 31) as u8,
            _ => panic!("operand_pos: index out of bounds"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn pos_has_extension_word_false() {
        assert_eq!( pos!(0).has_extension_word(), false );
    }

    #[test]
    pub fn pos_has_extension_word_true() {
        assert_eq!( pos!(1).has_extension_word(), true );
    }

    #[test]
    pub fn pos_operand_pos_defined() {
        assert_eq!( pos!(21,        1).operand_pos(0), 21 );

        assert_eq!( pos!(21, 11,    1).operand_pos(0), 21 );
        assert_eq!( pos!(21, 11,    1).operand_pos(1), 11 );

        assert_eq!( pos!(21, 11, 5, 1).operand_pos(0), 21 );
        assert_eq!( pos!(21, 11, 5, 1).operand_pos(1), 11 );
        assert_eq!( pos!(21, 11, 5, 1).operand_pos(2),  5 );
    }

    #[test]
    pub fn pos_operand_pos_undefined() {
        assert_eq!( pos!(        1).operand_pos(0), 0 );
        assert_eq!( pos!(        1).operand_pos(1), 0 );
        assert_eq!( pos!(        1).operand_pos(2), 0 );

        assert_eq!( pos!(21,     1).operand_pos(1), 0 );
        assert_eq!( pos!(21,     1).operand_pos(2), 0 );

        assert_eq!( pos!(21, 11, 1).operand_pos(2), 0 );
    }
}

/*
pub type BitPos = u8;

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

