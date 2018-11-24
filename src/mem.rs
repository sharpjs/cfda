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

use crate::arch::Arch;
use crate::util::Word;

/// A memory region.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Region<A: Arch> {
    /// Load Memory Address (LMA).  LMA + relocation = VMA.
    pub lma: u64,

    /// Virtual Memory Address (VMA).  LMA + relocation = VMA.
    pub vma: A::Addr,

    /// Length in bytes.
    pub len: A::Addr,
}
}

impl<A> Region<A> where A: Arch {
    /// Gets the relocation in bytes.  LMA + relocation = VMA.
    pub fn reloc(&self) -> u64 {
        self.vma.to_u64().wrapping_sub(self.lma)
    }

    /// Gets the ending Load Memory Address.
    pub fn end_lma(&self) -> u64 {
        self.lma + self.len.to_u64()
    }

    /// Gets the ending Virtual Memory Address.
    pub fn end_vma(&self) -> A::Addr {
        self.vma + self.len
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn reloc_positive() {
        let region = Region { lma: 0x2000, vma: 0x3000, len: 0x0100 };
        let reloc  = region.reloc();
        assert_eq!(reloc, 0x1000);
    }

    #[test]
    pub fn reloc_negative() {
        let region = Region { lma: 0x2000, vma: 0x1000, len: 0x0100 };
        let reloc  = region.reloc();
        assert_eq!(reloc, 0x1000u32.wrapping_neg());
    }

    #[test]
    pub fn end_lma() {
        let region = Region { lma: 0x2000, vma: 0x3000, len: 0x0100 };
        let addr   = region.end_lma();
        assert_eq!(addr, 0x2100);
    }

    #[test]
    pub fn end_vma() {
        let region = Region { lma: 0x2000, vma: 0x3000, len: 0x0100 };
        let addr   = region.end_vma();
        assert_eq!(addr, 0x3100);
    }
    
    #[test]
    #[should_panic]
    pub fn end_lma_out_of_range() {
        let region = Region { lma: 0x2000, vma: 0x3000, len: u32::max_value() };
        let addr   = region.end_lma();
    }
    
    #[test]
    #[should_panic]
    pub fn end_vma_out_of_range() {
        let region = Region { lma: 0x2000, vma: 0x3000, len: u32::max_value() };
        let addr   = region.end_vma();
    }
}

