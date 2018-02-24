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

use std::fmt::Debug;
use std::ops::*;

use num_traits::{PrimInt, Zero, One, ToPrimitive};

pub trait BitTrieItem {
    type Key: PrimInt + Debug;

    fn bits(&self) -> Self::Key;
    fn mask(&self) -> Self::Key;
}

#[derive(Clone, Debug)]
pub struct BitTrie<T: BitTrieItem> {
    nodes: Vec<Node<T>>
}

#[derive(Clone, Debug)]
struct Node<T: BitTrieItem> {
    bits:    T::Key,
    mask:    T::Key,
    content: Content<T>,
}

#[derive(Clone, Debug)]
enum Content<T: BitTrieItem> {
    Leaf(T),
    Trie(BitTrie<T>),
}

impl<T> BitTrie<T> where T: BitTrieItem {
    pub fn empty() -> Self {
        Self { nodes: vec![] }
    }

    pub fn from(src: &[&T]) -> Self where T: Clone /*TODO*/ {
        if src.is_empty() {
            Self::empty()
        } else {
            Self::from_(src, !T::Key::zero())
        }
    }

    fn from_(src: &[&T], care: T::Key) -> Self where T: Clone /*TODO*/ {
        debug_assert!(!src.is_empty());

        // Scan 0:
        // * Determine which bits are significant (mask) for all items.
        // * Determine which bits have the same value for all items.

        let (first, rest) = src.split_first().unwrap();

        let mut diff = T::Key::zero();
        let mut prev = first.bits();
        let mut mask = first.mask() & care;

        for x in rest {
            let bits = x.bits();
            diff = diff | bits ^ prev;
            prev = bits;

            mask = mask & x.mask();
        }

        // Select a range of selective bits: significant and differing.

        let (sel, pos, len) = Self::find_mask(mask & diff, 6);
        let care = care & !sel; // for subnodes
        // what if len = 0?

        // Distribute items into bins by their selective bits.

        let mut bins = vec![Vec::new(); 1 << len];

        for &x in src {
            let i = ((x.bits() & sel) >> (pos as usize)).to_usize().unwrap();
            bins[i].push(x);
        }

        // Convert bins into nodes

        Self {
            nodes: bins
                .into_iter()
                .map(|items| Node {
                    bits:    T::Key::zero(), // TODO
                    mask:    T::Key::zero(), // TODO
                    content: Content::Trie(Self::from_(&items[..], care)),
                })
                .collect()
        }
    }

    fn find_mask(mask: T::Key, max_len: u8) -> (T::Key, u8, u8) {
        // Find length, position, and mask of longest consecutive ones.
        // Length:
        //
        // ..11.111...1. start  len=0
        // ...11.111...1 >> 1
        // ...1..11..... &      len=1  !=0
        // ....1..11.... >> 1
        // .......1..... &      len=2  !=0
        // ........1.... >> 1
        // ............. &      len=3  ==0  *stop*
        //
        // Position:
        //
        // ........1.... just before last & above
        // .......1..... << 1
        //         ^^^^^ trailing_zeros => start=5
        //
        // Mask:
        //
        // ........1.... just before last & above
        // .......1..... << 1         i=2
        // ......11..... << 1 then |  i=1
        // .....111..... << 1 then |  i=0 *stop*

        if mask.is_zero() { return (T::Key::zero(), 0, 0) };

        let mut val = mask;
        let mut shr = val;
        let mut len = 0;

        while {
            shr  = shr >> 1;
            val  = val & shr;
            len += 1;
            !val.is_zero() && len < max_len
        } {}

        // len is known

        shr = if shr.is_zero() {
            T::Key::one()
        } else {
            shr << 1
        };

        let pos = shr.trailing_zeros() as u8;
        
        // pos is known

        val = val | shr;
        let mut i = len - 1;

        while i > 0 {
            shr = shr << 1;
            val = val | shr;
            i -= 1;
        }

        // mask is known

        (val, pos, len)
    }
}

