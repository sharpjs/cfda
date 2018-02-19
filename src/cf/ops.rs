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

/// ColdFire instruction specification.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Instruction {
    pub names: &'static [&'static str],                             // +16 => 16 | + 8 =>  8 (bytes)

    /// Disassembly special-case handler.
    pub disasm: Option<fn(/*ctx: &mut DasmContext*/) -> bool>,      // + 8 => 48 | + 4 => 36

    /// Simulation runner.
    pub run: fn(/*ctx: &mut RunContext*/),                          // + 8 => 56 | + 4 => 40
}

/// ColdFire opcode and operands specification.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Op {                                                     // 64-bit    | 32-bit
    /// Mnemonic names.  The first name is the preferred one.
    pub names: &'static [&'static str],                             // +16 => 16 | + 8 =>  8 (bytes)

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

    /// Data/address register list (16 bits in extension word)
    RegList,

    /// Cache selector (2 bits)
    CacheSel(BitPos),

    /// Immediate (16 or 32 bits in extension words)
    Immediate,

    /// Quick immediate (3 bits unsigned; 0 => 8)
    Quick3(BitPos),

    /// Quick immediate (8 bits signed)
    Quick8(BitPos),

    /// PC-relative immediate offset (8 bits signed)
    PcRel8(BitPos),

    /// PC-relative immediate offset (16 bits signed in extension word)
    PcRel16,

    /// PC-relative immediate offset (32 bits signed in extension words)
    PcRel32,
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

macro_rules! instructions {
    {
        $(
            $id:ident [ $($name:expr),+ ] ;
        )*
    } =>
    {
        $(
            pub static $id: Instruction = Instruction {
                names: &[$($name),+],
                disasm:     None,
                run:        run_stub,
            };
        )*
    };
}

macro_rules! opcodes {
    {
        $(
            $($name:expr),+ =>
                ( $($bits:expr),+ ) ( $($mask:expr),+ )
                [ $( $($arg:tt):+ ),* ] $size:tt $flags:expr ;
        )*
    } =>
    {
        pub static OPCODES: &'static [Op/*code*/] = &[
            $(
                Op/*code*/ {
                    names:    &[$($name),+],
                    bits:     words!($($bits),+),
                    mask:     words!($($mask),+),
                    arity:    0,           // TODO
                    size:     size!($size),
                    operands: operands!($( $($arg):+ ),*),
                    flags:    $flags | ext!($($bits),+),
                },
            )*
        ];
    };
}

macro_rules! words {
    { $a:expr          } => { ($a,  0) };
    { $a:expr, $b:expr } => { ($a, $b) };
}

macro_rules! ext {
    { $a:expr          } => { 0        };
    { $a:expr, $b:expr } => { EXT_WORD };
}

macro_rules! size {
    { - } => { Size::Zero };
    { B } => { Size::Byte };
    { W } => { Size::Word };
    { L } => { Size::Long };
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

instructions! {
//  IDENT  NAMES
//  -----  ---------------------------
    ADDL   ["add.l",  "addl" , "add" ];
    ADDAL  ["adda.l", "addal", "adda"];
    ADDIL  ["addi.l", "addil", "addi"];
    ADDQL  ["addq.l", "addql", "addq"];
    ADDXL  ["addx.l", "addxl", "addx"];
}

opcodes! {
//  NAME                              WORDS             MASKS             OPERANDS                          S  FLAGS
//  ------                            ----------------  ----------------  --------------------------------  -  -----
    "addxl"                        => (0o150600)        (0o170770)        [DataReg:0, DataReg:9]            L  ISA_A_UP;
    "addl"                         => (0o150200)        (0o170700)        [MdaipmdxnfDXI:0, DataReg:9]      L  ISA_A_UP;
    "addl"                         => (0o150600)        (0o170700)        [DataReg:9, M__ipmdxnf___:0]      L  ISA_A_UP;
    "addal"                        => (0o150700)        (0o170700)        [MdaipmdxnfDXI:0, AddrReg:9]      L  ISA_A_UP;
    "addil"                        => (0o003200)        (0o177770)        [Immediate, DataReg:0]            L  ISA_A_UP;
    "addql"                        => (0o050200)        (0o170700)        [Quick3:9, Mdaipmdxnf___:0]       L  ISA_A_UP;

    "andl"                         => (0o140200)        (0o170700)        [Md_ipmdxnfDXI:0, DataReg:9]      L  ISA_A_UP;
    "andl"                         => (0o140600)        (0o170700)        [DataReg:9, M__ipmdxnf___:0]      L  ISA_A_UP;
    "andil"                        => (0o001200)        (0o177770)        [Immediate, DataReg:0]            L  ISA_A_UP;

    "orl"                          => (0o100200)        (0o170700)        [Md_ipmdxnfDXI:0, DataReg:9]      L  ISA_A_UP;
    "orl"                          => (0o100600)        (0o170700)        [DataReg:9, M__ipmdxnf___:0]      L  ISA_A_UP;
    "oril"                         => (0o000200)        (0o177770)        [Immediate, DataReg:0]            L  ISA_A_UP;
                                                                                               
    "eorl"                         => (0o130600)        (0o170700)        [DataReg:9, M__ipmdxnf___:0]      L  ISA_A_UP;
    "eoril"                        => (0o005200)        (0o177770)        [Immediate, DataReg:0]            L  ISA_A_UP;

    "asr.l"                        => (0o160200)        (0o170770)        [Quick3:9, DataReg:0]             L  ISA_A_UP;
    "asl.l"                        => (0o160600)        (0o170770)        [Quick3:9, DataReg:0]             L  ISA_A_UP;
    "asr.l"                        => (0o160240)        (0o170770)        [DataReg:9, DataReg:0]            L  ISA_A_UP;
    "asl.l"                        => (0o160640)        (0o170770)        [DataReg:9, DataReg:0]            L  ISA_A_UP;

    "lsr.l"                        => (0o160210)        (0o170770)        [Quick3:9, DataReg:0]             L  ISA_A_UP;
    "lsl.l"                        => (0o160610)        (0o170770)        [Quick3:9, DataReg:0]             L  ISA_A_UP;
    "lsr.l"                        => (0o160250)        (0o170770)        [DataReg:9, DataReg:0]            L  ISA_A_UP;
    "lsl.l"                        => (0o160650)        (0o170770)        [DataReg:9, DataReg:0]            L  ISA_A_UP;

    "braw"                         => (0x6000)          (0xFFFF)          [PcRel16]                         W  ISA_A_UP;
    "bsrw"                         => (0x6100)          (0xFFFF)          [PcRel16]                         W  ISA_A_UP;
    "bhiw"                         => (0x6200)          (0xFFFF)          [PcRel16]                         W  ISA_A_UP;
    "blsw"                         => (0x6300)          (0xFFFF)          [PcRel16]                         W  ISA_A_UP;
    "bhsw", "bccw"                 => (0x6400)          (0xFFFF)          [PcRel16]                         W  ISA_A_UP;
    "blow", "bcsw"                 => (0x6500)          (0xFFFF)          [PcRel16]                         W  ISA_A_UP;
    "bnew", "bnzw"                 => (0x6600)          (0xFFFF)          [PcRel16]                         W  ISA_A_UP;
    "beqw", "bzw"                  => (0x6700)          (0xFFFF)          [PcRel16]                         W  ISA_A_UP;
    "bvcw"                         => (0x6800)          (0xFFFF)          [PcRel16]                         W  ISA_A_UP;
    "bvsw"                         => (0x6900)          (0xFFFF)          [PcRel16]                         W  ISA_A_UP;
    "bplw"                         => (0x6A00)          (0xFFFF)          [PcRel16]                         W  ISA_A_UP;
    "bmiw"                         => (0x6B00)          (0xFFFF)          [PcRel16]                         W  ISA_A_UP;
    "bgew"                         => (0x6C00)          (0xFFFF)          [PcRel16]                         W  ISA_A_UP;
    "bltw"                         => (0x6D00)          (0xFFFF)          [PcRel16]                         W  ISA_A_UP;
    "bgtw"                         => (0x6E00)          (0xFFFF)          [PcRel16]                         W  ISA_A_UP;
    "blew"                         => (0x6F00)          (0xFFFF)          [PcRel16]                         W  ISA_A_UP;
                                                                                               
    "bral"                         => (0x60FF)          (0xFFFF)          [PcRel32]                         L  ISA_B_UP;
    "bsrl"                         => (0x61FF)          (0xFFFF)          [PcRel32]                         L  ISA_B_UP;
    "bhil"                         => (0x62FF)          (0xFFFF)          [PcRel32]                         L  ISA_B_UP;
    "blsl"                         => (0x63FF)          (0xFFFF)          [PcRel32]                         L  ISA_B_UP;
    "bhsl", "bccl"                 => (0x64FF)          (0xFFFF)          [PcRel32]                         L  ISA_B_UP;
    "blol", "bcsl"                 => (0x65FF)          (0xFFFF)          [PcRel32]                         L  ISA_B_UP;
    "bnel", "bnzl"                 => (0x66FF)          (0xFFFF)          [PcRel32]                         L  ISA_B_UP;
    "beql", "bzl"                  => (0x67FF)          (0xFFFF)          [PcRel32]                         L  ISA_B_UP;
    "bvcl"                         => (0x68FF)          (0xFFFF)          [PcRel32]                         L  ISA_B_UP;
    "bvsl"                         => (0x69FF)          (0xFFFF)          [PcRel32]                         L  ISA_B_UP;
    "bpll"                         => (0x6AFF)          (0xFFFF)          [PcRel32]                         L  ISA_B_UP;
    "bmil"                         => (0x6BFF)          (0xFFFF)          [PcRel32]                         L  ISA_B_UP;
    "bgel"                         => (0x6CFF)          (0xFFFF)          [PcRel32]                         L  ISA_B_UP;
    "bltl"                         => (0x6DFF)          (0xFFFF)          [PcRel32]                         L  ISA_B_UP;
    "bgtl"                         => (0x6EFF)          (0xFFFF)          [PcRel32]                         L  ISA_B_UP;
    "blel"                         => (0x6FFF)          (0xFFFF)          [PcRel32]                         L  ISA_B_UP;
                                                                                               
    "bras", "brab"                 => (0x6000)          (0xFF00)          [PcRel8:0]                        B  ISA_A_UP;
    "bsrs", "bsrb"                 => (0x6100)          (0xFF00)          [PcRel8:0]                        B  ISA_A_UP;
    "bhis", "bhib"                 => (0x6200)          (0xFF00)          [PcRel8:0]                        B  ISA_A_UP;
    "blss", "blsb"                 => (0x6300)          (0xFF00)          [PcRel8:0]                        B  ISA_A_UP;
    "bhss", "bhsb", "bccs", "bccb" => (0x6400)          (0xFF00)          [PcRel8:0]                        B  ISA_A_UP;
    "blos", "blob", "bcss", "bcsb" => (0x6500)          (0xFF00)          [PcRel8:0]                        B  ISA_A_UP;
    "bnes", "bneb", "bnzs", "bnzb" => (0x6600)          (0xFF00)          [PcRel8:0]                        B  ISA_A_UP;
    "beqs", "beqb", "bzs",  "bzb"  => (0x6700)          (0xFF00)          [PcRel8:0]                        B  ISA_A_UP;
    "bvcs", "bvcb"                 => (0x6800)          (0xFF00)          [PcRel8:0]                        B  ISA_A_UP;
    "bvss", "bvsb"                 => (0x6900)          (0xFF00)          [PcRel8:0]                        B  ISA_A_UP;
    "bpls", "bplb"                 => (0x6A00)          (0xFF00)          [PcRel8:0]                        B  ISA_A_UP;
    "bmis", "bmib"                 => (0x6B00)          (0xFF00)          [PcRel8:0]                        B  ISA_A_UP;
    "bges", "bgeb"                 => (0x6C00)          (0xFF00)          [PcRel8:0]                        B  ISA_A_UP;
    "blts", "bltb"                 => (0x6D00)          (0xFF00)          [PcRel8:0]                        B  ISA_A_UP;
    "bgts", "bgtb"                 => (0x6E00)          (0xFF00)          [PcRel8:0]                        B  ISA_A_UP;
    "bles", "bleb"                 => (0x6F00)          (0xFF00)          [PcRel8:0]                        B  ISA_A_UP;

    "nop"                          => (0x4E71)          (0xFFFF)          []                                -  ISA_A_UP;

//  NAME        WORDS                 MASKS                 OPERANDS                                   S  FLAGS
//  ------      --------------------  --------------------  -----------------------------------------  -  -----
    "rems.l" => (0o046100, 0o004000)  (0o177700, 0o107770)  [Md_ipmd______:0, DataReg:16, DataReg:28]  L  HWDIV; // TODO: arg2 cannot be same register as arg1
    "remu.l" => (0o046100, 0o000000)  (0o177700, 0o107770)  [Md_ipmd______:0, DataReg:16, DataReg:28]  L  HWDIV; // TODO: arg2 cannot be same register as arg1
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
        assert_eq!( size_of::<Op>(), 40 /* temporarily, until it is 32 */ );
    }
}

