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
use std::ops::*;

pub trait Word: Copy + Eq + Ord + Debug
    + Not    <    Output=Self>
    + Shl    <u8, Output=Self> + ShlAssign    <u8  >
    + Shr    <u8, Output=Self> + ShrAssign    <u8  >
    + BitAnd <    Output=Self> + BitAndAssign <Self>
    + BitXor <    Output=Self> + BitXorAssign <Self>
    + BitOr  <    Output=Self> + BitOrAssign  <Self>
{
    const BITS: u8;
    const ZERO: Self;
    const ONE:  Self;

    fn to_usize(self) -> usize;

    fn leading_zeros(self) -> u8;

    fn trailing_zeros(self) -> u8;

    #[inline(always)]
    fn is_zero(self) -> bool {
        self == Self::ZERO
    }

    #[inline(always)]
    fn is_nonzero(self) -> bool {
        self != Self::ZERO
    }
}

impl Word for u32 {
    const BITS: u8   = 32;
    const ZERO: Self =  0;
    const ONE:  Self =  1;

    #[inline(always)]
    fn to_usize(self) -> usize {
        self as usize
    }

    #[inline(always)]
    fn leading_zeros(self) -> u8 {
        self.leading_zeros() as u8
    }

    #[inline(always)]
    fn trailing_zeros(self) -> u8 {
        self.trailing_zeros() as u8
    }
}

