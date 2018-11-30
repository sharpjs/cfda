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

use std::ops::{Shr, BitAnd};
use crate::util::Cast;

/// Access to embedded bit fields.
pub trait Field<P, M> {
    /// Gets the value of the bit field at position `pos`, masked with `mask`.
    fn field(self, pos: P, mask: M) -> M;
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

