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

/// Trait for decoding machine code.
///
/// Type `U` is the unit of storage of machine code.  This type typically is
/// `u8` for byte-oriented architectures and the word type for word-oriented
/// architectures.
///
/// Type `C` is the contextual data readable during decoding.  This can be
/// anything from relatively static configuration data to ephemeral partial
/// decode state during a nested decode.
pub trait Decode<U, C=()> {
    /// The result of successful decoding.
    type Output;

    /// Attempts to decode the machine code in `buf`, given the context `ctx`.
    ///
    /// If decoding is successful, this method returns a tuple consisting of
    /// the decoded result and the remaining machine code, if any.  If decoding
    /// was not successful, this method returns `None`.
    fn decode<'a>(&self, buf: &'a [U], ctx: &C) -> Option<(Self::Output, &'a [U])>;
}
