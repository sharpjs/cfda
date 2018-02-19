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
pub struct Instruction {                                            // 64-bit    | 32-bit
    /// Preferred mnemonic.
    pub name: &'static str,                                         // +16 => 16 | + 8 =>  8 (bytes)

    /// Simulation runner.
    pub run: fn(/*ctx: &mut RunContext*/),                          // + 8 => 24 | + 4 => 12 

    // ... probably more later
}

/// ColdFire opcode specification.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Op {                                                     // 64-bit    | 32-bit
    /// Instruction specification (name, functions, etc.).
    pub instruction: &'static Instruction,                          // + 8 =>  8 | + 4 =>  4 (bytes)

    /// Values of required bits in opword and extension word.
    pub bits: (u16, u16),                                           // + 4 => 12 | + 4 =>  8 
 
    /// Mask of required bits in opword and extension word.
    pub mask: (u16, u16),                                           // + 4 => 16 | + 4 => 12

    /// Number of operands.
    pub arity: u8,                                                  // + 1 => 17 | + 1 => 13

    /// Size of operands.
    pub size: Size,                                                 // + 1 => 18 | + 1 => 14

    /// Operand kinds and positions.                
    pub operands: [Operand; 5],                                     // +10 => 28 | +10 => 24

    /// Flags (supported architectures, extension word usage)
    pub flags: Flags,                                               // + 4 => 32 | + 4 => 28
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

macro_rules! one {
    { $($x:tt)* } => { 1 }
}

macro_rules! instructions {
    {
        $( $id:ident = $name:expr, $run:expr; )*
    } =>
    {
        $(
            pub static $id: Instruction = Instruction {
                name: $name,
                run:  $run,
            };
        )*
    };
}

macro_rules! opcodes {
    {
        $(
            $name:ident
                ( $($bits:expr),+ ) ( $($mask:expr),+ )
                [ $( $($arg:tt):+ ),* ] $size:tt $flags:expr ;
        )*
    } =>
    {
        pub static OPCODES: [Op/*code*/; 0 $(+ one!($name))*] = [
            $(
                Op/*code*/ {
                    instruction: &$name,
                    bits:        words!($($bits),+),
                    mask:        words!($($mask),+),
                    arity:       0,           // TODO
                    size:        size!($size),
                    operands:    operands!($( $($arg):+ ),*),
                    flags:       $flags | ext!($($bits),+),
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
//  Integer User Instructions
//  IDENT     MNEMONIC    RUN
//  -----     --------    --------
    // Add
    ADDL    = "add.l",    run_stub;
    ADDAL   = "adda.l",   run_stub;
    ADDIL   = "addi.l",   run_stub;
    ADDQL   = "addq.l",   run_stub;
    ADDXL   = "addx.l",   run_stub;

    ANDL    = "and.l",    run_stub;
    ANDIL   = "andi.l",   run_stub;

    ORL     = "or.l",     run_stub;
    ORIL    = "ori.l",    run_stub;

    EORL    = "eor.l",    run_stub;
    EORIL   = "eori.l",   run_stub;

    ASRL    = "asr.l",    run_stub;
    ASLL    = "asl.l",    run_stub;

    LSRL    = "lsr.l",    run_stub;
    LSLL    = "lsl.l",    run_stub;

    // Branch
    BRAS    = "bra.s",    run_stub;
    BSRS    = "bsr.s",    run_stub;
    BHIS    = "bhi.s",    run_stub;
    BLSS    = "bls.s",    run_stub;
    BHSS    = "bhs.s",    run_stub;
    BLOS    = "blo.s",    run_stub;
    BNES    = "bne.s",    run_stub;
    BEQS    = "beq.s",    run_stub;
    BVCS    = "bvc.s",    run_stub;
    BVSS    = "bvs.s",    run_stub;
    BPLS    = "bpl.s",    run_stub;
    BMIS    = "bmi.s",    run_stub;
    BGES    = "bge.s",    run_stub;
    BLTS    = "blt.s",    run_stub;
    BGTS    = "bgt.s",    run_stub;
    BLES    = "ble.s",    run_stub;

    BRAW    = "bra.w",    run_stub;
    BSRW    = "bsr.w",    run_stub;
    BHIW    = "bhi.w",    run_stub;
    BLSW    = "bls.w",    run_stub;
    BHSW    = "bhs.w",    run_stub;
    BLOW    = "blo.w",    run_stub;
    BNEW    = "bne.w",    run_stub;
    BEQW    = "beq.w",    run_stub;
    BVCW    = "bvc.w",    run_stub;
    BVSW    = "bvs.w",    run_stub;
    BPLW    = "bpl.w",    run_stub;
    BMIW    = "bmi.w",    run_stub;
    BGEW    = "bge.w",    run_stub;
    BLTW    = "blt.w",    run_stub;
    BGTW    = "bgt.w",    run_stub;
    BLEW    = "ble.w",    run_stub;

    BRAL    = "bra.l",    run_stub;
    BSRL    = "bsr.l",    run_stub;
    BHIL    = "bhi.l",    run_stub;
    BLSL    = "bls.l",    run_stub;
    BHSL    = "bhs.l",    run_stub;
    BLOL    = "blo.l",    run_stub;
    BNEL    = "bne.l",    run_stub;
    BEQL    = "beq.l",    run_stub;
    BVCL    = "bvc.l",    run_stub;
    BVSL    = "bvs.l",    run_stub;
    BPLL    = "bpl.l",    run_stub;
    BMIL    = "bmi.l",    run_stub;
    BGEL    = "bge.l",    run_stub;
    BLTL    = "blt.l",    run_stub;
    BGTL    = "bgt.l",    run_stub;
    BLEL    = "ble.l",    run_stub;

    // Bit
    BCHGB   = "bchg.b",   run_stub;
    BCHGL   = "bchg.l",   run_stub;
                     
    BCLRB   = "bclr.b",   run_stub;
    BCLRL   = "bclr.l",   run_stub;
                     
    BSETB   = "bset.b",   run_stub;
    BSETL   = "bset.l",   run_stub;
                     
    BTSTB   = "btst.b",   run_stub;
    BTSTL   = "btst.l",   run_stub;
                     
    CLRB    = "clr.b",    run_stub;
    CLRW    = "clr.w",    run_stub;
    CLRL    = "clr.l",    run_stub;
                     
    CMPL    = "cmp.l",    run_stub;
    CMPAL   = "cmpa.l",   run_stub;
    CMPIL   = "cmpi.l",   run_stub;
                     
    DIVSW   = "divs.w",   run_stub;
    DIVSL   = "divs.l",   run_stub;
                     
    DIVUW   = "divu.w",   run_stub;
    DIVUL   = "divu.l",   run_stub;
                     
    EXTW    = "ext.w",    run_stub;
    EXTL    = "ext.l",    run_stub;
    EXTBL   = "extb.l",   run_stub;
                     
    JMP     = "jmp",      run_stub;
                     
    JSR     = "jsr",      run_stub;
                     
    LEAL    = "lea.l",    run_stub;
                     
    LINKW   = "link.w",   run_stub;
                     
    MOVEB   = "move.b",   run_stub;
    MOVEW   = "move.w",   run_stub;
    MOVEL   = "move.l",   run_stub;
    MOVEAW  = "movea.w",  run_stub;
    MOVEAL  = "movea.l",  run_stub;
    MOVEML  = "movem.l",  run_stub;
    MOVEQL  = "moveq.l",  run_stub;
                     
    MULSW   = "muls.w",   run_stub;
    MULSL   = "muls.l",   run_stub;
                     
    MULUW   = "mulu.w",   run_stub;
    MULUL   = "mulu.l",   run_stub;
                     
    NEGL    = "neg.l",    run_stub;
                     
    NEGXL   = "negx.l",   run_stub;
                     
    NOP     = "nop",      run_stub;

    NOTL    = "not.l",    run_stub;
                    
    PEAL    = "pea.l",    run_stub;
                    
    PULSE   = "pulse",    run_stub;
                    
    RTS     = "rts",      run_stub;
                    
    SEQB    = "seq.b",    run_stub;
    SFB     = "sf.b",     run_stub;
    SGEB    = "sge.b",    run_stub;
    SGTB    = "sgt.b",    run_stub;
    SHIB    = "shi.b",    run_stub;
    SHSB    = "shs.b",    run_stub;
    SLEB    = "sle.b",    run_stub;
    SLOB    = "slo.b",    run_stub;
    SLSB    = "sls.b",    run_stub;
    SLTB    = "slt.b",    run_stub;
    SMIB    = "smi.b",    run_stub;
    SNEB    = "sne.b",    run_stub;
    SPLB    = "spl.b",    run_stub;
    STB     = "st.b",     run_stub;
    SVCB    = "svc.b",    run_stub;
    SVSB    = "svs.b",    run_stub;
                    
    SUBL    = "sub.l",    run_stub;
    SUBAL   = "suba.l",   run_stub;
    SUBIL   = "subi.l",   run_stub;
    SUBQL   = "subq.l",   run_stub;
    SUBXL   = "subx.l",   run_stub;
                    
    SWAPW   = "swap.w",   run_stub;

    REMSL   = "rems.l",   run_stub;
    REMUL   = "remu.l",   run_stub;

    TPF     = "tpf",      run_stub;
    TPFW    = "tpf.w",    run_stub;
    TPFL    = "tpf.l",    run_stub;
                   
    TRAP    = "trap",     run_stub;
                   
    TSTB    = "tst.b",    run_stub;
    TSTW    = "tst.w",    run_stub;
    TSTL    = "tst.l",    run_stub;
                   
    UNLK    = "unlk",     run_stub;

    WDDATAB = "wddata.b", run_stub;
    WDDATAW = "wddata.w", run_stub;
    WDDATAL = "wddata.l", run_stub;

// TODO: MAC, EMAC, FPU, ISA_A+, ...

//  Supervisor instructions
//
//  IDENT     MNEMONIC    RUN
//  -----     --------    --------
    CPUSHL  = "cpushl",   run_stub;
    HALT    = "halt",     run_stub;
    MOVECL  = "movec.l",  run_stub;
    RTE     = "rte",      run_stub;
    STOP    = "stop",     run_stub;
    WDEBUGL = "wdebug.l", run_stub;
}

opcodes! {
//  NAME     WORDS                 MASKS                 OPERANDS                                       S  FLAGS
//  -------  --------------------  --------------------  ---------------------------------------------  -  -----
    ADDXL    (0o150600)            (0o170770)            [DataReg:0, DataReg:9]                         L  ISA_A_UP;
    ADDL     (0o150200)            (0o170700)            [MdaipmdxnfDXI:0, DataReg:9]                   L  ISA_A_UP;
    ADDL     (0o150600)            (0o170700)            [DataReg:9, M__ipmdxnf___:0]                   L  ISA_A_UP;
    ADDAL    (0o150700)            (0o170700)            [MdaipmdxnfDXI:0, AddrReg:9]                   L  ISA_A_UP;
    ADDIL    (0o003200)            (0o177770)            [Immediate, DataReg:0]                         L  ISA_A_UP;
    ADDQL    (0o050200)            (0o170700)            [Quick3:9, Mdaipmdxnf___:0]                    L  ISA_A_UP;

    ANDL     (0o140200)            (0o170700)            [Md_ipmdxnfDXI:0, DataReg:9]                   L  ISA_A_UP;
    ANDL     (0o140600)            (0o170700)            [DataReg:9, M__ipmdxnf___:0]                   L  ISA_A_UP;
    ANDIL    (0o001200)            (0o177770)            [Immediate, DataReg:0]                         L  ISA_A_UP;

    ORL      (0o100200)            (0o170700)            [Md_ipmdxnfDXI:0, DataReg:9]                   L  ISA_A_UP;
    ORL      (0o100600)            (0o170700)            [DataReg:9, M__ipmdxnf___:0]                   L  ISA_A_UP;
    ORIL     (0o000200)            (0o177770)            [Immediate, DataReg:0]                         L  ISA_A_UP;
                                                                               
    EORL     (0o130600)            (0o170700)            [DataReg:9, M__ipmdxnf___:0]                   L  ISA_A_UP;
    EORIL    (0o005200)            (0o177770)            [Immediate, DataReg:0]                         L  ISA_A_UP;

    ASRL     (0o160200)            (0o170770)            [Quick3:9, DataReg:0]                          L  ISA_A_UP;
    ASLL     (0o160600)            (0o170770)            [Quick3:9, DataReg:0]                          L  ISA_A_UP;
    ASRL     (0o160240)            (0o170770)            [DataReg:9, DataReg:0]                         L  ISA_A_UP;
    ASLL     (0o160640)            (0o170770)            [DataReg:9, DataReg:0]                         L  ISA_A_UP;

    LSRL     (0o160210)            (0o170770)            [Quick3:9, DataReg:0]                          L  ISA_A_UP;
    LSLL     (0o160610)            (0o170770)            [Quick3:9, DataReg:0]                          L  ISA_A_UP;
    LSRL     (0o160250)            (0o170770)            [DataReg:9, DataReg:0]                         L  ISA_A_UP;
    LSLL     (0o160650)            (0o170770)            [DataReg:9, DataReg:0]                         L  ISA_A_UP;

    BRAW     (0x6000)              (0xFFFF)              [PcRel16]                                      W  ISA_A_UP;
    BSRW     (0x6100)              (0xFFFF)              [PcRel16]                                      W  ISA_A_UP;
    BHIW     (0x6200)              (0xFFFF)              [PcRel16]                                      W  ISA_A_UP;
    BLSW     (0x6300)              (0xFFFF)              [PcRel16]                                      W  ISA_A_UP;
    BHSW     (0x6400)              (0xFFFF)              [PcRel16]                                      W  ISA_A_UP;
    BLOW     (0x6500)              (0xFFFF)              [PcRel16]                                      W  ISA_A_UP;
    BNEW     (0x6600)              (0xFFFF)              [PcRel16]                                      W  ISA_A_UP;
    BEQW     (0x6700)              (0xFFFF)              [PcRel16]                                      W  ISA_A_UP;
    BVCW     (0x6800)              (0xFFFF)              [PcRel16]                                      W  ISA_A_UP;
    BVSW     (0x6900)              (0xFFFF)              [PcRel16]                                      W  ISA_A_UP;
    BPLW     (0x6A00)              (0xFFFF)              [PcRel16]                                      W  ISA_A_UP;
    BMIW     (0x6B00)              (0xFFFF)              [PcRel16]                                      W  ISA_A_UP;
    BGEW     (0x6C00)              (0xFFFF)              [PcRel16]                                      W  ISA_A_UP;
    BLTW     (0x6D00)              (0xFFFF)              [PcRel16]                                      W  ISA_A_UP;
    BGTW     (0x6E00)              (0xFFFF)              [PcRel16]                                      W  ISA_A_UP;
    BLEW     (0x6F00)              (0xFFFF)              [PcRel16]                                      W  ISA_A_UP;
                                                                       
    BRAL     (0x60FF)              (0xFFFF)              [PcRel32]                                      L  ISA_B_UP;
    BSRL     (0x61FF)              (0xFFFF)              [PcRel32]                                      L  ISA_B_UP;
    BHIL     (0x62FF)              (0xFFFF)              [PcRel32]                                      L  ISA_B_UP;
    BLSL     (0x63FF)              (0xFFFF)              [PcRel32]                                      L  ISA_B_UP;
    BHSL     (0x64FF)              (0xFFFF)              [PcRel32]                                      L  ISA_B_UP;
    BLOL     (0x65FF)              (0xFFFF)              [PcRel32]                                      L  ISA_B_UP;
    BNEL     (0x66FF)              (0xFFFF)              [PcRel32]                                      L  ISA_B_UP;
    BEQL     (0x67FF)              (0xFFFF)              [PcRel32]                                      L  ISA_B_UP;
    BVCL     (0x68FF)              (0xFFFF)              [PcRel32]                                      L  ISA_B_UP;
    BVSL     (0x69FF)              (0xFFFF)              [PcRel32]                                      L  ISA_B_UP;
    BPLL     (0x6AFF)              (0xFFFF)              [PcRel32]                                      L  ISA_B_UP;
    BMIL     (0x6BFF)              (0xFFFF)              [PcRel32]                                      L  ISA_B_UP;
    BGEL     (0x6CFF)              (0xFFFF)              [PcRel32]                                      L  ISA_B_UP;
    BLTL     (0x6DFF)              (0xFFFF)              [PcRel32]                                      L  ISA_B_UP;
    BGTL     (0x6EFF)              (0xFFFF)              [PcRel32]                                      L  ISA_B_UP;
    BLEL     (0x6FFF)              (0xFFFF)              [PcRel32]                                      L  ISA_B_UP;
                                                                                             
    BRAS     (0x6000)              (0xFF00)              [PcRel8:0]                                     B  ISA_A_UP;
    BSRS     (0x6100)              (0xFF00)              [PcRel8:0]                                     B  ISA_A_UP;
    BHIS     (0x6200)              (0xFF00)              [PcRel8:0]                                     B  ISA_A_UP;
    BLSS     (0x6300)              (0xFF00)              [PcRel8:0]                                     B  ISA_A_UP;
    BHSS     (0x6400)              (0xFF00)              [PcRel8:0]                                     B  ISA_A_UP;
    BLOS     (0x6500)              (0xFF00)              [PcRel8:0]                                     B  ISA_A_UP;
    BNES     (0x6600)              (0xFF00)              [PcRel8:0]                                     B  ISA_A_UP;
    BEQS     (0x6700)              (0xFF00)              [PcRel8:0]                                     B  ISA_A_UP;
    BVCS     (0x6800)              (0xFF00)              [PcRel8:0]                                     B  ISA_A_UP;
    BVSS     (0x6900)              (0xFF00)              [PcRel8:0]                                     B  ISA_A_UP;
    BPLS     (0x6A00)              (0xFF00)              [PcRel8:0]                                     B  ISA_A_UP;
    BMIS     (0x6B00)              (0xFF00)              [PcRel8:0]                                     B  ISA_A_UP;
    BGES     (0x6C00)              (0xFF00)              [PcRel8:0]                                     B  ISA_A_UP;
    BLTS     (0x6D00)              (0xFF00)              [PcRel8:0]                                     B  ISA_A_UP;
    BGTS     (0x6E00)              (0xFF00)              [PcRel8:0]                                     B  ISA_A_UP;
    BLES     (0x6F00)              (0xFF00)              [PcRel8:0]                                     B  ISA_A_UP;

    NOP      (0x4E71)              (0xFFFF)              []                                             -  ISA_A_UP;

    REMSL    (0o046100, 0o004000)  (0o177700, 0o107770)  [Md_ipmd______:0, DataReg:16, DataRegDiff:28]  L  HWDIV;
    REMUL    (0o046100, 0o000000)  (0o177700, 0o107770)  [Md_ipmd______:0, DataReg:16, DataRegDiff:28]  L  HWDIV;
}

macro_rules! aliases {
    {
        $( $name:expr => $instruction:ident ; )*
    } =>
    {
        pub static ALIASES: [(&str, &Instruction); 0 $(+ one!($name))*] = [
            $( ($name, &$instruction) ),*
        ];
    };
}

aliases! {
    "bra.b" => BRAS;
    "bsr.b" => BSRS;
    "bhi.b" => BHIS;
    "bls.b" => BLSS;
    "bhs.b" => BHSS;
    "blo.b" => BLOS;
    "bne.b" => BNES;
    "beq.b" => BEQS;
    "bvc.b" => BVCS;
    "bvs.b" => BVSS;
    "bpl.b" => BPLS;
    "bmi.b" => BMIS;
    "bge.b" => BGES;
    "blt.b" => BLTS;
    "bgt.b" => BGTS;
    "ble.b" => BLES;

    "bcc.s" => BHSS;
    "bcs.s" => BLOS;
    "bnz.s" => BNES;
    "bz.s"  => BEQS;

    "bcc.b" => BHSS;
    "bcs.b" => BLOS;
    "bnz.b" => BNES;
    "bz.b"  => BEQS;

    "bcc.w" => BHSW;
    "bcs.w" => BLOW;
    "bnz.w" => BNEW;
    "bz.w"  => BEQW;

    "bcc.l" => BHSL;
    "bcs.l" => BLOL;
    "bnz.l" => BNEL;
    "bz.l"  => BEQL;
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::mem::size_of;

    #[test]
    fn op_size_of() {
        assert_eq!( size_of::<Op>(), 32 );
    }
}

