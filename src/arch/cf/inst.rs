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

use super::flags::*;
use super::{Operand, Operand as O};

/// ColdFire instruction specification.
#[derive(Clone, Copy, Debug)]
pub struct Instruction {
    /// Preferred mnemonic.
    pub name: &'static str,
}

/// ColdFire 1-word instruction encoding.
#[derive(Clone, Copy, Debug)]
pub struct WordEncoding {
    /// Values of required bits in opword.
    pub bits: u16,
 
    /// Mask of required bits in opword.
    pub mask: u16,

    /// Operand kinds and bit positions.
    pub operands: [Operand; 2],

    /// Flags (arity, hardware support)
    pub flags: CfFlags,
}

/// ColdFire 2-word instruction encoding.
#[derive(Clone, Copy, Debug)]
pub struct LongEncoding {
    /// Values of required bits in opword and extension word.
    /// The extension word portion occupies the upper 16 bits.
    pub bits: u32,
 
    /// Mask of required bits in opword and extension word.
    /// The extension word portion occupies the upper 16 bits.
    pub mask: u32,

    /// Operand kinds and bit positions.
    pub operands: [Operand; 5],

    /// Flags (arity, hardware support)
    pub flags: CfFlags,
}

macro_rules! encodings {
    {
        $array:ident: [$type:ident] <$operands:ident, $indexes:ident> =
        $( $name:ident $bits:tt $mask:tt [$($operand:ident),*] $flags:expr; )*
    } =>
    {
        pub static $array: [$type; count!($($name)*)] = [$(
            $type {
                bits:     opcode!($bits),
                mask:     opcode!($mask),
                operands: $operands!($($operand),*),
                flags:    CfFlags::new(count!($($operand)*), $flags),
            }
        ),*];

        pub enum $indexes { $($name),* }
    };
}

macro_rules! opcode {
    { ($a:expr, $b:expr) } => { $a | $b << 16 };
    {  $a:expr           } => { $a            };
}

macro_rules! operands2 {
    {                    } => {[ O::None, O::None ]};
    { $a:ident           } => {[ O::$a,   O::None ]};
    { $a:ident, $b:ident } => {[ O::$a,   O::$b   ]};
}

macro_rules! operands5 {
    {                                                  } => {[ O::None, O::None, O::None, O::None, O::None ]};
    { $a:ident                                         } => {[ O::$a,   O::None, O::None, O::None, O::None ]};
    { $a:ident, $b:ident                               } => {[ O::$a,   O::$b,   O::None, O::None, O::None ]};
    { $a:ident, $b:ident, $c:ident                     } => {[ O::$a,   O::$b,   O::$c,   O::None, O::None ]};
    { $a:ident, $b:ident, $c:ident, $d:ident           } => {[ O::$a,   O::$b,   O::$c,   O::$d,   O::None ]};
    { $a:ident, $b:ident, $c:ident, $d:ident, $e:ident } => {[ O::$a,   O::$b,   O::$c,   O::$d,   O::$e   ]};
}

macro_rules! count {
    { $($x:tt)* } => { 0 $(+ one!($x))* }
}

macro_rules! one {
    { $($x:tt)* } => { 1 }
}

// Source: ColdFire Family Programmer’s Reference Manual, Rev. 3

// Integer Instructions
pub static ADDL:     Instruction = Instruction { name: "add.l"     };
pub static ADDAL:    Instruction = Instruction { name: "adda.l"    };
pub static ADDIL:    Instruction = Instruction { name: "addi.l"    };
pub static ADDQL:    Instruction = Instruction { name: "addq.l"    };
pub static ADDXL:    Instruction = Instruction { name: "addx.l"    };
pub static ANDL:     Instruction = Instruction { name: "and.l"     };
pub static ANDIL:    Instruction = Instruction { name: "andi.l"    };
pub static ASLL:     Instruction = Instruction { name: "asl.l"     };
pub static ASRL:     Instruction = Instruction { name: "asr.l"     };
pub static BCHGB:    Instruction = Instruction { name: "bchg.b"    };
pub static BCHGL:    Instruction = Instruction { name: "bchg.l"    };
pub static BCLRB:    Instruction = Instruction { name: "bclr.b"    };
pub static BCLRL:    Instruction = Instruction { name: "bclr.l"    };
pub static BITREVL:  Instruction = Instruction { name: "bitrev.l"  };
pub static BSETB:    Instruction = Instruction { name: "bset.b"    };
pub static BSETL:    Instruction = Instruction { name: "bset.l"    };
pub static BTSTB:    Instruction = Instruction { name: "btst.b"    };
pub static BTSTL:    Instruction = Instruction { name: "btst.l"    };
pub static BYTEREVL: Instruction = Instruction { name: "byterev.l" };
pub static CLRB:     Instruction = Instruction { name: "clr.b"     };
pub static CLRW:     Instruction = Instruction { name: "clr.w"     };
pub static CLRL:     Instruction = Instruction { name: "clr.l"     };
pub static CMPB:     Instruction = Instruction { name: "cmp.b"     };
pub static CMPW:     Instruction = Instruction { name: "cmp.w"     };
pub static CMPL:     Instruction = Instruction { name: "cmp.l"     };
pub static CMPAB:    Instruction = Instruction { name: "cmpa.b"    };
pub static CMPAW:    Instruction = Instruction { name: "cmpa.w"    };
pub static CMPAL:    Instruction = Instruction { name: "cmpa.l"    };
pub static CMPIB:    Instruction = Instruction { name: "cmpi.b"    };
pub static CMPIW:    Instruction = Instruction { name: "cmpi.w"    };
pub static CMPIL:    Instruction = Instruction { name: "cmpi.l"    };
pub static CPUSHL:   Instruction = Instruction { name: "cpushl"    };
pub static DIVSW:    Instruction = Instruction { name: "divs.w"    };
pub static DIVSL:    Instruction = Instruction { name: "divs.l"    };
pub static DIVUW:    Instruction = Instruction { name: "divu.w"    };
pub static DIVUL:    Instruction = Instruction { name: "divu.l"    };
pub static EORL:     Instruction = Instruction { name: "eor.l"     };
pub static EORIL:    Instruction = Instruction { name: "eori.l"    };
pub static EXTW:     Instruction = Instruction { name: "ext.w"     };
pub static EXTL:     Instruction = Instruction { name: "ext.l"     };
pub static EXTBL:    Instruction = Instruction { name: "extb.l"    };
pub static FF1L:     Instruction = Instruction { name: "ff1.l"     };
pub static HALT:     Instruction = Instruction { name: "halt"      };
pub static ILLEGAL:  Instruction = Instruction { name: "illegal"   };
pub static INTOUCH:  Instruction = Instruction { name: "intouch"   };
pub static JMP:      Instruction = Instruction { name: "jmp"       };
pub static JSR:      Instruction = Instruction { name: "jsr"       };
pub static LEAL:     Instruction = Instruction { name: "lea.l"     };
pub static LINKW:    Instruction = Instruction { name: "link.w"    };
pub static LSLL:     Instruction = Instruction { name: "lsl.l"     };
pub static LSRL:     Instruction = Instruction { name: "lsr.l"     };
pub static MOV3QL:   Instruction = Instruction { name: "mov3q.l"   };
pub static MOVEB:    Instruction = Instruction { name: "move.b"    };
pub static MOVEW:    Instruction = Instruction { name: "move.w"    };
pub static MOVEL:    Instruction = Instruction { name: "move.l"    };
pub static MOVEAW:   Instruction = Instruction { name: "movea.w"   };
pub static MOVEAL:   Instruction = Instruction { name: "movea.l"   };
pub static MOVECL:   Instruction = Instruction { name: "movec.l"   };
pub static MOVEML:   Instruction = Instruction { name: "movem.l"   };
pub static MOVEQL:   Instruction = Instruction { name: "moveq.l"   };
pub static MULSW:    Instruction = Instruction { name: "muls.w"    };
pub static MULSL:    Instruction = Instruction { name: "muls.l"    };
pub static MULUW:    Instruction = Instruction { name: "mulu.w"    };
pub static MULUL:    Instruction = Instruction { name: "mulu.l"    };
pub static MVSB:     Instruction = Instruction { name: "mvs.b"     };
pub static MVSW:     Instruction = Instruction { name: "mvs.w"     };
pub static MVZB:     Instruction = Instruction { name: "mvz.b"     };
pub static MVZW:     Instruction = Instruction { name: "mvz.w"     };
pub static NEGL:     Instruction = Instruction { name: "neg.l"     };
pub static NEGXL:    Instruction = Instruction { name: "negx.l"    };
pub static NOP:      Instruction = Instruction { name: "nop"       };
pub static NOTL:     Instruction = Instruction { name: "not.l"     };
pub static ORL:      Instruction = Instruction { name: "or.l"      };
pub static ORIL:     Instruction = Instruction { name: "ori.l"     };
pub static PEAL:     Instruction = Instruction { name: "pea.l"     };
pub static PULSE:    Instruction = Instruction { name: "pulse"     };
pub static REMSL:    Instruction = Instruction { name: "rems.l"    };
pub static REMUL:    Instruction = Instruction { name: "remu.l"    };
pub static RTE:      Instruction = Instruction { name: "rte"       };
pub static RTS:      Instruction = Instruction { name: "rts"       };
pub static SATSL:    Instruction = Instruction { name: "sats.l"    };
pub static SUBL:     Instruction = Instruction { name: "sub.l"     };
pub static SUBAL:    Instruction = Instruction { name: "suba.l"    };
pub static SUBIL:    Instruction = Instruction { name: "subi.l"    };
pub static SUBQL:    Instruction = Instruction { name: "subq.l"    };
pub static SUBXL:    Instruction = Instruction { name: "subx.l"    };
pub static SWAPW:    Instruction = Instruction { name: "swap.w"    };
pub static STOP:     Instruction = Instruction { name: "stop"      };
pub static STRLDSR:  Instruction = Instruction { name: "strldsr"   };
pub static TASB:     Instruction = Instruction { name: "tas.b"     };
pub static TPF:      Instruction = Instruction { name: "tpf"       };
pub static TPFW:     Instruction = Instruction { name: "tpf.w"     };
pub static TPFL:     Instruction = Instruction { name: "tpf.l"     };
pub static TRAP:     Instruction = Instruction { name: "trap"      };
pub static TSTB:     Instruction = Instruction { name: "tst.b"     };
pub static TSTW:     Instruction = Instruction { name: "tst.w"     };
pub static TSTL:     Instruction = Instruction { name: "tst.l"     };
pub static UNLK:     Instruction = Instruction { name: "unlk"      };
pub static WDDATAB:  Instruction = Instruction { name: "wddata.b"  };
pub static WDDATAW:  Instruction = Instruction { name: "wddata.w"  };
pub static WDDATAL:  Instruction = Instruction { name: "wddata.l"  };
pub static WDEBUGL:  Instruction = Instruction { name: "wdebug.l"  };

// Branch Instructions - Unconditional
pub static BRAB:     Instruction = Instruction { name: "bra.b"     };
pub static BRAW:     Instruction = Instruction { name: "bra.w"     };
pub static BRAL:     Instruction = Instruction { name: "bra.l"     };
pub static BSRB:     Instruction = Instruction { name: "bsr.b"     };
pub static BSRW:     Instruction = Instruction { name: "bsr.w"     };
pub static BSRL:     Instruction = Instruction { name: "bsr.l"     };
// Branch Instructions - Equative
pub static BEQB:     Instruction = Instruction { name: "beq.b"     };
pub static BEQW:     Instruction = Instruction { name: "beq.w"     };
pub static BEQL:     Instruction = Instruction { name: "beq.l"     };
pub static BNEB:     Instruction = Instruction { name: "bne.b"     };
pub static BNEW:     Instruction = Instruction { name: "bne.w"     };
pub static BNEL:     Instruction = Instruction { name: "bne.l"     };
// Branch Instructions - Unsigned Relative
pub static BLOB:     Instruction = Instruction { name: "blo.b"     }; // bcs.b
pub static BLOW:     Instruction = Instruction { name: "blo.w"     }; // bcs.w
pub static BLOL:     Instruction = Instruction { name: "blo.l"     }; // bcs.l
pub static BLSB:     Instruction = Instruction { name: "bls.b"     };
pub static BLSW:     Instruction = Instruction { name: "bls.w"     };
pub static BLSL:     Instruction = Instruction { name: "bls.l"     };
pub static BHIB:     Instruction = Instruction { name: "bhi.b"     };
pub static BHIW:     Instruction = Instruction { name: "bhi.w"     };
pub static BHIL:     Instruction = Instruction { name: "bhi.l"     };
pub static BHSB:     Instruction = Instruction { name: "bhs.b"     }; // bcc.b
pub static BHSW:     Instruction = Instruction { name: "bhs.w"     }; // bcc.w
pub static BHSL:     Instruction = Instruction { name: "bhs.l"     }; // bcc.l
// Branch Instructions - Signed Relative
pub static BLTB:     Instruction = Instruction { name: "blt.b"     };
pub static BLTW:     Instruction = Instruction { name: "blt.w"     };
pub static BLTL:     Instruction = Instruction { name: "blt.l"     };
pub static BLEB:     Instruction = Instruction { name: "ble.b"     };
pub static BLEW:     Instruction = Instruction { name: "ble.w"     };
pub static BLEL:     Instruction = Instruction { name: "ble.l"     };
pub static BGEB:     Instruction = Instruction { name: "bge.b"     };
pub static BGEW:     Instruction = Instruction { name: "bge.w"     };
pub static BGEL:     Instruction = Instruction { name: "bge.l"     };
pub static BGTB:     Instruction = Instruction { name: "bgt.b"     };
pub static BGTW:     Instruction = Instruction { name: "bgt.w"     };
pub static BGTL:     Instruction = Instruction { name: "bgt.l"     };
// Branch Instructions - Flags
pub static BPLB:     Instruction = Instruction { name: "bpl.b"     };
pub static BPLW:     Instruction = Instruction { name: "bpl.w"     };
pub static BPLL:     Instruction = Instruction { name: "bpl.l"     };
pub static BMIB:     Instruction = Instruction { name: "bmi.b"     };
pub static BMIW:     Instruction = Instruction { name: "bmi.w"     };
pub static BMIL:     Instruction = Instruction { name: "bmi.l"     };
pub static BVCB:     Instruction = Instruction { name: "bvc.b"     };
pub static BVCW:     Instruction = Instruction { name: "bvc.w"     };
pub static BVCL:     Instruction = Instruction { name: "bvc.l"     };
pub static BVSB:     Instruction = Instruction { name: "bvs.b"     };
pub static BVSW:     Instruction = Instruction { name: "bvs.w"     };
pub static BVSL:     Instruction = Instruction { name: "bvs.l"     };

// Set to Condition - Unconditional
pub static STB:      Instruction = Instruction { name: "st.b"     };
pub static SFB:      Instruction = Instruction { name: "sf.b"     };
// Set to Condition - Equative
pub static SEQB:     Instruction = Instruction { name: "seq.b"     };
pub static SNEB:     Instruction = Instruction { name: "sne.b"     };
// Set to Condition - Unsigned Relative
pub static SLOB:     Instruction = Instruction { name: "slo.b"     }; // scs.b
pub static SLSB:     Instruction = Instruction { name: "sls.b"     };
pub static SHIB:     Instruction = Instruction { name: "shi.b"     };
pub static SHSB:     Instruction = Instruction { name: "shs.b"     }; // scc.b
// Set to Condition - Signed Relative
pub static SLTB:     Instruction = Instruction { name: "slt.b"     };
pub static SLEB:     Instruction = Instruction { name: "sle.b"     };
pub static SGEB:     Instruction = Instruction { name: "sge.b"     };
pub static SGTB:     Instruction = Instruction { name: "sgt.b"     };
// Set to Condition - Flags
pub static SPLB:     Instruction = Instruction { name: "spl.b"     };
pub static SMIB:     Instruction = Instruction { name: "smi.b"     };
pub static SVCB:     Instruction = Instruction { name: "svc.b"     };
pub static SVSB:     Instruction = Instruction { name: "svs.b"     };

// TODO: Multiply-Accumulate Instructions
// TODO: Floating-Point Instructions

encodings! {
    WORD_ENCODINGS: [WordEncoding] <operands2, WordEncodings> =
    
//  NAME       WORD      MASK      OPERANDS                    FLAGS
//  ---------  --------  --------  --------------------------  -----
    Addl0      0o150200  0o170700  [MdaipmdxnfDXI0, DataReg9]  ISA_A_UP;
    Addl1      0o150600  0o170700  [DataReg9, M__ipmdxnf___0]  ISA_A_UP;
//  ---------  --------  --------  --------------------------  -----
    Byterevl0  0o001300  0o177770  [DataReg0]                  ISA_C;
//  ---------  --------  --------  --------------------------  -----
//  ...
}

encodings! {
    LONG_ENCODINGS: [LongEncoding] <operands5, LongEncodings> =
    
//  NAME       WORD                  MASK                  OPERANDS                     FLAGS
//  ---------  --------------------  --------------------  ---------------------------  -----
    Mulul0     (0o046000, 0o000000)  (0o177700, 0o107777)  [Md_ipmd______0, DataReg28]  ISA_A_UP;
//  ---------  --------------------  --------------------  ---------------------------  -----
    Mulsl0     (0o046000, 0o004000)  (0o177700, 0o107777)  [Md_ipmd______0, DataReg28]  ISA_A_UP;
//  ---------  --------------------  --------------------  ---------------------------  -----
//  ...
}

