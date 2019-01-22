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

use std::fmt::{self, Display, Formatter};
use std::mem::transmute;
use crate::ast::Expr;
use crate::num::{Field, SetField};

/// A ColdFire assembly operation.
#[derive(Clone, Eq, PartialEq, Debug)]
pub enum Op {
    Halt,
    Nop,
    //...
}

// -----------------------------------------------------------------------------
// Arguments

/// A ColdFire argument to an operation.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum Arg {
// General addressing modes

    /// Data register direct.
    DataReg(DataReg),

    /// Address register direct.
    AddrReg(AddrReg),

    /// Address register indirect.
    AddrRegInd(AddrReg),

    /// Address register indirect with post-increment.
    AddrRegIndInc(AddrReg),

    /// Address register indirect with pre-decrement.
    AddrRegIndDec(AddrReg),

    /// Address register indirect with displacement.
    AddrRegDisp(AddrReg, Expr),

    /// Address register indirect with displacement and scaled index.
    AddrRegIdx(AddrReg, Expr, Index),

    /// Program counter relative with displacement.
    PcDisp(PcReg, Expr),

    /// Program counter relative with displacement and scaled index.
    PcIdx(PcReg, Expr, Index),

    /// Absolute short.
    AbsShort(Expr),

    /// Absolute long.
    AbsLong(Expr),

    /// Immediate.
    Imm(Expr),

// Special addressing modes

    /// Data register pair (REMS, REMU).
    DataRegPair(DataRegPair),

    /// Cache specifier.
    Cache(Cache),
}

/// ColdFire index register and scale.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Index {
    /// Index register.
    pub reg: IndexReg,

    /// Scale factor applied to index value.
    pub scale: Expr,
}

/// ColdFire cache selectors.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Cache {
    /// Instruction cache.
    Inst,

    /// Data cache.
    Data,

    /// Both instruction and data caches.
    Both,
}

// -----------------------------------------------------------------------------
// Data Registers

/// A ColdFire data register.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[repr(u8)]
pub enum DataReg {
    D0, D1, D2, D3, D4, D5, D6, D7
}

static DATA_REG_NAMES: [&str; 8] = [
    "d0", "d1", "d2", "d3", "d4", "d5", "d6", "d7"
];

impl DataReg {
    pub const MAX_NUM: u8 = 7;

    #[inline]
    pub unsafe fn with_num_unchecked(n: u8) -> Self {
        transmute(n)
    }

    #[inline]
    pub fn with_num(n: u8) -> Option<Self> {
        if n <= Self::MAX_NUM {
            Some(unsafe { Self::with_num_unchecked(n) })
        } else {
            None
        }
    }

    #[inline]
    pub fn decode<W, P>(word: W, pos: P) -> Self
    where
        W: Copy + Field<P, u8>
    {
        let n = word.field(pos, 0b111);
        unsafe { Self::with_num_unchecked(n) }
    }

    #[inline]
    pub fn encode<W, P>(self, word: &mut W, pos: P)
    where
        W: Copy + SetField<P, u8>
    {
        *word = word.with_field(pos, 0b111, self as u8);
    }

    #[inline]
    pub const fn num(self) -> u8 {
        self as u8
    }

    #[inline]
    pub fn name(self) -> &'static str {
        DATA_REG_NAMES[self as usize]
    }
}

impl Display for DataReg {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str(self.name())
    }
}

// -----------------------------------------------------------------------------
// Address Registers

/// A ColdFire address register.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[repr(u8)]
pub enum AddrReg {
    A0, A1, A2, A3, A4, A5, A6, A7
}

use AddrReg::*;

static ADDR_REG_NAMES: [&str; 8] = [
    "a0", "a1", "a2", "a3", "a4", "a5", "fp", "sp"
];

impl AddrReg {
    pub const MAX_NUM: u8 = 7;

    pub const FP: Self = A6;
    pub const SP: Self = A7;

    #[inline]
    pub unsafe fn with_num_unchecked(n: u8) -> Self {
        transmute(n)
    }

    #[inline]
    pub fn with_num(n: u8) -> Option<Self> {
        if n <= Self::MAX_NUM {
            Some(unsafe { Self::with_num_unchecked(n) })
        } else {
            None
        }
    }

    #[inline]
    pub fn decode<W, P>(word: W, pos: P) -> Self
    where
        W: Copy + Field<P, u8>
    {
        let n = word.field(pos, 0b111);
        unsafe { Self::with_num_unchecked(n) }
    }

    #[inline]
    pub fn encode<W>(self, word: &mut W, pos: u8)
    where
        W: Copy + SetField<u8, u8>
    {
        *word = word.with_field(pos, 0b111, self as u8);
    }

    #[inline]
    pub const fn num(self) -> u8 {
        self as u8
    }

    #[inline]
    pub fn name(self) -> &'static str {
        ADDR_REG_NAMES[self as usize]
    }
}

impl Display for AddrReg {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str(self.name())
    }
}

// -----------------------------------------------------------------------------
// Index Registers

/// ColdFire index register.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum IndexReg {
    /// Data register.
    Data(DataReg),

    /// Address register.
    Addr(AddrReg),
}

impl IndexReg {
    #[inline]
    pub fn decode<W>(word: W, pos: u8) -> Self
    where
        W: Copy + Field<u8, u8>
    {
        let bits = word.field(pos, 0b1111);
        let kind = bits >> 3;
        let num  = bits  & 0b111;

        match kind {
            0 => IndexReg::Data(unsafe { transmute(num) }),
            1 => IndexReg::Addr(unsafe { transmute(num) }),
            _ => unreachable!()
        }
    }

    #[inline]
    pub fn encode<W>(self, word: &mut W, pos: u8)
    where
        W: Copy + SetField<u8, u8>
    {
        let bits = match self {
            IndexReg::Data(r) => {          r as u8 },
            IndexReg::Addr(r) => { 0b1000 | r as u8 },
        };

        *word = word.with_field(pos, 0b1111, bits);
    }
}

impl Display for IndexReg {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            IndexReg::Data(r) => r.fmt(f),
            IndexReg::Addr(r) => r.fmt(f),
        }
    }
}

// -----------------------------------------------------------------------------
// Register Pairs

/// ColdFire data register pair.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct DataRegPair (
    DataReg,  // remainder
    DataReg,  // divisor
);

impl Display for DataRegPair {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.0, self.1)
    }
}

// -----------------------------------------------------------------------------
// Miscellaneous Registers

/// The ColdFire program counter register.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct PcReg;

impl Display for PcReg {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str("pc")
    }
}

/// The ColdFire condition code register.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct CcrReg;

impl Display for CcrReg {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str("ccr")
    }
}

/// The ColdFire status register.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct SrReg;

impl Display for SrReg {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str("sr")
    }
}

// -----------------------------------------------------------------------------
// Tests

#[cfg(test)]
mod tests {
    use super::*;
    use super::DataReg::*;
    //use super::AddrReg::*;

    #[test]
    pub fn data_reg_with_num() {
        assert_eq!( DataReg::with_num(0), Some(D0) );
        assert_eq!( DataReg::with_num(1), Some(D1) );
        assert_eq!( DataReg::with_num(2), Some(D2) );
        assert_eq!( DataReg::with_num(3), Some(D3) );
        assert_eq!( DataReg::with_num(4), Some(D4) );
        assert_eq!( DataReg::with_num(5), Some(D5) );
        assert_eq!( DataReg::with_num(6), Some(D6) );
        assert_eq!( DataReg::with_num(7), Some(D7) );
        assert_eq!( DataReg::with_num(8), None     );
    }

    #[test]
    pub fn data_reg_encode() {
        let mut word = 0u32;
        D0.encode(&mut word, 3); assert_eq!( word, 0o00 );
        D1.encode(&mut word, 3); assert_eq!( word, 0o10 );
        D2.encode(&mut word, 3); assert_eq!( word, 0o20 );
        D3.encode(&mut word, 3); assert_eq!( word, 0o30 );
        D4.encode(&mut word, 3); assert_eq!( word, 0o40 );
        D5.encode(&mut word, 3); assert_eq!( word, 0o50 );
        D6.encode(&mut word, 3); assert_eq!( word, 0o60 );
        D7.encode(&mut word, 3); assert_eq!( word, 0o70 );
    }

    #[test]
    pub fn data_reg_num() {
        assert_eq!( D0.num(), 0 );
        assert_eq!( D1.num(), 1 );
        assert_eq!( D2.num(), 2 );
        assert_eq!( D3.num(), 3 );
        assert_eq!( D4.num(), 4 );
        assert_eq!( D5.num(), 5 );
        assert_eq!( D6.num(), 6 );
        assert_eq!( D7.num(), 7 );
    }

    #[test]
    pub fn data_reg_name() {
        assert_eq!( D0.name(), "d0" );
        assert_eq!( D1.name(), "d1" );
        assert_eq!( D2.name(), "d2" );
        assert_eq!( D3.name(), "d3" );
        assert_eq!( D4.name(), "d4" );
        assert_eq!( D5.name(), "d5" );
        assert_eq!( D6.name(), "d6" );
        assert_eq!( D7.name(), "d7" );
    }

    #[test]
    pub fn data_reg_fmt() {
        assert_eq!( format!("{}", D0), "d0" );
        assert_eq!( format!("{}", D1), "d1" );
        assert_eq!( format!("{}", D2), "d2" );
        assert_eq!( format!("{}", D3), "d3" );
        assert_eq!( format!("{}", D4), "d4" );
        assert_eq!( format!("{}", D5), "d5" );
        assert_eq!( format!("{}", D6), "d6" );
        assert_eq!( format!("{}", D7), "d7" );
    }

    #[test]
    pub fn addr_reg_with_num() {
        assert_eq!( AddrReg::with_num(0), Some(A0) );
        assert_eq!( AddrReg::with_num(1), Some(A1) );
        assert_eq!( AddrReg::with_num(2), Some(A2) );
        assert_eq!( AddrReg::with_num(3), Some(A3) );
        assert_eq!( AddrReg::with_num(4), Some(A4) );
        assert_eq!( AddrReg::with_num(5), Some(A5) );
        assert_eq!( AddrReg::with_num(6), Some(A6) );
        assert_eq!( AddrReg::with_num(7), Some(A7) );
        assert_eq!( AddrReg::with_num(8), None     );
    }

    #[test]
    pub fn addr_reg_num() {
        assert_eq!( A0.num(), 0 );
        assert_eq!( A1.num(), 1 );
        assert_eq!( A2.num(), 2 );
        assert_eq!( A3.num(), 3 );
        assert_eq!( A4.num(), 4 );
        assert_eq!( A5.num(), 5 );
        assert_eq!( A6.num(), 6 );
        assert_eq!( A7.num(), 7 );
    }

    #[test]
    pub fn addr_reg_name() {
        assert_eq!( A0.name(), "a0" );
        assert_eq!( A1.name(), "a1" );
        assert_eq!( A2.name(), "a2" );
        assert_eq!( A3.name(), "a3" );
        assert_eq!( A4.name(), "a4" );
        assert_eq!( A5.name(), "a5" );
        assert_eq!( A6.name(), "fp" );
        assert_eq!( A7.name(), "sp" );
    }

    #[test]
    pub fn addr_reg_fmt() {
        assert_eq!( format!("{}", A0), "a0" );
        assert_eq!( format!("{}", A1), "a1" );
        assert_eq!( format!("{}", A2), "a2" );
        assert_eq!( format!("{}", A3), "a3" );
        assert_eq!( format!("{}", A4), "a4" );
        assert_eq!( format!("{}", A5), "a5" );
        assert_eq!( format!("{}", A6), "fp" );
        assert_eq!( format!("{}", A7), "sp" );
    }

    #[test]
    pub fn addr_reg_aliases() {
        assert_eq!( AddrReg::FP, A6 ); // As of 2019-01-01, * does not import the aliases
        assert_eq!( AddrReg::SP, A7 ); // As of 2019-01-01, * does not import the aliases
    }

    #[test]
    pub fn index_reg_fmt() {
        assert_eq!( format!("{}", IndexReg::Data(D0)), "d0" );
        assert_eq!( format!("{}", IndexReg::Data(D1)), "d1" );
        assert_eq!( format!("{}", IndexReg::Data(D2)), "d2" );
        assert_eq!( format!("{}", IndexReg::Data(D3)), "d3" );
        assert_eq!( format!("{}", IndexReg::Data(D4)), "d4" );
        assert_eq!( format!("{}", IndexReg::Data(D5)), "d5" );
        assert_eq!( format!("{}", IndexReg::Data(D6)), "d6" );
        assert_eq!( format!("{}", IndexReg::Data(D7)), "d7" );
        assert_eq!( format!("{}", IndexReg::Addr(A0)), "a0" );
        assert_eq!( format!("{}", IndexReg::Addr(A1)), "a1" );
        assert_eq!( format!("{}", IndexReg::Addr(A2)), "a2" );
        assert_eq!( format!("{}", IndexReg::Addr(A3)), "a3" );
        assert_eq!( format!("{}", IndexReg::Addr(A4)), "a4" );
        assert_eq!( format!("{}", IndexReg::Addr(A5)), "a5" );
        assert_eq!( format!("{}", IndexReg::Addr(A6)), "fp" );
        assert_eq!( format!("{}", IndexReg::Addr(A7)), "sp" );
    }

    #[test]
    pub fn pc_reg_fmt() {
        assert_eq!( format!("{}", PcReg), "pc" );
    }

    #[test]
    pub fn ccr_reg_fmt() {
        assert_eq!( format!("{}", CcrReg), "ccr" );
    }

    #[test]
    pub fn sr_reg_fmt() {
        assert_eq!( format!("{}", SrReg), "sr" );
    }
}

