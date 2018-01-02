// This file is part of cfda, a fun little disassembler project.
// Copyright (C) 2017 Jeffrey Sharp
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

// TODO: Support BE and LE byte order

use std::slice;

/// An assembly character literal.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Char {
    pub value:    char,
    pub encoding: Encoding,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Encoding {
    Utf8,
    Utf16,
    Utf32
}

impl Char {
    pub fn new(value: char, encoding: Encoding) -> Self {
        Self { value, encoding }
    }

    pub fn encode(&self, dst: &mut [u8]) -> usize {
        match self.encoding {
            Encoding::Utf8 => {
                self.value.encode_utf8(dst).len()
            },
            Encoding::Utf16 => {
                let mut buf = [0u16; 2];
                let values = self.value.encode_utf16(&mut buf);
                let bytes = unsafe {
                    slice::from_raw_parts(
                        values.as_ptr() as *const u8,
                        values.len() * 2
                    )
                };
                dst[..bytes.len()].copy_from_slice(bytes);
                bytes.len()
            },
            Encoding::Utf32 => {
                const SIZE: usize = 4; // size_of::<u32>()
                let value = self.value as u32;
                let bytes = unsafe {
                    slice::from_raw_parts(
                        &value as *const u32 as *const u8,
                        SIZE
                    )
                };
                dst[..4].copy_from_slice(bytes);
                SIZE
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn encode_utf8_simple() {
        let mut b = [0u8; 4];
        let     c = Char { value: 'a', encoding: Encoding::Utf8 };
        let     n = c.encode(&mut b);

        assert_eq!(n, 1);
        assert_eq!(b, [0x61, 0, 0, 0]);
    }

    #[test]
    pub fn encode_utf8_complex() {
        let mut b = [0u8; 4];
        let     c = Char { value: '\u{211D9}', encoding: Encoding::Utf8 };
        let     n = c.encode(&mut b);

        assert_eq!(n, 4);
        assert_eq!(b, [0xF0, 0xA1, 0x87, 0x99]);
    }

    #[test]
    pub fn encode_utf16_simple() {
        let mut b = [0u8; 4];
        let     c = Char { value: 'a', encoding: Encoding::Utf16 };
        let     n = c.encode(&mut b);

        assert_eq!(n, 2);
        assert_eq!(b, [0x61, 0, 0, 0]);
    }

    #[test]
    pub fn encode_utf16_complex() {
        let mut b = [0u8; 4];
        let     c = Char { value: '\u{211D9}', encoding: Encoding::Utf16 };
        let     n = c.encode(&mut b);

        assert_eq!(n, 4);
        assert_eq!(b, [0x44, 0xD8, 0xD9, 0xDD]);
    }

    #[test]
    pub fn encode_utf32_simple() {
        let mut b = [0u8; 4];
        let     c = Char { value: 'a', encoding: Encoding::Utf32 };
        let     n = c.encode(&mut b);

        assert_eq!(n, 4);
        assert_eq!(b, [0x61, 0, 0, 0]);
    }

    #[test]
    pub fn encode_utf32_complex() {
        let mut b = [0u8; 4];
        let     c = Char { value: '\u{211D9}', encoding: Encoding::Utf32 };
        let     n = c.encode(&mut b);

        assert_eq!(n, 4);
        assert_eq!(b, [0xD9, 0x11, 0x02, 0x00]);
    }
}

