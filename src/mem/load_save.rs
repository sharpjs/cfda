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
use crate::mem::{ByteOrder, Endian};

/// Trait to load values from buffers of type `B` given formats of type `F`.
pub trait Load<B: ?Sized, F=()>: Sized {
    /// Attempts to load a value from the buffer `buf` using the format `fmt`.
    ///
    /// If the buffer length is sufficient to represent the `Self` type, this
    /// method loads a `Self` value from the first elements of the buffer and
    /// returns both the value and a slice of the remaining elements.
    /// Otherwise, this method returns `None`.
    fn load(buf: &B, fmt: F) -> Option<(Self, &B)>;
}

/// Trait to save values into buffers of type `B` given formats of type `F`.
pub trait Save<B: ?Sized, F=()> {
    /// Attempts to save the value in the buffer `buf` using the format `fmt`.
    ///
    /// If the buffer length is sufficient to represent the `Self` type, this
    /// method saves `self` in the first elements of the buffer and returns a
    /// slice of the remaining elements.  Otherwise, this method returns
    /// `None`.
    fn save<'a>(&self, buf: &'a mut B, fmt: F) -> Option<&'a mut B>;
}

impl<T> Load<[u8]> for T where T: Copy {
    #[inline]
    fn load(buf: &[u8], _: ()) -> Option<(T, &[u8])> {
        let size = size_of::<T>();
        if buf.len() >= size {
            let val = unsafe { *(buf.as_ptr() as *const T) };
            Some((val, &buf[size..]))
        } else {
            None
        }
    }
}

impl<T> Save<[u8]> for T where T: Copy {
    #[inline]
    fn save<'a>(&self, buf: &'a mut [u8], _: ()) -> Option<&'a mut [u8]> {
        let size = size_of::<T>();
        if buf.len() >= size {
            unsafe { *(buf.as_ptr() as *mut T) = *self };
            Some(&mut buf[size..])
        } else {
            None
        }
    }
}

impl<T> Load<[u8], ByteOrder> for T where T: Copy + Endian {
    #[inline]
    fn load(buf: &[u8], fmt: ByteOrder) -> Option<(T, &[u8])> {
        let (val, rem) = T::load(buf, ())?;
        Some((T::from_order(fmt, val), rem))
    }
}

impl<T> Save<[u8], ByteOrder> for T where T: Copy + Endian {
    #[inline]
    fn save<'a>(&self, buf: &'a mut [u8], fmt: ByteOrder) -> Option<&'a mut [u8]> {
        self.to_order(fmt).save(buf, ())
    }
}

#[cfg(test)]
mod tests {
    use crate::mem::BE;
    use super::*;

    #[test]
    fn load_raw_some() {
        let buf = [0x12, 0x34, 0x56];
        let (val, rem) = u16::load(&buf[..], ()).unwrap();
        let val = u16::from_be(val);
        assert_eq!(val, 0x1234);
        assert_eq!(rem, [0x56]);
    }

    #[test]
    fn load_endian_some() {
        let buf = [0x12, 0x34, 0x56];
        let (val, rem) = u16::load(&buf[..], BE).unwrap();
        assert_eq!(val, 0x1234);
        assert_eq!(rem, [0x56]);
    }

    #[test]
    fn load_raw_none() {
        let buf = [0x12];
        let ret = u16::load(&buf[..], ());
        assert_eq!(ret, None);
    }

    #[test]
    fn load_endian_none() {
        let buf = [0x12];
        let ret = u16::load(&buf[..], BE);
        assert_eq!(ret, None);
    }

    #[test]
    fn save_raw_some() {
        let mut buf = [0x00, 0x00, 0x56];
        let val = 0x1234_u16.to_be();
        let rem = val.save(&mut buf[..], ()).unwrap();
        assert_eq!(rem, [            0x56]);
        assert_eq!(buf, [0x12, 0x34, 0x56]);
    }

    #[test]
    fn save_endian_some() {
        let mut buf = [0x00, 0x00, 0x56];
        let val = 0x1234_u16;
        let rem = val.save(&mut buf[..], BE).unwrap();
        assert_eq!(rem, [            0x56]);
        assert_eq!(buf, [0x12, 0x34, 0x56]);
    }

    #[test]
    fn save_raw_none() {
        let mut buf = [0x00];
        let val = 0x1234_u16.to_be();
        let ret = val.save(&mut buf[..], ());
        assert_eq!(ret, None);
        assert_eq!(buf, [0x00]);
    }

    #[test]
    fn save_endian_none() {
        let mut buf = [0x00];
        let val = 0x1234_u16;
        let ret = val.save(&mut buf[..], BE);
        assert_eq!(ret, None);
        assert_eq!(buf, [0x00]);
    }
}

