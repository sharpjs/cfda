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

/// Simple, reinterpreting conversion to another, usually primitive, type.
/// Conversion to narrower types truncates, while conversion to wider types
/// extends.
pub trait Cast<T> {
    /// Converts the value to type `T`.  If `T` is a narrower type, the
    /// conversion truncates.  IF `T` is a wider type, the conversion sign- or
    /// zero-extends, depending on the signedness of the source type.
    fn cast(self) -> T;
}

macro_rules! impl_cast {
    { $( $src:ty => $($dst:ty),* ; )* } => { $( $(
        impl Cast<$dst> for $src {
            #[inline(always)]
            fn cast(self) -> $dst { self as $dst }
        }
    )* )* }
}

impl_cast! {
      u8 => u8, u16, u32, u64, u128;
     u16 => u8, u16, u32, u64, u128;
     u32 => u8, u16, u32, u64, u128;
     u64 => u8, u16, u32, u64, u128;
    u128 => u8, u16, u32, u64, u128;

      i8 => i8, i16, i32, i64, i128;
     i16 => i8, i16, i32, i64, i128;
     i32 => i8, i16, i32, i64, i128;
     i64 => i8, i16, i32, i64, i128;
    i128 => i8, i16, i32, i64, i128;
}

