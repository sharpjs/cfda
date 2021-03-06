// This file is part of cfda, a fun little disassembler project.
// Copyright (C) 2019 Jeffrey Sharp
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

use std::ops::{BitAnd, BitXor, Shl, Shr};
use super::Cast;

/// Trait for reading bit fields within numbers.
pub trait Field<P, V> {
    /// Gets the value of the bit field at position `pos`, masked with `mask`.
    fn field(self, pos: P, mask: V) -> V;
}

/// Trait for writing bit fields within numbers.
pub trait SetField<P, V> {
    /// Sets the value of the bit field at position `pos`, masked with `mask`.
    fn with_field(self, pos: P, mask: V, value: V) -> Self;

    /// Sets the value of the bit field at position `pos`, masked with `mask`.
    fn set_field(&mut self, pos: P, mask: V, value: V);
}

impl<T, P, V> Field<P, V> for T
where
    T: Copy + Shr<P, Output=T> + Cast<V>,
    P: Copy,
    V: Copy + BitAnd<Output=V>
{
    #[inline(always)]
    fn field(self, pos: P, mask: V) -> V {
        (self >> pos).cast() & mask
    }
}

impl<T, P, V> SetField<P, V> for T
where
    T: Copy + Shl<P, Output=T> + BitAnd<Output=T> + BitXor<Output=T>,
    P: Copy,
    V: Copy + Cast<T>
{
    #[inline(always)]
    fn with_field(self, pos: P, mask: V, value: V) -> Self {
        let mask  = mask .cast() << pos;
        let value = value.cast() << pos;
        self ^ ((self ^ value) & mask)
        // = (self & !mask) | (value & mask)
        // from: https://graphics.stanford.edu/~seander/bithacks.html#MaskedMerge 
    }

    #[inline(always)]
    fn set_field(&mut self, pos: P, mask: V, value: V) {
        *self = self.with_field(pos, mask, value)
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
    pub fn with_field() {
        //            FEDCBA(987)6543210
        let word = 0b_000000_000_0000000_u16;
        let mask =        0b_111_u8;
        let val  =        0b_101_u8;
        let ret  = 0b_000000_101_0000000_u16;

        assert_eq!( word.with_field(7, mask, val), ret );
    }

    #[test]
    pub fn set_field() {
        //            FEDCBA(987)6543210
        let mut word = 0b_000000_000_0000000_u16;
        let     mask =        0b_111_u8;
        let     val  =        0b_101_u8;
        let     ret  = 0b_000000_101_0000000_u16;

        word.set_field(7, mask, val);

        assert_eq!( word, ret );
    }
}

