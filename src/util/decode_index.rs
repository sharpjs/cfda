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

type Flags = u32;

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
enum Node<T: DecodeItem> {
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

use self::Node::*;

pub struct Ins;

impl DecodeItem for Ins {
    type Word   = u16;
    type Output = Ins;

    fn try_decode(&self, word: u16) -> Option<Ins> { Some(Ins) }
}

// Root Node
static DECODE_ROOT: Node<Ins> =
    /*......*/ Trie16(&DECODE_XXXXXX, 12)
;

// All Instructions
static DECODE_XXXXXX: [Node<Ins>; 16] = [
    /*00....*/ Trie8(&DECODE_00XXXX, /*>>*/ 6),     // Bit Manipulation/Immediate
    /*01....*/ Leaf(&MOVEB),                        // Move Byte
    /*02....*/ Scan2(&DECODE_02XXXX),               // Move Long
    /*03....*/ Scan2(&DECODE_03XXXX),               // Move Word
    /*04....*/ Empty,                               // Miscellaneous
    /*05....*/ Empty,                               // ADDQ/SUBQ/Scc/TPF
    /*06....*/ Empty,                               // Bcc/BSR/BRA
    /*07....*/ Empty,                               // MOVEQ/MVS/MVZ
    /*10....*/ Empty,                               // OR/DIV
    /*11....*/ Trie8(&DECODE_11XXXX, /*>>*/ 6),     // SUB/SUBA/SUBX
    /*12....*/ Empty,                               // MAC/EMAC/MOV3Q
    /*13....*/ Empty,                               // CMP/EOR
    /*14....*/ Empty,                               // AND/MUL
    /*15....*/ Trie8(&DECODE_15XXXX, /*>>*/ 6),     // ADD/ADDA/ADDX
    /*16....*/ Empty,                               // Shift
    /*17....*/ Empty,                               // Floating-Point/Debug/Cache
];

// Bit Manipulation/Immediate
static DECODE_00XXXX: [Node<Ins>; 8] = [
    /*00.0..*/ Trie2(&DECODE_00X0XX, /*shr*/10),
    /*00.1..*/ Empty,
    /*00.2..*/ Empty,
    /*00.3..*/ Empty,
    /*00.4..*/ Scan2(&DECODE_00X4XX),               // Bit Test
    /*00.5..*/ Scan2(&DECODE_00X5XX),               // Bit Change
    /*00.6..*/ Scan2(&DECODE_00X6XX),               // Bit Clear
    /*00.7..*/ Scan2(&DECODE_00X7XX),               // Bit Set
];

// Bit Test Immediate / Compare Byte Immediate
static DECODE_00X0XX: [Node<Ins>; 2] = [
    /* 0000 .0. 000 ... ... */ Scan2(&DECODE_0000XX),
    /* 0000 .1. 000 ... ... */ Leaf(&CMPIB),
];

// Bit Test Immediate
static DECODE_0000XX: [Node<Ins>; 2] = [
    /*[0]*/ Leaf(&BTSTL), // use immediate encoding, dst=dr
    /*[1]*/ Leaf(&BTSTB), // use immediate encoding, dst=ea
];

// Bit Test
static DECODE_00X4XX: [Node<Ins>; 2] = [
    /*[0]*/ Leaf(&BTSTL),
    /*[1]*/ Leaf(&BTSTB),
];

// Bit Change
static DECODE_00X5XX: [Node<Ins>; 2] = [
    /*[0]*/ Leaf(&BCHGL),
    /*[1]*/ Leaf(&BCHGB),
];

// Bit Clear
static DECODE_00X6XX: [Node<Ins>; 2] = [
    /*[0]*/ Leaf(&BCLRL),
    /*[1]*/ Leaf(&BCLRB),
];

// Bit Set
static DECODE_00X7XX: [Node<Ins>; 2] = [
    /*[0]*/ Leaf(&BSETL),
    /*[1]*/ Leaf(&BSETB),
];

// Move Long
static DECODE_02XXXX: [Node<Ins>; 2] = [
    /*[0]*/ Leaf(&MOVEL),
    /*[1]*/ Leaf(&MOVEAL),
];

// Move Word
static DECODE_03XXXX: [Node<Ins>; 2] = [
    /*[0]*/ Leaf(&MOVEW),
    /*[1]*/ Leaf(&MOVEAW),
];

static DECODE_11XXXX: [Node<Ins>; 8] = [
    /*11.0..*/ Empty,
    /*11.1..*/ Empty,
    /*11.2..*/ Leaf(&SUBL),                         // sub.l ea,dr
    /*11.3..*/ Empty,
    /*11.4..*/ Empty,
    /*11.5..*/ Empty,
    /*11.6..*/ Scan2(&DECODE_11X6XX),               // sub.l dr,ea; subx.l
    /*11.7..*/ Leaf(&SUBAL),                        // suba.l
];

static DECODE_11X6XX: [Node<Ins>; 2] = [
    /*[0]*/ Leaf(&SUBL),                            // sub.l dr,ea
    /*[1]*/ Leaf(&SUBXL),                           // subx.l
];

static DECODE_15XXXX: [Node<Ins>; 8] = [
    /*15.0..*/ Empty,
    /*15.1..*/ Empty,
    /*15.2..*/ Leaf(&ADDL),                         // add.l ea,dr
    /*15.3..*/ Empty,
    /*15.4..*/ Empty,
    /*15.5..*/ Empty,
    /*15.6..*/ Scan2(&DECODE_15X6XX),               // add.l dr,ea; addx.l
    /*15.7..*/ Leaf(&ADDAL),                        // adda.l
];

static DECODE_15X6XX: [Node<Ins>; 2] = [
    /*[0]*/ Leaf(&ADDL),                            // add.l dr,ea
    /*[1]*/ Leaf(&ADDXL),                           // addx.l
];

// Instructs + Encodings
static ADDL:   Ins = Ins;
static ADDAL:  Ins = Ins;
static ADDIL:  Ins = Ins;
static ADDQL:  Ins = Ins;
static ADDXL:  Ins = Ins;
static BCHGB:  Ins = Ins;
static BCHGL:  Ins = Ins;
static BCLRB:  Ins = Ins;
static BCLRL:  Ins = Ins;
static BSETB:  Ins = Ins;
static BSETL:  Ins = Ins;
static BTSTB:  Ins = Ins;
static BTSTL:  Ins = Ins;
static CMPIB:  Ins = Ins;
static CMPIW:  Ins = Ins;
static CMPIL:  Ins = Ins;
static DIVSW:  Ins = Ins;
static DIVSL:  Ins = Ins;
static DIVUW:  Ins = Ins;
static DIVUL:  Ins = Ins;
static MOVEB:  Ins = Ins;
static MOVEW:  Ins = Ins;
static MOVEL:  Ins = Ins;
static MOVEAW: Ins = Ins;
static MOVEAL: Ins = Ins;
static MULSW:  Ins = Ins;
static MULSL:  Ins = Ins;
static MULUW:  Ins = Ins;
static MULUL:  Ins = Ins;
static SUBL:   Ins = Ins;
static SUBAL:  Ins = Ins;
static SUBIL:  Ins = Ins;
static SUBQL:  Ins = Ins;
static SUBXL:  Ins = Ins;

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

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::size_of;

    #[test]
    fn size_of_node() {
        assert_eq!( size_of::<Node<Ins>>(), 16 );
    }

    #[test]
    fn size_of_option_decoded() {
        assert_eq!( size_of::<Option<Decoded<Ins>>>(), 16 );
    }
}

