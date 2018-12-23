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

use crate::num::Field;

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

/// Trait to obtain an opword from a decoding context.
pub trait Opword {
    /// The opword type.
    type Opword;

    /// Returns the opword.
    fn opword(&self) -> Self::Opword;
}

/// A data structure for efficient decoding of instructions.
///
/// A `DecodeIndex` is a recursive, tree-shaped structure.  An index consists
/// of a single node, which can be leaf or non-leaf.  A leaf node either is
/// empty or references a single item.  A non-leaf node references a set of
/// subnodes, each of which is an independent index.  A non-leaf node also
/// indicates how the appropriate subnode is chosen during an index lookup.
#[derive(Debug)]
pub enum DecodeIndex<T: 'static> {

    // === LEAF ===

    /// An empty leaf node.
    Empty,

    /// A leaf node with one item.
    Leaf(&'static T),

    // === SCAN ===

    /// Two subnodes, selected by sequential scan.
    Scan2(&'static [DecodeIndex<T>; 2]),

    /// Three subnodes, selected by sequential scan.
    Scan3(&'static [DecodeIndex<T>; 3]),

    /// Four subnodes, selected by sequential scan.
    Scan4(&'static [DecodeIndex<T>; 4]),

    // === TRIE ===

    /// 2 subnodes, selected by the 1-bit field at the given position.
    Trie2(&'static [DecodeIndex<T>; 2], u8),

    /// 4 subnodes, selected by the 2-bit field whose least significant bit is
    /// at the given position.
    Trie4(&'static [DecodeIndex<T>; 4], u8),

    /// 8 subnodes, selected by the 3-bit field whose least significant bit is
    /// at the given position.
    Trie8(&'static [DecodeIndex<T>; 8], u8),

    /// 16 subnodes, selected by the 4-bit field whose least significant bit is
    /// at the given position.
    Trie16(&'static [DecodeIndex<T>; 16], u8),
}

impl<T, U, C> Decode<U, C> for DecodeIndex<T>
where
    T: Decode<U, C>,
    C: Opword,
    C::Opword: Field<u8, u8>
{
    type Output = T::Output;

    fn decode<'a>(&self, buf: &'a [U], ctx: &C) -> Option<(T::Output, &'a [U])> {
        use self::DecodeIndex::*;

        enum Plan<T: 'static> {
            Scan(&'static [DecodeIndex<T>]),
            Seek(&'static [DecodeIndex<T>], u8, u8),
        }

        let plan = match *self {
            Empty               => return None,
            Leaf   (item)       => return item.decode(buf, ctx),
            Scan2  (nodes)      => Plan::Scan(nodes),
            Scan3  (nodes)      => Plan::Scan(nodes),
            Scan4  (nodes)      => Plan::Scan(nodes),
            Trie2  (nodes, pos) => Plan::Seek(nodes, pos, 0b0001),
            Trie4  (nodes, pos) => Plan::Seek(nodes, pos, 0b0011),
            Trie8  (nodes, pos) => Plan::Seek(nodes, pos, 0b0111),
            Trie16 (nodes, pos) => Plan::Seek(nodes, pos, 0b1111),
        };

        match plan {
            Plan::Scan(nodes) => {
                nodes.iter().find_map(|n| n.decode(buf, ctx))
            },
            Plan::Seek(nodes, pos, mask) => {
                let val = ctx.opword().field(pos, mask);
                nodes[val as usize].decode(buf, ctx)
            },
        }
    }
}

