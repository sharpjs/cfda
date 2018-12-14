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

use crate::mem::cast::ReadCast;
use crate::mem::{ByteOrdered, Endian};

macro_rules! fn_read {
    { $( $n:ident : $t:ident ; )* } => {$(
        fn $n(&mut self) -> Option<$t> {
            let value = self.bytes().read::<$t>()?;
            let order = self.byte_order();
            Some($t::from_order(order, value))
        }
    )*}
}

pub trait ReadPrimitive : ByteOrdered  {

    fn bytes(&mut self) -> &mut &[u8];

    fn_read! {
        //read_u8:  u8;  read_i8:  i8;
        read_u16: u16; read_i16: i16;
        read_u32: u32; read_i32: i32;
    }
}

