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

pub trait DecodeItem: 'static {
    type Word: Word;

    fn bits(&self) -> Self::Word;
    fn mask(&self) -> Self::Word;
}

#[derive(Clone, Debug)]
pub struct DecodeIndex<T: DecodeItem> {
    node: Node<T>
}

#[derive(Clone, Debug)]
enum Node<T: DecodeItem> {
    Empty,
    Scan(&'static [T]),
    Trie(Trie<T>),
}

#[derive(Clone, Debug)]
struct Trie<T: DecodeItem> {
    mask:  T::Word,
    shift: u8,
    nodes: Box<[Node<T>]>,
}

impl<T> DecodeIndex<T> where T: DecodeItem {
    const MAX_SEL_BITS: u8 = 4;

    pub fn empty() -> Self {
        Self { node: Node::Empty }
    }

    pub fn from(src: &'static [T]) -> Self {
        if src.is_empty() {
            Self::empty()
        } else {
            Self { node: Self::index(src, !T::Word::ZERO) }
        }
    }

    fn index(src: &'static [T], care: T::Word) -> Node<T> {
        debug_assert!(!src.is_empty());

        let (first, rest) = src.split_first().unwrap();

        // Scan 0: Find potential selective bits
        // * Determine which bits are significant (mask) for all items.
        // * Determine which bits differ in value (diff) across all items.

        let (mask, diff) = {
            let mut diff = T::Word::ZERO;       // bits that differ (1=different)
            let mut prev = first.bits();        // bits of previous item
            let mut mask = first.mask() & care; // bits significant to all items

            for item in rest {
                let bits = item.bits();
                diff |= bits ^ prev;
                prev  = bits;
                mask &= item.mask();
            }

            (mask, diff)
        };

        // TODO: Choose most selective consecutive bits
        //
        // ..11.111...1. 

        Node::Scan(&src[..])
    }

    // Find length, position, and mask of longest consecutive ones, up to the
    // given maximum length.
    fn find_mask(mask: T::Word, max_len: u8) -> (T::Word, u8, u8) {
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

        // Shortcut for zeros
        if mask.is_zero() || max_len == 0 {
            return (T::Word::ZERO, 0, 0);
        }

        let mut x;
        let mut mask = mask;
        let mut len  = 0;

        // Reduce consecutive ones regions by their leftmost bits until mask is
        // zero or max length is reached.
        while {
            x    = mask;
            mask = mask & (x >> 1);
            len   += 1;
            mask.is_nonzero() && len < max_len
        } {}

        // Here:
        // * x is mask just before mask became zero or max length was reached.
        // * len is computed.

        // There might be a tie for longest len, or max length might have been
        // reached.  Resolve ambiguity by choosing the leftmost one bit of x
        // as new rightmost bit.
        x = T::Word::ONE << (T::Word::BITS - 1 - x.leading_zeros());

        // Rebuild mask for chosen chosen consecutive ones region by restoring
        // the region's leftmost bits that were reduced earlier.
        mask = x;
        for i in 0..(len - 1) {
            x   <<= 1;
            mask |= x;
        }

        // Compute pos
        let pos = x.trailing_zeros();

        // Result
        (mask, pos, len)
    }
}

