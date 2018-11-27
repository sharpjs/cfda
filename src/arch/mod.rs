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

use crate::util::Word;

/// A computer architecture.
pub trait Arch {
    /// The type of the largest logical memory address within the archicture.
    type Addr: Word;

    /// The type of data referenced by an address within the architecture.
    /// This will be `u8` for byte-addressable architectures.
    type Data: Word;
}

/// The architecture of the Motorola 68000 and its descendants.
#[derive(PartialEq, Eq, Debug)]
pub struct M68k;

impl Arch for M68k {
    type Addr = u32;
    type Data = u8;
}

/// The architecture of the DEC PDP-11 and its descendants.
#[derive(PartialEq, Eq, Debug)]
pub struct Pdp11;

impl Arch for Pdp11 {
    type Addr = u16; // logical; physical could be as much as 22 bits
    type Data = u8;
}

/// The architecture of 64-bit x86 CPUs.
#[derive(PartialEq, Eq, Debug)]
pub struct X86_64;

impl Arch for X86_64 {
    type Addr = u64;
    type Data = u8;
}

