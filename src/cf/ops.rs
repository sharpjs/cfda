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
//    /// Instruction specification (name, functions, etc.).
//    pub instruction: &'static Instruction,                          // + 8 =>  8 | + 4 =>  4 (bytes)

    /// Values of required bits in opword and extension word.
    pub bits: (u16, u16),                                           // + 4 => 12 | + 4 =>  8 
 
    /// Mask of required bits in opword and extension word.
    pub mask: (u16, u16),                                           // + 4 => 16 | + 4 => 12

    /// Number of operands.
    pub arity: u8,                                                  // + 1 => 17 | + 1 => 13

//    /// Size of operands.
//    pub size: Size,                                                 // + 1 => 18 | + 1 => 14
//
//    /// Operand kinds and positions.                
//    pub operands: [Operand; 5],                                     // +10 => 28 | +10 => 24

//    /// Flags (supported architectures, extension word usage)
//    pub flags: Flags,                                               // + 4 => 32 | + 4 => 28
}


#[cfg(test)]
mod tests {
}

