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

// TODO: Make note about using u32 for op + ext word.

#[derive(Clone, Copy, Debug)]
pub struct DecodeContext<I> {
    bits: u32,
    more: I
}

impl<I> DecodeContext<I> where I: Iterator<Item=u16> {
    pub fn new(mut words: I) -> Option<Self> {
        let word = match words.next() {
            Some(w) => w as u32,
            None    => return None,
        };
        Some(Self { bits: word, more: words })
    }

    pub fn advance(&mut self) -> bool {
        let word = match self.more.next() {
            Some(w) => w as u32,
            None    => return false,
        };
        self.bits |= word << 16;
        true
    }

    #[inline(always)]
    pub fn bits(&self) -> u32 { self.bits }

    // check with mask
    //
    // extract bit field
}

