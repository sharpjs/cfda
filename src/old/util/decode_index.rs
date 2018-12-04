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

use super::word::Word;
//use self::DecodeIndex::*;

// Decoding Data Structures
// ------------------------
//
// aliases
//   |
//   V
//   instruction (name, impl)
//   A  |
//   |  *
//   | encoding (bits, mask, args, flags)
//   |  #
//   |  |
// decode_index_node
//

/// Trait for items in a `DecodeIndex`.
pub trait DecodeItem: 'static {

    /// The type of word to be decoded.
    type Word: Word;

    /// Type output type of an index lookup.
    type Output;

    /// Attempts to match the item with the given word.
    fn try_decode(&self, word: Self::Word) -> Option<Self::Output>;
}

/// A data structure for efficient decoding of instructions.
///
/// A `DecodeIndex` is a recursive, tree-shaped structure.  An index consists
/// of a single node, which can be leaf or non-leaf.  A leaf node either is
/// empty or references a single item.  A non-leaf node references a set of
/// subnodes, each of which is an independent index.  A non-leaf node also
/// indicates how the appropriate subnode is chosen during an index lookup.
///
#[derive(Clone, Debug)]
pub enum DecodeIndex<T: DecodeItem> {

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

    // === CHAIN ===

    /// A single subnode.  A lookup against the subnode will use the next word
    /// from input.
    Chain(&'static DecodeIndex<T>),
}

pub enum Decoded<T> where T: DecodeItem {
    Item(&'static T),
    More(&'static DecodeIndex<T>),
}

//enum DecodeIndexResult<T> where T: DecodeItem {
//    Fail,                               // 0 items; lookup fails
//    Succeed(&'static T),                // 1 item;  lookup succeeds
//    Examine(&'static DecodeIndex<T>),   // ? items; examine subnode using same word
//    Advance(&'static DecodeIndex<T>),   // ? items; examine subnode using next word
//}

impl<T> DecodeIndex<T> where T: DecodeItem {
    fn decode<W>(&self, words: W) -> Option<T::Output> {
        panic!()
    }

    //fn get(&self, word: T::Word) -> Option<T::Output> {
    //    self.get_(word)
    //}

    //fn get2<I>(&self, words: &mut I) -> Option<&'static T> where
    //    I: Iterator<Item=T::Word>,
    //{
    //    let mut node = self;
    //
    //    loop {
    //        let word = match words.next() {
    //            None    => return None,
    //            Some(w) => w,
    //        };
    //
    //        loop {
    //            match node.lookup(word) {
    //                DecodeIndexResult::Fail          => return None,
    //                DecodeIndexResult::Succeed(item) => return Some(item),
    //                DecodeIndexResult::Examine(next) => { node = next        },
    //                DecodeIndexResult::Advance(next) => { node = next; break },
    //            }
    //        }
    //    }
    //}

    //fn lookup(&self, word: T::Word) -> DecodeIndexResult<T> {
    //    panic!()
    //}

    //fn get_(&self, word: T::Word) -> Option<T::Output> {
    //    match *self {
    //        Empty                 => None,
    //        Leaf   (item)         => item.try_decode(word),
    //        Scan2  (nodes)        => Self::scan(nodes, word),
    //        Scan3  (nodes)        => Self::scan(nodes, word),
    //        Scan4  (nodes)        => Self::scan(nodes, word),
    //        Trie2  (nodes, shift) => Self::seek(nodes, word, shift, 0b0001),
    //        Trie4  (nodes, shift) => Self::seek(nodes, word, shift, 0b0011),
    //        Trie8  (nodes, shift) => Self::seek(nodes, word, shift, 0b0111),
    //        Trie16 (nodes, shift) => Self::seek(nodes, word, shift, 0b1111),
    //        Chain  (node)         => panic!(),
    //    }
    //}

    //fn scan(nodes: &[Self], word: T::Word) -> Option<T::Output> {
    //    nodes.iter().find_map(|n| n.get(word))
    //}

    //fn seek(nodes: &[Self], word: T::Word, shift: u8, mask: u8) -> Option<T::Output> {
    //    let mask = T::Word::from(mask);
    //    let bits = word >> shift & mask;
    //    nodes[bits.to_usize()].get(word)
    //}
}

//#[cfg(test)]
//mod tests {
//    use super::*;
//    use std::mem::size_of;
//
//    #[test]
//    fn size_of_node() {
//        assert_eq!( size_of::<DecodeIndex<Ins>>(), 16 );
//    }
//
//    #[test]
//    fn size_of_option_decoded() {
//        assert_eq!( size_of::<Option<Decoded<Ins>>>(), 16 );
//    }
//}

