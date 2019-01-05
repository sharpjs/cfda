// This file is part of cfda, a fun little disassembler project.
// Copyright (C) 2019 Jeffrey Sharp
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

use crate::ast::Expr;
use crate::decode::{* /*, DecodeIndex as X*/};
use crate::mem::{BE, Load};
//use crate::num::Field;
use super::{Arg, AddrReg, DataReg};

/// ColdFire operand kinds and bit positions.
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Operand {
    /// Operand not used.
    None,

    // General addressing modes
    //
    // d a i p m d x n f D X I
    //
    // d: data    register direct
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

    /// Readable addressing mode (6 bits at 5:0)
    MdaipmdxnfDXI0,
/*
    /// Writable addressing modes (6 bits at 5:0)
    Mdaipmdxnf___0,

    /// Readable data addressing modes (6 bits at 5:0)
    Md_ipmdxnfDXI0,

    /// Writable data addressing modes (6 bits at 5:0)
    Md_ipmdxnf___0,

    /// Readable memory addressing modes without side effects (6 bits at 5:0)
    M__i__dxnfDX_0,
*/
    /// Writable memory addressing modes (6 bits at 5:0)
    M__ipmdxnf___0,

    /// Source modes for op with extension word (6 bits at 5:0)
    Md_ipmd______0,
/*
    /// Address register indirect mode, potentially displaced (movem) (6 bits at 5:0)
    M__i__d______0,

    /// Data register or immediate modes (move to ccr/sr) (6 bits at 5:0)
    Md__________I0,
*/
    /// Data register (3 bits at 2:0)
    DataReg0,

    /// Data register (3 bits at 11:9)
    DataReg9,

    /// Data register (3 bits at 30:28)
    DataReg28,
/*
    /// Data register, same one as previous operand (3 bits)
    DataRegSame(BitPos),

    /// Data register, different one from prevous operand (3 bits)
    DataRegDiff(BitPos),
*/
    /// Address register (3 bits at 11:9)
    AddrReg9,
/*
    /// Data or address register (4 bits at ?)
    NormalReg?,

    /// Control register (12 bits at ?)
    CtlReg?,

    /// Debug control register (5 bits at ?)
    DbgReg?,

    /// Condition code register (implicit)
    Ccr,

    /// Condition code register (implicit)
    Sr,

    /// User stack pointer (implicit)
    Usp,

    /// Data/address register list (16 bits in extension word)
    RegList,

    /// Cache selector (2 bits)
    CacheSel?,
*/
    /// Immediate (16 bits in extension words)
    Imm16,

    /// Immediate (32 bits in extension words)
    Imm32,
/*
    /// Quick immediate (3 bits unsigned; 0 => 8; at ?)
    Quick3?,

    /// Quick immediate (4 bits unsigned at ?)
    Quick4?,

    /// Quick immediate (8 bits signed at ?)
    Quick8?,

    /// PC-relative immediate offset (8 bits signed at ?)
    PcRel8?,

    /// PC-relative immediate offset (16 bits signed in extension word)
    PcRel16,

    /// PC-relative immediate offset (32 bits signed in extension words)
    PcRel32,
*/
}

impl Decode<[u8], u16> for Operand {
    type Output = Arg;

    fn decode<'a>(&self, buf: &'a [u8], ctx: &u16) -> Option<(Arg, &'a [u8])> {
        match *self {
            Operand::DataReg0 => {
                Some(( Arg::DataReg(DataReg::decode(*ctx, 0)), buf ))
            },
            Operand::DataReg9 => {
                Some(( Arg::DataReg(DataReg::decode(*ctx, 9)), buf ))
            },
            Operand::AddrReg9 => {
                Some(( Arg::AddrReg(AddrReg::decode(*ctx, 9)), buf ))
            },
            Operand::Imm16 => {
                let (ext, buf) = u16::load(buf, BE)?;
                Some(( Arg::Imm(Expr::LitInt(ext as i64)), buf ))
            },
            Operand::Imm32 => {
                let (ext, buf) = u32::load(buf, BE)?;
                Some(( Arg::Imm(Expr::LitInt(ext as i64)), buf ))
            },
            _ => None
        }
    }
}

/*

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Mode {
    bits: u8,
    mask: u8,
    f:    fn(),
}

static MODES: [Mode; 12] = [
    Mode { bits: 0b_000_000, mask: 0b_111_000, f: mode_data         },
    Mode { bits: 0b_001_000, mask: 0b_111_000, f: mode_addr         },
    Mode { bits: 0b_010_000, mask: 0b_111_000, f: mode_addr_ind     },
    Mode { bits: 0b_011_000, mask: 0b_111_000, f: mode_addr_ind_inc },
    Mode { bits: 0b_100_000, mask: 0b_111_000, f: mode_addr_ind_dec },
    Mode { bits: 0b_101_000, mask: 0b_111_000, f: mode_addr_disp    },
    Mode { bits: 0b_110_000, mask: 0b_111_000, f: mode_addr_index   },
    Mode { bits: 0b_111_000, mask: 0b_111_111, f: mode_abs_short    },
    Mode { bits: 0b_111_001, mask: 0b_111_111, f: mode_abs_long     },
    Mode { bits: 0b_111_010, mask: 0b_111_111, f: mode_pc_disp      },
    Mode { bits: 0b_111_011, mask: 0b_111_111, f: mode_pc_index     },
    Mode { bits: 0b_111_100, mask: 0b_111_111, f: mode_imm          },
];

static MODES_ROOT: DecodeIndex<Mode> =
    /*[..]*/ X::Trie8(&MODES_XX, 3);

static MODES_XX: [DecodeIndex<Mode>; 8] = [
    /*[0.]*/ X::Leaf(&MODES[0]),
    /*[1.]*/ X::Leaf(&MODES[1]),
    /*[2.]*/ X::Leaf(&MODES[2]),
    /*[3.]*/ X::Leaf(&MODES[3]),
    /*[4.]*/ X::Leaf(&MODES[4]),
    /*[5.]*/ X::Leaf(&MODES[5]),
    /*[6.]*/ X::Leaf(&MODES[6]),
    /*[7.]*/ X::Trie8(&MODES_7X, 0),
];

static MODES_7X: [DecodeIndex<Mode>; 8] = [
    /*[70]*/ X::Leaf(&MODES[ 7]),
    /*[71]*/ X::Leaf(&MODES[ 8]),
    /*[72]*/ X::Leaf(&MODES[ 9]),
    /*[73]*/ X::Leaf(&MODES[10]),
    /*[74]*/ X::Leaf(&MODES[11]),
    /*[75]*/ X::Empty,
    /*[76]*/ X::Empty,
    /*[77]*/ X::Empty,
];

const MODE_DATA:            u16 = 1 <<  0;  // d "data"
const MODE_ADDR:            u16 = 1 <<  1;  // a "address"
const MODE_ADDR_IND:        u16 = 1 <<  2;  // i "indirect"
const MODE_ADDR_IND_INC:    u16 = 1 <<  3;  // p "plus"
const MODE_ADDR_IND_DEC:    u16 = 1 <<  4;  // m "minus"
const MODE_ADDR_DISP:       u16 = 1 <<  5;  // d "displacement"
const MODE_ADDR_DISP_IDX:   u16 = 1 <<  6;  // x "index"
const MODE_ABS_NEAR:        u16 = 1 <<  7;  // n "near"
const MODE_ABS_FAR:         u16 = 1 <<  8;  // f "far"
const MODE_PC_DISP:         u16 = 1 <<  9;  // D "Displacement"
const MODE_PC_DISP_IDX:     u16 = 1 << 10;  // X "indeX"
const MODE_IMM:             u16 = 1 << 11;  // I "Immediate"

fn mode_data() {}
fn mode_addr() {}
fn mode_addr_ind() {}
fn mode_addr_ind_inc() {}
fn mode_addr_ind_dec() {}
fn mode_addr_disp() {}
fn mode_addr_index() {}
fn mode_pc_disp() {}
fn mode_pc_index() {}
fn mode_abs_short() {}
fn mode_abs_long() {}
fn mode_imm() {}
*/

