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

use util::DecodeIndex;
use util::DecodeIndex::*;
use super::ops::*;

type Dc = DecodeIndex<Instruction>;

// Root Node
static DECODE_ROOT: Dc =
    /*......*/ Trie16(&DECODE_XXXXXX, 12)
;

// All Instructions
static DECODE_XXXXXX: [Dc; 16] = [
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
static DECODE_00XXXX: [Dc; 8] = [
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
static DECODE_00X0XX: [Dc; 2] = [
    /* 0000 .0. 000 ... ... */ Scan2(&DECODE_0000XX),
    /* 0000 .1. 000 ... ... */ Leaf(&CMPIB),
];

// Bit Test Immediate
static DECODE_0000XX: [Dc; 2] = [
    /*[0]*/ Leaf(&BTSTL), // use immediate encoding, dst=dr
    /*[1]*/ Leaf(&BTSTB), // use immediate encoding, dst=ea
];

// Bit Test
static DECODE_00X4XX: [Dc; 2] = [
    /*[0]*/ Leaf(&BTSTL),
    /*[1]*/ Leaf(&BTSTB),
];

// Bit Change
static DECODE_00X5XX: [Dc; 2] = [
    /*[0]*/ Leaf(&BCHGL),
    /*[1]*/ Leaf(&BCHGB),
];

// Bit Clear
static DECODE_00X6XX: [Dc; 2] = [
    /*[0]*/ Leaf(&BCLRL),
    /*[1]*/ Leaf(&BCLRB),
];

// Bit Set
static DECODE_00X7XX: [Dc; 2] = [
    /*[0]*/ Leaf(&BSETL),
    /*[1]*/ Leaf(&BSETB),
];

// Move Long
static DECODE_02XXXX: [Dc; 2] = [
    /*[0]*/ Leaf(&MOVEL),
    /*[1]*/ Leaf(&MOVEAL),
];

// Move Word
static DECODE_03XXXX: [Dc; 2] = [
    /*[0]*/ Leaf(&MOVEW),
    /*[1]*/ Leaf(&MOVEAW),
];

static DECODE_11XXXX: [Dc; 8] = [
    /*11.0..*/ Empty,
    /*11.1..*/ Empty,
    /*11.2..*/ Leaf(&SUBL),                         // sub.l ea,dr
    /*11.3..*/ Empty,
    /*11.4..*/ Empty,
    /*11.5..*/ Empty,
    /*11.6..*/ Scan2(&DECODE_11X6XX),               // sub.l dr,ea; subx.l
    /*11.7..*/ Leaf(&SUBAL),                        // suba.l
];

static DECODE_11X6XX: [Dc; 2] = [
    /*[0]*/ Leaf(&SUBL),                            // sub.l dr,ea
    /*[1]*/ Leaf(&SUBXL),                           // subx.l
];

static DECODE_15XXXX: [Dc; 8] = [
    /*15.0..*/ Empty,
    /*15.1..*/ Empty,
    /*15.2..*/ Leaf(&ADDL),                         // add.l ea,dr
    /*15.3..*/ Empty,
    /*15.4..*/ Empty,
    /*15.5..*/ Empty,
    /*15.6..*/ Scan2(&DECODE_15X6XX),               // add.l dr,ea; addx.l
    /*15.7..*/ Leaf(&ADDAL),                        // adda.l
];

static DECODE_15X6XX: [Dc; 2] = [
    /*[0]*/ Leaf(&ADDL),                            // add.l dr,ea
    /*[1]*/ Leaf(&ADDXL),                           // addx.l
];

