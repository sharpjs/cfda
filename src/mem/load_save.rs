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

/// Ability to load and save arbitrary copyable values in a buffer.
pub trait LoadSaveRaw {
    /// Attempts to load a `T` value from the buffer.
    ///
    /// If the buffer length is sufficient to represent a `T` value, this
    /// method loads a `T` value from the first elements of the buffer and
    /// returns both the value and a slice of the remaining elements.
    /// Otherwise, this method returns `None`.
    fn load<T: Copy>(&self) -> Option<(T, &Self)>;

    /// Attempts to save the given `T` value into the buffer.
    ///
    /// If the buffer length is sufficient to represent a `T` value, this
    /// method saves the given `T` value in the first elements of the buffer
    /// and returns a slice of the remaining elements.  Otherwise, this method
    /// returns `None`.
    fn save<T: Copy>(&mut self, val: T) -> Option<&mut Self>;
}

impl LoadSaveRaw for [u8] {
    #[inline]
    fn load<T: Copy>(&self) -> Option<(T, &[u8])> {
        let size = size_of::<T>();
        if self.len() >= size {
            let val = unsafe { *(self.as_ptr() as *const T) };
            Some((val, &self[size..]))
        } else {
            None
        }
    }

    #[inline]
    fn save<T: Copy>(&mut self, val: T) -> Option<&mut [u8]> {
        let size = size_of::<T>();
        if self.len() >= size {
            unsafe { *(self.as_ptr() as *mut T) = val };
            Some(&mut self[size..])
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_some() {
        let buf = [0x12, 0x34, 0x56];
        let (val, rem) = buf.load().unwrap();
        let val = u16::from_be(val);
        assert_eq!(val, 0x1234);
        assert_eq!(rem, [0x56]);
    }

    #[test]
    fn load_none() {
        let buf = [0x12];
        let ret = buf.load::<u16>();
        assert_eq!(ret, None);
    }

    #[test]
    fn save_some() {
        let mut buf = [0x00, 0x00, 0x56];
        let val = 0x1234_u16.to_be();
        let rem = buf.save(val).unwrap();
        assert_eq!(rem, [            0x56]);
        assert_eq!(buf, [0x12, 0x34, 0x56]);
    }

    #[test]
    fn save_none() {
        let mut buf = [0x00];
        let val = 0x1234_u16.to_be();
        let ret = buf.save(val);
        assert_eq!(ret, None);
        assert_eq!(buf, [0x00]);
    }
}

