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

/// Bit position within opword or extension word.
pub type BitPos = u8;

/// Specifies the bit position and accepted forms of an argument/operand.
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Arg {
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

    /// Readable addressing mode (6 bits)
    MdaipmdxnfDXI(BitPos),

    /// Writable addressing modes (6 bits)
    Mdaipmdxnf___(BitPos),

    /// Readable data addressing modes (6 bits)
    Md_ipmdxnfDXI(BitPos),

    /// Writable data addressing modes (6 bits)
    Md_ipmdxnf___(BitPos),

    /// Readable memory addressing modes without side effects (6 bits)
    M__i__dxnfDX_(BitPos),

    /// Writable memory addressing modes (6 bits)
    M__ipmdxnf___(BitPos),

    /// Source modes for op with extension word (6 bits)
    Md_ipmd______(BitPos),

    /// Address register indirect mode, potentially displaced (movem) (6 bits)
    M__i__d______(BitPos),

    /// Data register or immediate modes (move to ccr/sr) (6 bits)
    Md__________I(BitPos),

    /// Data register (3 bits)
    DataReg(BitPos),

    /// Data register, same one as previous operand (3 bits)
    DataRegSame(BitPos),

    /// Data register, different one from prevous operand (3 bits)
    DataRegDiff(BitPos),

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

    /// User stack pointer (implicit)
    Usp,

    /// Data/address register list (16 bits in extension word)
    RegList,

    /// Cache selector (2 bits)
    CacheSel(BitPos),

    /// Immediate (16 or 32 bits in extension words)
    Immediate,

    /// Quick immediate (3 bits unsigned; 0 => 8)
    Quick3(BitPos),

    /// Quick immediate (4 bits unsigned)
    Quick4(BitPos),

    /// Quick immediate (8 bits signed)
    Quick8(BitPos),

    /// PC-relative immediate offset (8 bits signed)
    PcRel8(BitPos),

    /// PC-relative immediate offset (16 bits signed in extension word)
    PcRel16,

    /// PC-relative immediate offset (32 bits signed in extension words)
    PcRel32,
}

