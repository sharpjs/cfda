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

// aliases
//   |
//   V
//   instruction (name, impl)
//   A  |
//   |  *
//   | encoding (bits, mask, args, flags)
//   |  #
//   |  |
// decode_index_entry

use super::word::Word;
use self::Node::*;

pub trait DecodeItem: 'static {
    type Word: Word;
    type Output;

    fn try_decode(&self, word: Self::Word) -> Option<Self::Output>;
}

#[derive(Clone, Debug)]
pub struct DecodeIndex<T: DecodeItem> {
    root: Node<T>
}

#[derive(Clone, Debug)]
pub enum Node<T: DecodeItem> {
    Empty,

    Leaf   (&'static T),

    Scan2  (&'static [Node<T>;  2]),
    Scan3  (&'static [Node<T>;  3]),
    Scan4  (&'static [Node<T>;  4]),

    Trie2  (&'static [Node<T>;  2], u8),
    Trie4  (&'static [Node<T>;  4], u8),
    Trie8  (&'static [Node<T>;  8], u8),
    Trie16 (&'static [Node<T>; 16], u8),

    Chain  (&'static Node<T>),
}

enum Decoded<T> where T: DecodeItem {
    Item(&'static T),
    More(&'static DecodeIndex<T>),
}

enum NodeResult<T> where T: DecodeItem {
    Fail,                       // 0 items; lookup fails
    Succeed(&'static T),        // 1 item;  lookup succeeds
    Examine(&'static Node<T>),  // ? items; examine subnode using same word
    Advance(&'static Node<T>),  // ? items; examine subnode using next word
}

impl<T> DecodeIndex<T> where T: DecodeItem {
    fn get(&self, word: T::Word) -> Option<T::Output> {
        self.root.get(word)
    }

    fn get2<I>(&self, words: &mut I) -> Option<&'static T> where
        I: Iterator<Item=T::Word>,
    {
        let mut node = &self.root;

        loop {
            let word = match words.next() {
                None    => return None,
                Some(w) => w,
            };

            loop {
                match node.lookup(word) {
                    NodeResult::Fail          => return None,
                    NodeResult::Succeed(item) => return Some(item),
                    NodeResult::Examine(next) => { node = next        },
                    NodeResult::Advance(next) => { node = next; break },
                }
            }
        }
    }
}

impl<T> Node<T> where T: DecodeItem {
    fn lookup(&self, word: T::Word) -> NodeResult<T> {
        panic!()
    }

    fn get(&self, word: T::Word) -> Option<T::Output> {
        match *self {
            Empty                 => None,
            Leaf   (item)         => item.try_decode(word),
            Scan2  (nodes)        => Self::scan(nodes, word),
            Scan3  (nodes)        => Self::scan(nodes, word),
            Scan4  (nodes)        => Self::scan(nodes, word),
            Trie2  (nodes, shift) => Self::seek(nodes, word, shift, 0b0001),
            Trie4  (nodes, shift) => Self::seek(nodes, word, shift, 0b0011),
            Trie8  (nodes, shift) => Self::seek(nodes, word, shift, 0b0111),
            Trie16 (nodes, shift) => Self::seek(nodes, word, shift, 0b1111),
            Chain  (node)         => panic!(),
        }
    }

    fn scan(nodes: &[Self], word: T::Word) -> Option<T::Output> {
        nodes.iter().find_map(|n| n.get(word))
    }

    fn seek(nodes: &[Self], word: T::Word, shift: u8, mask: u8) -> Option<T::Output> {
        let mask = T::Word::from(mask);
        let bits = word >> shift & mask;
        nodes[bits.to_usize()].get(word)
    }
}

//#[cfg(test)]
//mod tests {
//    use super::*;
//    use std::mem::size_of;
//
//    #[test]
//    fn size_of_node() {
//        assert_eq!( size_of::<Node<Ins>>(), 16 );
//    }
//
//    #[test]
//    fn size_of_option_decoded() {
//        assert_eq!( size_of::<Option<Decoded<Ins>>>(), 16 );
//    }
//}

