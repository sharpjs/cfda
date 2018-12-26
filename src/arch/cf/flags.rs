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

/// Indicates hardware support and arity of ColdFire instructions.
///
/// When part of an instruction encoding, the flags indicate both the arity of
/// the instruction and which hardware feature(s) are each independently
/// sufficient to support the instruction encoding.
///
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct CfFlags (u16);

const ARITY_MASK:       u16 = 0b0000_0000_0000_0111;
const FEATURES_MASK:    u16 = 0b1111_1111_1111_1000;

// Features
pub const ISA_A:        u16 = 1 <<  3; // ColdFire ISA_A
pub const ISA_A2:       u16 = 1 <<  4; // ColdFire ISA_A+
pub const ISA_B:        u16 = 1 <<  5; // ColdFire ISA_B
pub const ISA_C:        u16 = 1 <<  6; // ColdFire ISA_C
pub const HWDIV:        u16 = 1 <<  7; // ColdFire hardware divide
pub const FPU:          u16 = 1 <<  8; // ColdFire FPU
pub const MAC:          u16 = 1 <<  9; // ColdFire MAC
pub const EMAC:         u16 = 1 << 10; // ColdFire EMAC
pub const EMAC_B:       u16 = 1 << 11; // ColdFire EMAC_B
pub const MMU:          u16 = 1 << 12; // ColdFire MMU
pub const USP:          u16 = 1 << 13; // ColdFire user stack pointer

// Composite features
pub const ISA_A_UP:     u16 = ISA_A | ISA_A2 | ISA_B | ISA_C;
pub const ISA_A2_UP:    u16 =         ISA_A2 | ISA_B | ISA_C;
pub const ISA_B_UP:     u16 =                  ISA_B | ISA_C;

impl CfFlags {
    #[inline]
    const fn new(arity: usize, features: u16) -> CfFlags {
        CfFlags(
            (arity as u16 &    ARITY_MASK) |
            (features     & FEATURES_MASK)
        )
    }

    #[inline]
    const fn arity(self) -> usize {
        (self.0 & ARITY_MASK) as usize
    }

    #[inline]
    const fn features(self) -> u16 {
        self.0 & FEATURES_MASK
    }

    #[inline]
    const fn has_any(self, features: u16) -> bool {
        self.features() & features != 0
    }
}

