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

//use crate::ast::*;
use crate::decode::*;
use crate::decode::DecodeIndex::*;
use crate::mem::{BE, Load};
use super::CfStmt;
use super::inst::*;

#[derive(Copy, Clone, Debug)]
pub struct CfContext;

#[derive(Copy, Clone, Debug)]
pub struct CfDecoder;
impl Decode<[u8], CfContext> for CfDecoder {
    type Output = CfStmt;

    fn decode<'a>(&self, buf: &'a [u8], ctx: &CfContext) -> Option<(CfStmt, &'a [u8])> {
        let (word, rest) = u16::load(buf, BE)?;
        let ctx = CfDecode16 { opword: word, state: CfContext };
        DECODE_ROOT.decode(rest, &ctx)
    }
}

struct CfDecode16 {
    pub opword: u16,
    pub state:  CfContext,
}

impl Opword for CfDecode16 {
    type Opword = u16;

    fn opword(&self) -> u16 { self.opword }
}

impl Decode<[u8], CfDecode16> for WordEncoding {
    type Output = CfStmt;

    fn decode<'a>(&self, buf: &'a [u8], ctx: &CfDecode16) -> Option<(CfStmt, &'a [u8])> {
        let word = ctx.opword;
        if word & self.mask != self.bits { return None }
        // operands
        // assemble ast
        None
    }
}

/*

struct CfDecode32 {
    pub opword: u32,
    pub state:  u16,
    }

pub struct CfDecoder;

impl Decode<[u8]> for CfDecoder {
    type Output = ();

    fn decode(&self, buf: &[u8], ctx: &()) -> Option<((), &[u8])> {
        panic!()
    }
}

pub enum CfOp16 {
    Next(&'static ()),
    Inst(CfInstruction)
}
*/

type WNode = DecodeIndex<WordEncoding>;

// Root Node
static DECODE_ROOT: WNode = Empty
//    /*......*/ Trie16(&DECODE_XXXXXX, 12)
;

