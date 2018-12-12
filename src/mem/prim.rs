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
use crate::num::{ByteOrdered, Endian};

pub trait ReadPrimitive : ByteOrdered  {

    fn bytes(&mut self) -> &mut &[u8];

    fn read_u16(&mut self) -> Option<u16> {
        let value = self.bytes().read::<u16>()?;
        let order = self.byte_order();
        Some(u16::from_order(order, value))
    }
}

