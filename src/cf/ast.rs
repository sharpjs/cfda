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

use ast::{Expr, Slot};
use super::reg::{AddrReg, DataReg, PcReg};

/// A ColdFire-specific argument to an operation.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum CfArg {
// General addressing modes

    /// Data register direct.
    DataReg(Slot<DataReg>),

    /// Address register direct.
    AddrReg(Slot<AddrReg>),

    /// Address register indirect.
    AddrRegInd(Slot<AddrReg>),

    /// Address register indirect with post-increment.
    AddrRegIndInc(Slot<AddrReg>),

    /// Address register indirect with pre-decrement.
    AddrRegIndDec(Slot<AddrReg>),

    /// Address register indirect with displacement.
    AddrRegDisp(Slot<AddrReg>, Slot<Expr>),

    /// Address register indirect with displacement and scaled index.
    AddrRegIdx(Slot<AddrReg>, Slot<Expr>, Slot<Index>),

    /// Program counter relative with displacement.
    PcDisp(Slot<PcReg>, Slot<Expr>),

    /// Program counter relative with displacement and scaled index.
    PcIdx(Slot<PcReg>, Slot<Expr>, Slot<Index>),

    /// Absolute short.
    AbsShort(Slot<Expr>),

    /// Absolute long.
    AbsLong(Slot<Expr>),

    /// Immediate.
    Imm(Slot<Expr>),

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
    pub reg: Slot<IndexReg>,

    /// Scale factor applied to index value.
    pub scale: Slot<Expr>,
}

/// ColdFire index register.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum IndexReg {
    /// Data register.
    Data(DataReg),

    /// Address register.
    Addr(AddrReg),
}

/// ColdFire data register pair.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct DataRegPair (
    Slot<DataReg>,  // remainder
    Slot<DataReg>,  // divisor
);

/// ColdFire cache selectors.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Cache {
    /// Both instruction and data caches.
    Both,
}

