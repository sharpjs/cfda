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

use super::Ident;

/// A slot containing a value and/or an identifier that resolves to that value.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Slot<V> {
    /// Unresolved identifier.
    Ident(Ident),

    /// Identifier resolved to a value.
    Resolved(Ident, V),

    /// Value resolved without an identifier.
    Value(V),
}

impl<V> Slot<V> {
    /// Returns the identifier reference, if any, in the slot.
    #[inline]
    pub fn ident(&self) -> Option<Ident> {
        match *self {
            Slot::Ident    (i   ) => Some(i),
            Slot::Resolved (i, _) => Some(i),
            Slot::Value    (   _) => None,
        }
    }

    /// Returns a reference to the resolved value, if any, in the slot.
    #[inline]
    pub fn value(&self) -> Option<&V> {
        match *self {
            Slot::Ident    (_       ) => None,
            Slot::Resolved (_, ref v) => Some(v),
            Slot::Value    (   ref v) => Some(v),
        }
    }

    /// Returns a copy of the resolved value, if any, in the slot.
    #[inline]
    pub fn value_copy(&self) -> Option<V> where V: Copy {
        match *self {
            Slot::Ident    (_   ) => None,
            Slot::Resolved (_, v) => Some(v),
            Slot::Value    (   v) => Some(v),
        }
    }
}

