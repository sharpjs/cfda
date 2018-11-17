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

use util::DecodeItem;

/// ColdFire instruction specification.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Instruction {                                            // 64-bit    | 32-bit
    /// Preferred mnemonic.
    pub name: &'static str,                                         // +16 => 16 | + 8 =>  8 (bytes)

//  /// Simulation runner.
//  pub run: fn(/*ctx: &mut RunContext*/),                          // + 8 => 24 | + 4 => 12 
}

/// ColdFire opcode specification.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Op {                                                     // 64-bit    | 32-bit
    /// Values of required bits in opword and extension word.
    pub bits: (u16, u16),                                           // + 4 => 12 | + 4 =>  8 
 
    /// Mask of required bits in opword and extension word.
    pub mask: (u16, u16),                                           // + 4 => 16 | + 4 => 12

    /// Number of operands.
    pub arity: u8,                                                  // + 1 => 17 | + 1 => 13

//  /// Size of operands.
//  pub size: Size,                                                 // + 1 => 18 | + 1 => 14
//
//  /// Operand kinds and positions.                
//  pub operands: [Operand; 5],                                     // +10 => 28 | +10 => 24

//  /// Flags (supported architectures, extension word usage)
//  pub flags: Flags,                                               // + 4 => 32 | + 4 => 28
}

impl DecodeItem for Instruction {
    type Word   = u16;
    type Output = &'static Instruction;

    fn try_decode(&self, word: u16) -> Option<&'static Instruction> {
        Some(&ADDL)
    }
}

static ADDL:   Instruction = Instruction { name: "add.l"   };
static ADDAL:  Instruction = Instruction { name: "adda.l"  };
static ADDIL:  Instruction = Instruction { name: "addi.l"  };
static ADDQL:  Instruction = Instruction { name: "addq.l"  };
static ADDXL:  Instruction = Instruction { name: "addx.l"  };
static BCHGB:  Instruction = Instruction { name: "bchg.b"  };
static BCHGL:  Instruction = Instruction { name: "bchg.l"  };
static BCLRB:  Instruction = Instruction { name: "bclr.b"  };
static BCLRL:  Instruction = Instruction { name: "bclr.l"  };
static BSETB:  Instruction = Instruction { name: "bset.b"  };
static BSETL:  Instruction = Instruction { name: "bset.l"  };
static BTSTB:  Instruction = Instruction { name: "btst.b"  };
static BTSTL:  Instruction = Instruction { name: "btst.l"  };
static CMPIB:  Instruction = Instruction { name: "cmpi.b"  };
static CMPIW:  Instruction = Instruction { name: "cmpi.w"  };
static CMPIL:  Instruction = Instruction { name: "cmpi.l"  };
static DIVSW:  Instruction = Instruction { name: "divs.w"  };
static DIVSL:  Instruction = Instruction { name: "divs.l"  };
static DIVUW:  Instruction = Instruction { name: "divu.w"  };
static DIVUL:  Instruction = Instruction { name: "divu.l"  };
static MOVEB:  Instruction = Instruction { name: "move.b"  };
static MOVEW:  Instruction = Instruction { name: "move.w"  };
static MOVEL:  Instruction = Instruction { name: "move.l"  };
static MOVEAW: Instruction = Instruction { name: "movea.w" };
static MOVEAL: Instruction = Instruction { name: "movea.l" };
static MULSW:  Instruction = Instruction { name: "muls.w"  };
static MULSL:  Instruction = Instruction { name: "muls.l"  };
static MULUW:  Instruction = Instruction { name: "mulu.w"  };
static MULUL:  Instruction = Instruction { name: "mulu.l"  };
static SUBL:   Instruction = Instruction { name: "sub.l"   };
static SUBAL:  Instruction = Instruction { name: "suba.l"  };
static SUBIL:  Instruction = Instruction { name: "subi.l"  };
static SUBQL:  Instruction = Instruction { name: "subq.l"  };
static SUBXL:  Instruction = Instruction { name: "subx.l"  };

#[cfg(test)]
mod tests {
}

