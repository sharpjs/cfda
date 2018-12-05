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

use std::ops::{Shl, Shr, BitAnd, BitOr};
use super::Cast;

/// Trait for reading bit fields within numbers.
pub trait Field<P, M> {
    /// Gets the value of the bit field at position `pos`, masked with `mask`.
    fn field(self, pos: P, mask: M) -> M;
}

/// Trait for writing bit fields within numbers.
pub trait SetField<P, M> {
    /// Sets the value of the bit field at position `pos`, masked with `mask`.
    fn set_field(self, pos: P, mask: M, value: M) -> Self;
}

impl<T, P, M> Field<P, M> for T
where
    T: Shr<P, Output=T> + Cast<M>,
    M: BitAnd<Output=M>
{
    #[inline(always)]
    fn field(self, pos: P, mask: M) -> M {
        (self >> pos).cast() & mask
    }
}

impl<T, P, M> SetField<P, M> for T
where
    T: Shl<P, Output=T> + BitOr<Output=T>,
    M: BitAnd<Output=M> + Cast<T>
{
    #[inline(always)]
    fn set_field(self, pos: P, mask: M, value: M) -> Self {
        (value & mask).cast() << pos | self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn field() {
        //            FEDCBA(987)6543210
        let word = 0b_111111_101_1111111_u16;
        let mask =        0b_111_u8;
        let val  =        0b_101_u8;

        assert_eq!( word.field(7, mask), val );
    }

    #[test]
    pub fn set_field() {
        //            FEDCBA(987)6543210
        let word = 0b_000000_000_0000000_u16;
        let mask =        0b_111_u8;
        let val  =        0b_101_u8;
        let ret  = 0b_000000_101_0000000_u16;

        assert_eq!( word.set_field(7, mask, val), ret );
    }
}

