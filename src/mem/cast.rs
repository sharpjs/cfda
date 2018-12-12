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

use std::mem::size_of;

/// Ability to extract arbitrary copyable values from an immutable sequence.
pub trait TakeCast {
    /// Attempts to extract a `T` value from the sequence.
    ///
    /// If the next elements of the sequence are sufficient to represent `T`,
    /// then this method returns the resulting `T` value and the remaining
    /// elements.  Else, this method returns `None`.
    fn take<T: Copy>(&self) -> Option<(T, &Self)>;
}

impl TakeCast for [u8] {
    fn take<T: Copy>(&self) -> Option<(T, &Self)> {
        let size = size_of::<T>();
        if self.len() >= size {
            Some((
                unsafe { *(self.as_ptr() as *const T) },
                &self[size..]
            ))
        } else {
            None
        }
    }
}

/// Ability to read arbitary copyable values from a source.
pub trait ReadCast {
    /// Attempts to read a `T` value from the source.
    ///
    /// If the source has insufficient content to represent `T`, this method
    /// returns `None`, and the source is unchanged.
    fn read<T: Copy>(&mut self) -> Option<T>;
}

impl ReadCast for &'_ [u8] {
    fn read<T: Copy>(&mut self) -> Option<T> {
        let (value, rest) = (*self).take::<T>()?;
        *self = rest;
        Some(value)
    }
}

