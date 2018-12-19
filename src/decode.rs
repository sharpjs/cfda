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

/// Trait for context-aware decoding of machine code.
pub trait Decode {
    /// The unit of storage of machine code.
    ///
    /// This type typically is `u8` for byte-oriented architectures, and the
    /// word type for word-oriented architectures.
    type Unit;

    /// Context passed to the decoding operation.
    type Context;

    /// The result of successful decoding.
    type Output;

    /// Attempts to decode the machine code in `buf`, using the context `ctx`.
    ///
    /// If decoding is successful, this method returns a tuple consisting of
    /// the decoded result and the remaining machine code, if any.  If decoding
    /// was not successful, this method returns `None`.
    fn decode(&self, buf: &[Self::Unit], ctx: &Self::Context)
        -> Option<(Self::Output, &[Self::Unit])>;
}

// inconsistencies:
//
// unit    --[read in byte order]--> word
//            ^^^^ and make backtrackable
// context ------------------------> context  (unchanged)
// output  ------------------------> output   (unchanged)
//
// DecodeCursor requirements:
//   - input a buffer of Unit (perhaps u8)
//   - readable in some word size
//   - either backtrackable ... or (prefarably) captures reads already performed
//   - get buffer of remaining units
//   ... if copy, it can backtrack
//
// could use 32 (u16,u16) for m68k "word"

pub trait DecodeCursor: Copy {
    type Unit;
    type Opword: Copy;

    // gets the buf of remaining units
    fn buffer(&self) -> &[Self::Unit];

    // gets the opword that has been read
    fn opword(&self) -> Self::Opword;

    // reads more into the opword
    fn read(&mut self) -> Option<Self::Opword>;
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct M68kDecodeCursor<'a> {
    buffer: &'a [u8],
    opword: u32,
}

impl<'a> DecodeCursor for M68kDecodeCursor<'a> {
    type Unit   = u8;
    type Opword = u32;

    fn buffer(&self) -> &[u8] {
        self.buffer
    }

    fn opword(&self) -> u32 {
        self.opword
    }

    fn read(&mut self) -> Option<u32> {
        // read extension word
        let (read, next) = /* self.buffer.read_u16_be()? */ (0x1234u16, &self.buffer[4..]);

        // put extension word in high half of opword
        let opword = self.opword | (read as u32) << 16;

        // advance
        self.opword = opword;
        self.buffer = next;
        Some(opword)
    }
} 

