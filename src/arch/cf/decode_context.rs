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

use crate::mem::TakeCast;

#[derive(Clone, Copy, Debug)]
pub struct DecodeContext<'a> {
    bits: u32,
    rest: &'a [u8],
}

impl<'a> DecodeContext<'a> {
    pub fn new(bytes: &'a [u8]) -> Option<Self> {
        let (word, rest) = bytes.take::<u16>()?;
        Some(Self { bits: word as u32, rest })
    }

    pub fn next(&self) -> Option<Self> {
        let (word, rest) = self.rest.take::<u16>()?;
        Some(Self { bits: self.bits | (word as u32) << 16, rest })
    }

    #[inline(always)]
    pub fn bits(&self) -> u32 { self.bits }

    #[inline(always)]
    pub fn rest(&self) -> &'a [u8] { self.rest }

    // check with mask
    // extract bit field
}

