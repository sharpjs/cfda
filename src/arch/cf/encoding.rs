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

use super::flags::*;
use super::{Operand, Operand as O};

/// ColdFire 1-word instruction encoding.
#[derive(Clone, Copy, Debug)]
pub struct WordEncoding {
    /// Values of required bits in opword.
    pub bits: u16,
 
    /// Mask of required bits in opword.
    pub mask: u16,

    /// Operand kinds and bit positions.
    pub operands: [Operand; 2],

    /// Flags (arity, hardware support)
    pub flags: CfFlags,
}

/// ColdFire 2-word instruction encoding.
#[derive(Clone, Copy, Debug)]
pub struct LongEncoding {
    /// Values of required bits in opword and extension word.
    /// The extension word portion occupies the upper 16 bits.
    pub bits: u32,
 
    /// Mask of required bits in opword and extension word.
    /// The extension word portion occupies the upper 16 bits.
    pub mask: u32,

    /// Operand kinds and bit positions.
    pub operands: [Operand; 5],

    /// Flags (arity, hardware support)
    pub flags: CfFlags,
}

macro_rules! encodings {
    {
        $array:ident: [$type:ident] <$operands:ident, $indexes:ident> =
        $( $name:ident $bits:tt $mask:tt [$($operand:ident),*] $flags:expr; )*
    } =>
    {
        pub static $array: [$type; count!($($name)*)] = [$(
            $type {
                bits:     opcode!($bits),
                mask:     opcode!($mask),
                operands: $operands!($($operand),*),
                flags:    CfFlags::new(count!($($operand)*), $flags),
            }
        ),*];

        pub enum $indexes { $($name),* }
    };
}

macro_rules! opcode {
    { ($a:expr, $b:expr) } => { $a | $b << 16 };
    {  $a:expr           } => { $a            };
}

macro_rules! operands2 {
    {                    } => {[ O::None, O::None ]};
    { $a:ident           } => {[ O::$a,   O::None ]};
    { $a:ident, $b:ident } => {[ O::$a,   O::$b   ]};
}

macro_rules! operands5 {
    {                                                  } => {[ O::None, O::None, O::None, O::None, O::None ]};
    { $a:ident                                         } => {[ O::$a,   O::None, O::None, O::None, O::None ]};
    { $a:ident, $b:ident                               } => {[ O::$a,   O::$b,   O::None, O::None, O::None ]};
    { $a:ident, $b:ident, $c:ident                     } => {[ O::$a,   O::$b,   O::$c,   O::None, O::None ]};
    { $a:ident, $b:ident, $c:ident, $d:ident           } => {[ O::$a,   O::$b,   O::$c,   O::$d,   O::None ]};
    { $a:ident, $b:ident, $c:ident, $d:ident, $e:ident } => {[ O::$a,   O::$b,   O::$c,   O::$d,   O::$e   ]};
}

macro_rules! count {
    { $($x:tt)* } => { 0 $(+ one!($x))* }
}

macro_rules! one {
    { $($x:tt)* } => { 1 }
}

// ColdFire instruction encodings table
// Source: ColdFire Family Programmer’s Reference Manual, Rev. 3

encodings! {
    WORD_ENCODINGS: [WordEncoding] <operands2, WordEncodings> =
    
//  NAME       WORD      MASK      OPERANDS                    FLAGS
//  ---------  --------  --------  --------------------------  -----
    Addl0      0o150200  0o170700  [MdaipmdxnfDXI0, DataReg9]  ISA_A_UP;
    Addl1      0o150600  0o170700  [DataReg9, M__ipmdxnf___0]  ISA_A_UP;
//  ---------  --------  --------  --------------------------  -----
    Byterevl0  0o001300  0o177770  [DataReg0]                  ISA_C;
//  ---------  --------  --------  --------------------------  -----
//  ...
}

encodings! {
    LONG_ENCODINGS: [LongEncoding] <operands5, LongEncodings> =
    
//  NAME       WORD                  MASK                  OPERANDS                     FLAGS
//  ---------  --------------------  --------------------  ---------------------------  -----
    Mulul0     (0o046000, 0o000000)  (0o177700, 0o107777)  [Md_ipmd______0, DataReg28]  ISA_A_UP;
//  ---------  --------------------  --------------------  ---------------------------  -----
    Mulsl0     (0o046000, 0o004000)  (0o177700, 0o107777)  [Md_ipmd______0, DataReg28]  ISA_A_UP;
//  ---------  --------------------  --------------------  ---------------------------  -----
//  ...
}

