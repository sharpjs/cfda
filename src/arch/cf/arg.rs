// This file is part of cfda, a fun little disassembler project.
// Copyright (C) 2018 Jeffrey Sharp
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

    /// Address register (3 bits at ?)
    AddrReg?,

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

    /// Immediate (16 or 32 bits in extension words)
    Immediate,

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

