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

use std::fmt::Debug;
use std::hash::Hash;
use std::ops::*;
use crate::util::Cast;

/// Trait for machine word types.
pub trait Word: Copy + Eq + Ord + Hash + Debug + 'static
    + From    <u8>
    + Cast    <u64>
//  + Neg     <Output=Self>
    + Not     <Output=Self>
    + Mul     <Output=Self>     + MulAssign    <Self>
    + Div     <Output=Self>     + DivAssign    <Self>
    + Rem     <Output=Self>     + RemAssign    <Self>
    + Add     <Output=Self>     + AddAssign    <Self>
    + Sub     <Output=Self>     + SubAssign    <Self>
    + Shl     <u8, Output=Self> + ShlAssign    <u8>
    + Shr     <u8, Output=Self> + ShrAssign    <u8>
    + BitAnd  <Output=Self>     + BitAndAssign <Self>
    + BitXor  <Output=Self>     + BitXorAssign <Self>
    + BitOr   <Output=Self>     + BitOrAssign  <Self>
{
    const BITS: u8;
    const ZERO: Self;
    const ONE:  Self;
    const MAX:  Self;

    // Using u64 for LMA, so this is required.
    fn to_u64(self) -> u64 { self.cast() }

    fn checked_add(self, Self) -> Option<Self>;

    fn checked_sub(self, Self) -> Option<Self>;
}

impl Word for u8 {
    const BITS: u8   =    8;
    const ZERO: Self =    0;
    const ONE:  Self =    1;
    const MAX:  Self = 0xFF;

    //#[inline(always)]
    //fn to_u64(self) -> u64 {
    //    self as u64
    //}

    #[inline(always)]
    fn checked_add(self, other: Self) -> Option<Self> {
        self.checked_add(other)
    }

    #[inline(always)]
    fn checked_sub(self, other: Self) -> Option<Self> {
        self.checked_sub(other)
    }
}

impl Word for u16 {
    const BITS: u8   =     16;
    const ZERO: Self =      0;
    const ONE:  Self =      1;
    const MAX:  Self = 0xFFFF;

    #[inline(always)]
    fn checked_add(self, other: Self) -> Option<Self> {
        self.checked_add(other)
    }

    #[inline(always)]
    fn checked_sub(self, other: Self) -> Option<Self> {
        self.checked_sub(other)
    }
}

impl Word for u32 {
    const BITS: u8   =          32;
    const ZERO: Self =           0;
    const ONE:  Self =           1;
    const MAX:  Self = 0xFFFF_FFFF;

    #[inline(always)]
    fn checked_add(self, other: Self) -> Option<Self> {
        self.checked_add(other)
    }

    #[inline(always)]
    fn checked_sub(self, other: Self) -> Option<Self> {
        self.checked_sub(other)
    }
}

impl Word for u64 {
    const BITS: u8   =                    64;
    const ZERO: Self =                     0;
    const ONE:  Self =                     1;
    const MAX:  Self = 0xFFFF_FFFF_FFFF_FFFF;

    #[inline(always)]
    fn checked_add(self, other: Self) -> Option<Self> {
        self.checked_add(other)
    }

    #[inline(always)]
    fn checked_sub(self, other: Self) -> Option<Self> {
        self.checked_sub(other)
    }
}

