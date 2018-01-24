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
pub struct Op {                                                     // 64-bit    | 32-bit
    /// Mnemonic names.  The first name is the preferred one.
    pub names: &'static [&'static str],                             // +16 => 16 | + 8 =>  8

    /// Values of required bits in opword and extension word.
    pub bits: (u16, u16),                                           // + 4 => 20 | + 4 => 12
 
    /// Mask of required bits in opword and extension word.
    pub mask: (u16, u16),                                           // + 4 => 24 | + 4 => 16

    /// Number of operands.
    pub arity: u8,                                                  // + 1 => 25 | + 1 => 17

    /// Size of operands.
    pub size: Size,                                                 // + 1 => 26 | + 1 => 18

    /// Operand kinds and positions.                
    pub operands: [Operand; 5],                                     // +10 => 36 | +10 => 28

    /// Flags (supported architectures, extension word usage)
    pub flags: Flags,                                               // + 4 => 40 | + 4 => 32

    /// Disassembly special-case handler.
    pub disasm: Option<fn(/*ctx: &mut DasmContext*/) -> bool>,      // + 8 => 48 | + 4 => 36

    /// Simulation runner.
    pub run: fn(/*ctx: &mut RunContext*/),                          // + 8 => 56 | + 4 => 40

    pub reserved: usize,                                            // + 8 => 64 | + 4 => 44
}

/// Operand sizes.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[repr(u8)]
pub enum Size {
    Zero,
    Byte,
    Word,
    Long
}

/// Bit position within opword or extension word.
pub type BitPos = u8;

/// Specifies the bit position and accepted forms of an operand.
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Operand {
    /// Operand not used.
    None,

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

    /// Source modes for op with extension word (6 bits)
    Md_ipmd______(BitPos),

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

/// Opcode flags.
pub type Flags = u32;

pub const EXT_WORD:   Flags = 1 <<  0; // Uses extension word
pub const ISA_A:      Flags = 1 <<  1; // Present in ColdFire ISA_A
pub const ISA_A2:     Flags = 1 <<  2; // Present in ColdFire ISA_A+
pub const ISA_B:      Flags = 1 <<  3; // Present in ColdFire ISA_B
pub const ISA_C:      Flags = 1 <<  4; // Present in ColdFire ISA_C
pub const HWDIV:      Flags = 1 <<  5; // Present in ColdFire hardware divide
pub const FPU:        Flags = 1 <<  6; // Present in ColdFire FPU
pub const MAC:        Flags = 1 <<  7; // Present in ColdFire MAC
pub const EMAC:       Flags = 1 <<  8; // Present in ColdFire EMAC
pub const EMAC_B:     Flags = 1 <<  9; // Present in ColdFire EMAC_B
pub const MMU:        Flags = 1 << 10; // Present in ColdFire MMU
pub const USP:        Flags = 1 << 11; // Present in ColdFire user stack pointer

pub const ISA_A_UP:   Flags = ISA_A | ISA_A2 | ISA_B | ISA_C;
pub const ISA_A2_UP:  Flags =         ISA_A2 | ISA_B | ISA_C;
pub const ISA_B_UP:   Flags =                  ISA_B | ISA_C;

fn run_stub() { }

static NOP: Op = Op {
    names:      &["nop"],
    bits:       (0x4E71, 0),
    mask:       (0xFFFF, 0),
    arity:      0,
    size:       Size::Zero,
    operands:   [Operand::None,
                 Operand::None,
                 Operand::None,
                 Operand::None,
                 Operand::None],
    flags:      ISA_A_UP,
    disasm:     None,
    run:        run_stub,
    reserved:   0,
};

static REMSL: Op = Op {
    names:      &["rems.l", "remsl"],
    bits:       (0o046100, 0o004000),
    mask:       (0o177700, 0o107770),
    arity:      3,
    size:       Size::Long,
    operands:   [Operand::Md_ipmd______ ( 0),
                 Operand::DataReg       (16),
                 Operand::DataReg       (28), // TODO: Cannot be same register as arg1
                 Operand::None,
                 Operand::None],
    flags:      HWDIV,
    disasm:     None,
    run:        run_stub,
    reserved:   0,
};

static REMUL: Op = Op {
    names:      &["remu.l", "remul"],
    bits:       (0o046100, 0o000000),
    mask:       (0o177700, 0o107770),
    arity:      3,
    size:       Size::Long,
    operands:   [Operand::Md_ipmd______ ( 0),
                 Operand::DataReg       (16),
                 Operand::DataReg       (28), // TODO: Cannot be same register as arg1
                 Operand::None,
                 Operand::None],
    flags:      HWDIV,
    disasm:     None,
    run:        run_stub,
    reserved:   0,
};

macro_rules! opcodes {
    {
        $(
            $name:expr => $size:tt
                ( $($bits:expr),+ ) ( $($mask:expr),+ )
                [ $( $($arg:tt):+ ),* ] $flags:expr ;
        )*
    } =>
    {
        pub static OPCODES: &'static [Op/*code*/] = &[
            $(
                Op/*code*/ {
                    names:      &[$name],
                    bits:       words!($($bits),+),
                    mask:       words!($($mask),+),
                    arity:      0,           // TODO
                    size:       size!($size),
                    operands:   [Operand::None, Operand::None, Operand::None, Operand::None, Operand::None], // args!($( $($arg):+ ),*),
                    flags:      $flags, // | ext!($($bits),+),
                    disasm:     None,
                    run:        run_stub,
                    reserved:   0,
                },
            )*
        ];
    };
}


macro_rules! size {
    { - } => { Size::Zero };
    { S } => { Size::Byte };
    { B } => { Size::Byte };
    { W } => { Size::Word };
    { L } => { Size::Long };
}

macro_rules! words {
    { $a:expr          } => { ($a,  0) };
    { $a:expr, $b:expr } => { ($a, $b) };
}

macro_rules! ext {
    { $a:expr          } => { 0        };
    { $a:expr, $b:expr } => { EXT_WORD };
}

macro_rules! operands {
    { } => {[ operand!(None), operand!(None), operand!(None), operand!(None), operand!(None) ]};

    { $($a:tt):+ }
        => {[ operand!($($a):+), operand!(None), operand!(None), operand!(None), operand!(None) ]};

    { $($a:tt):+, $($b:tt):+ }
        => {[ operand!($($a):+), operand!($($b):+), operand!(None), operand!(None), operand!(None) ]};

    { $($a:tt):+, $($b:tt):+, $($c:tt):+ }
        => {[ operand!($($a):+), operand!($($b):+), operand!($($c):+), operand!(None), operand!(None) ]};

    { $($a:tt):+, $($b:tt):+, $($c:tt):+, $($d:tt):+ }
        => {[ operand!($($a):+), operand!($($b):+), operand!($($c):+), operand!($($d):+), operand!(None) ]};

    { $($a:tt):+, $($b:tt):+, $($c:tt):+, $($d:tt):+, $($e:tt):+ }
        => {[ operand!($($a):+), operand!($($b):+), operand!($($c):+), operand!($($d):+), operand!($($e):+) ]};
}

macro_rules! operand {
    { $kind:ident             } => { Operand::$kind       };
    { $kind:ident : $pos:expr } => { Operand::$kind($pos) };
}

opcodes! {
//  NAME        S  WORDS             MASKS             OPERANDS                          FLAGS
//  ------      -  ----------------  ----------------  --------------------------------  -----
    "adda.l" => L  (0xD1C0)          (0xF1C0)          [MdaipmdxnfDXI:0, AddrReg:9]      ISA_A_UP;
}

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

#[cfg(test)]
mod tests {
    use super::*;

    use std::mem::size_of;

    #[test]
    fn op_size_of() {
        assert_eq!( size_of::<Op>(), 64 );
    }
}

