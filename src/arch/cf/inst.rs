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

/// ColdFire instruction specification.
#[derive(Clone, Copy, Debug)]
pub struct Instruction {
    /// Preferred mnemonic.
    pub name: &'static str,
}

macro_rules! instructions {
    { $( $id:ident: $name:expr; )* } =>
    {
        pub static INSTRUCTIONS: [Instruction; count!($($id)*)] = [$(
            Instruction {
                name: $name
            }
        ),*];

        pub enum Instructions { $($id),* }
    }
}

macro_rules! aliases {
    { $( $name:expr => $id:ident; )* } =>
    {
        pub static ALIASES: [(&str, usize); count!($($name)*)] = [$(
            ($name, Instructions::$id as usize)
        ),*];
    };
}

// ColdFire instructions table
// Source: ColdFire Family Programmer’s Reference Manual, Rev. 3

instructions! {
    // Integer Instructions
    ADDL:     "add.l"     ;
    ADDAL:    "adda.l"    ;
    ADDIL:    "addi.l"    ;
    ADDQL:    "addq.l"    ;
    ADDXL:    "addx.l"    ;
    ANDL:     "and.l"     ;
    ANDIL:    "andi.l"    ;
    ASLL:     "asl.l"     ;
    ASRL:     "asr.l"     ;
    BCHGB:    "bchg.b"    ;
    BCHGL:    "bchg.l"    ;
    BCLRB:    "bclr.b"    ;
    BCLRL:    "bclr.l"    ;
    BITREVL:  "bitrev.l"  ;
    BSETB:    "bset.b"    ;
    BSETL:    "bset.l"    ;
    BTSTB:    "btst.b"    ;
    BTSTL:    "btst.l"    ;
    BYTEREVL: "byterev.l" ;
    CLRB:     "clr.b"     ;
    CLRW:     "clr.w"     ;
    CLRL:     "clr.l"     ;
    CMPB:     "cmp.b"     ;
    CMPW:     "cmp.w"     ;
    CMPL:     "cmp.l"     ;
    CMPAB:    "cmpa.b"    ;
    CMPAW:    "cmpa.w"    ;
    CMPAL:    "cmpa.l"    ;
    CMPIB:    "cmpi.b"    ;
    CMPIW:    "cmpi.w"    ;
    CMPIL:    "cmpi.l"    ;
    CPUSHL:   "cpushl"    ;
    DIVSW:    "divs.w"    ;
    DIVSL:    "divs.l"    ;
    DIVUW:    "divu.w"    ;
    DIVUL:    "divu.l"    ;
    EORL:     "eor.l"     ;
    EORIL:    "eori.l"    ;
    EXTW:     "ext.w"     ;
    EXTL:     "ext.l"     ;
    EXTBL:    "extb.l"    ;
    FF1L:     "ff1.l"     ;
    HALT:     "halt"      ;
    ILLEGAL:  "illegal"   ;
    INTOUCH:  "intouch"   ;
    JMP:      "jmp"       ;
    JSR:      "jsr"       ;
    LEAL:     "lea.l"     ;
    LINKW:    "link.w"    ;
    LSLL:     "lsl.l"     ;
    LSRL:     "lsr.l"     ;
    MOV3QL:   "mov3q.l"   ;
    MOVEB:    "move.b"    ;
    MOVEW:    "move.w"    ;
    MOVEL:    "move.l"    ;
    MOVEAW:   "movea.w"   ;
    MOVEAL:   "movea.l"   ;
    MOVECL:   "movec.l"   ;
    MOVEML:   "movem.l"   ;
    MOVEQL:   "moveq.l"   ;
    MULSW:    "muls.w"    ;
    MULSL:    "muls.l"    ;
    MULUW:    "mulu.w"    ;
    MULUL:    "mulu.l"    ;
    MVSB:     "mvs.b"     ;
    MVSW:     "mvs.w"     ;
    MVZB:     "mvz.b"     ;
    MVZW:     "mvz.w"     ;
    NEGL:     "neg.l"     ;
    NEGXL:    "negx.l"    ;
    NOP:      "nop"       ;
    NOTL:     "not.l"     ;
    ORL:      "or.l"      ;
    ORIL:     "ori.l"     ;
    PEAL:     "pea.l"     ;
    PULSE:    "pulse"     ;
    REMSL:    "rems.l"    ;
    REMUL:    "remu.l"    ;
    RTE:      "rte"       ;
    RTS:      "rts"       ;
    SATSL:    "sats.l"    ;
    SUBL:     "sub.l"     ;
    SUBAL:    "suba.l"    ;
    SUBIL:    "subi.l"    ;
    SUBQL:    "subq.l"    ;
    SUBXL:    "subx.l"    ;
    SWAPW:    "swap.w"    ;
    STOP:     "stop"      ;
    STRLDSR:  "strldsr"   ;
    TASB:     "tas.b"     ;
    TPF:      "tpf"       ;
    TPFW:     "tpf.w"     ;
    TPFL:     "tpf.l"     ;
    TRAP:     "trap"      ;
    TSTB:     "tst.b"     ;
    TSTW:     "tst.w"     ;
    TSTL:     "tst.l"     ;
    UNLK:     "unlk"      ;
    WDDATAB:  "wddata.b"  ;
    WDDATAW:  "wddata.w"  ;
    WDDATAL:  "wddata.l"  ;
    WDEBUGL:  "wdebug.l"  ;

    // Branch Instructions - Unconditional
    BRAB:     "bra.b"     ;
    BRAW:     "bra.w"     ;
    BRAL:     "bra.l"     ;
    BSRB:     "bsr.b"     ;
    BSRW:     "bsr.w"     ;
    BSRL:     "bsr.l"     ;
    // Branch Instructions - Equative
    BEQB:     "beq.b"     ;
    BEQW:     "beq.w"     ;
    BEQL:     "beq.l"     ;
    BNEB:     "bne.b"     ;
    BNEW:     "bne.w"     ;
    BNEL:     "bne.l"     ;
    // Branch Instructions - Unsigned Relative
    BLOB:     "blo.b"     ; // bcs.b
    BLOW:     "blo.w"     ; // bcs.w
    BLOL:     "blo.l"     ; // bcs.l
    BLSB:     "bls.b"     ;
    BLSW:     "bls.w"     ;
    BLSL:     "bls.l"     ;
    BHIB:     "bhi.b"     ;
    BHIW:     "bhi.w"     ;
    BHIL:     "bhi.l"     ;
    BHSB:     "bhs.b"     ; // bcc.b
    BHSW:     "bhs.w"     ; // bcc.w
    BHSL:     "bhs.l"     ; // bcc.l
    // Branch Instructions - Signed Relative
    BLTB:     "blt.b"     ;
    BLTW:     "blt.w"     ;
    BLTL:     "blt.l"     ;
    BLEB:     "ble.b"     ;
    BLEW:     "ble.w"     ;
    BLEL:     "ble.l"     ;
    BGEB:     "bge.b"     ;
    BGEW:     "bge.w"     ;
    BGEL:     "bge.l"     ;
    BGTB:     "bgt.b"     ;
    BGTW:     "bgt.w"     ;
    BGTL:     "bgt.l"     ;
    // Branch Instructions - Flags
    BPLB:     "bpl.b"     ;
    BPLW:     "bpl.w"     ;
    BPLL:     "bpl.l"     ;
    BMIB:     "bmi.b"     ;
    BMIW:     "bmi.w"     ;
    BMIL:     "bmi.l"     ;
    BVCB:     "bvc.b"     ;
    BVCW:     "bvc.w"     ;
    BVCL:     "bvc.l"     ;
    BVSB:     "bvs.b"     ;
    BVSW:     "bvs.w"     ;
    BVSL:     "bvs.l"     ;

    // Set to Condition - Unconditional
    STB:      "st.b"      ;
    SFB:      "sf.b"      ;
    // Set to Condition - Equative
    SEQB:     "seq.b"     ;
    SNEB:     "sne.b"     ;
    // Set to Condition - Unsigned Relative
    SLOB:     "slo.b"     ; // scs.b
    SLSB:     "sls.b"     ;
    SHIB:     "shi.b"     ;
    SHSB:     "shs.b"     ; // scc.b
    // Set to Condition - Signed Relative
    SLTB:     "slt.b"     ;
    SLEB:     "sle.b"     ;
    SGEB:     "sge.b"     ;
    SGTB:     "sgt.b"     ;
    // Set to Condition - Flags
    SPLB:     "spl.b"     ;
    SMIB:     "smi.b"     ;
    SVCB:     "svc.b"     ;
    SVSB:     "svs.b"     ;

    // TODO: Multiply-Accumulate Instructions
    // TODO: Floating-Point Instructions
}

aliases! {
    "bra.s" => BRAB;
    "bsr.s" => BSRB;
    "bhi.s" => BHIB;
    "bls.s" => BLSB;
    "bhs.s" => BHSB;
    "blo.s" => BLOB;
    "bne.s" => BNEB;
    "beq.s" => BEQB;
    "bvc.s" => BVCB;
    "bvs.s" => BVSB;
    "bpl.s" => BPLB;
    "bmi.s" => BMIB;
    "bge.s" => BGEB;
    "blt.s" => BLTB;
    "bgt.s" => BGTB;
    "ble.s" => BLEB;

    "bcc.s" => BHSB;
    "bcs.s" => BLOB;
    "bnz.s" => BNEB;
    "bz.s"  => BEQB;

    "bcc.b" => BHSB;
    "bcs.b" => BLOB;
    "bnz.b" => BNEB;
    "bz.b"  => BEQB;

    "bcc.w" => BHSW;
    "bcs.w" => BLOW;
    "bnz.w" => BNEW;
    "bz.w"  => BEQW;

    "bcc.l" => BHSL;
    "bcs.l" => BLOL;
    "bnz.l" => BNEL;
    "bz.l"  => BEQL;
}

