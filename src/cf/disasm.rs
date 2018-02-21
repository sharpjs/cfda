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

//
// This is very likely to change.
//

#[derive(Clone, Copy, Debug)]
pub struct Disassembler<I: Iterator<Item=u8>> {
    bytes: I,
    vma:   u32,
}

impl<I> Disassembler<I> where I: Iterator<Item=u8> {
    //
    pub fn new(bytes: I, vma: u32) -> Self { 
        Disassembler { bytes, vma }
    }

    //
    pub fn process(&mut self) {
        /*loop*/ {
            let vma = self.vma;

            let hi = match self.bytes.next() {
                Some(b) => b as u16,
                None    => return, // end of file
            };

            let lo = match self.bytes.next() {
                Some(b) => b as u16,
                None    => return, // byte before end of file
            };

            let opcode   = hi << 8 | lo;
            let category = hi >> 4;

            match category {
                0xD => self.disassemble_d(),
                _ => return,
            }
        }
    }

    fn disassemble_d(&self) {
    }
}

