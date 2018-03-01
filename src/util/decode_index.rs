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

use self::Seg::*;
use super::word::Word;

pub trait DecodeItem: 'static {
    type Word: Word;

    fn bits(&self) -> Self::Word;
    fn mask(&self) -> Self::Word;
}

#[derive(Clone, Debug)]
pub struct DecodeIndex<T: DecodeItem> {
    segs: Vec<Seg<T>>
}

#[derive(Clone, Debug)]
enum Seg<T: DecodeItem> {
    Scan(&'static [T]),
    Trie(Vec<Node<T>>),
}

#[derive(Clone, Debug)]
struct Node<T: DecodeItem> {
    bits: T::Word,
    mask: T::Word,
    segs: Vec<Seg<T>>,
}

impl<T> DecodeIndex<T> where T: DecodeItem {
    const MAX_SEL_BITS: u8 = 6;

    pub fn empty() -> Self {
        Self { segs: vec![] }
    }

    /*
    pub fn from(src: &[&T]) -> Self {
        if src.is_empty() {
            Self::empty()
        } else {
            Self::from_(src, !T::Word::ZERO)
        }
    }

    fn index(src: &'static [T], care: T::Word) -> Seg<T> {
        debug_assert!(!src.is_empty());

        // Scan 0:
        // * Determine which bits are significant (mask) for all items.
        // * Determine which bits have the same value for all items.

        let (first, rest) = src.split_first().unwrap();

        let mut prev = first.bits();        // bits of previous item
        let mut diff = T::Word::ZERO;       // bits that differ (1=different)
        let mut mask = first.mask() & care; // bits significant to all items

        for item in rest {
            let bits = item.bits();
            diff |= bits ^ prev;
            prev  = bits;
            mask &= item.mask();
        }

        // Choose a 'selector'
        //   -- a range of selective (significant and differing) bits.

        let (mask, pos, len) = Self::find_mask(mask & diff, Self::MAX_SEL_BITS);

        // If no selective range, a trie is impossible; use scan instead.
        // This branch also used for single items.

        if len == 0 {
            return Scan(src)
        }

        // Scan 1:
        // * Discover contiguous item ranges with same selector.

        let mut ranges = Vec::with_capacity(1 << len);
        {
            let mut start  = 0;
            let mut end    = 1;
            let mut prev   = first.bits() & mask;

            for item in rest {
                let bits = item.bits() & mask;
                let next = end + 1;

                if bits != prev {
                    ranges.push((start..end, prev));
                    start = end;
                    prev  = bits;
                }

                end = next;
            }
        }

        // Scan 2:
        // * 
        //
        // need: (sel) => (range, is_contig)


        // * Contiguous => trie; noncontiguous => scan.
        //
        //for (i, item) in src.iter().enumerate() {
        //    let sel = ((item.bits() & mask) >> pos).to_usize();
        //    {
        //        let range = &mut ranges[sel];
        //        if range.len() == 0 {
        //            *range = i..(i+1);
        //        } else if range.end + 1 == i {
        //            range.end = i;
        //        } else {
        //            panic!();
        //        }
        //    }
        //}

        /*
        let care = care & !mask; // for subnodes

        // Distribute items into bins by their selective bits.

        let mut bins = vec![Vec::new(); 1 << len];

        for &x in src {
            let i = ((x.bits() & mask) >> (pos as usize)).to_usize().unwrap();
            bins[i].push(x);
        }

        // Convert bins into nodes

        Self {
            nodes: bins
                .into_iter()
                .map(|items| Node {
                    bits:    T::Word::ZERO, // TODO
                    mask:    T::Word::ZERO, // TODO
                    content: Content::Trie(Self::from_(&items[..], care)),
                })
                .collect()
        }
        */

        Trie(vec![])
    }
    */

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

