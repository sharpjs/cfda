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
    Mode,   // TODO

    // Data register pair.
    DataRegPair(DataRegPair),

    // Cache specifier.
    Cache(Cache),
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum Loc {
    DataReg         (Slot<DataReg>),
    AddrReg         (Slot<AddrReg>),
    AddrRegInd      (Slot<AddrReg>),
    AddrRegIndInc   (Slot<AddrReg>),
    AddrRegIndDec   (Slot<AddrReg>),
    AddrRegDisp     (Slot<AddrReg>, Slot<Expr>),
    AddrRegIdx      (Slot<AddrReg>, Slot<Expr>, Slot<Index>),
    PcDisp          (Slot<PcReg>, Slot<Expr>),
    PcIdx           (Slot<PcReg>, Slot<Expr>, Slot<Index>),
    AbsShort        (Slot<Expr>),
    AbsLong         (Slot<Expr>),
    Imm             (Slot<Expr>),
}

/// ColdFire data register pair.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Index {
    reg:   Slot<IndexReg>,
    scale: Slot<Expr>,
}

/// ColdFire data register pair.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum IndexReg {
    Data(DataReg),
    Addr(AddrReg),
}

/// ColdFire data register pair.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct DataRegPair (
    Slot<DataReg>,
    Slot<DataReg>,
);

/// ColdFire cache selectors.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Cache {
    // Inst,    // ic
    // Data,    // dc
    Both,       // bc
}

