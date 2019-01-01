// This file is part of cfda, a fun little disassembler project.
// Copyright (C) 2019 Jeffrey Sharp
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

mod ast;
mod decode;
mod encoding;
mod flags;
mod inst;
mod operand;

pub use self::ast::*;
pub use self::decode::*;
pub use self::encoding::*;
pub use self::flags::*;
pub use self::inst::*;
pub use self::operand::*;

use super::Arch;

use crate::ast::{Block, Stmt};

/// The ColdFire instruction set architecture.
#[derive(Debug)]
pub struct Cf;

impl Arch for Cf {
    type Op  = Op;
    type Arg = Arg;
}

type CfBlock = Block<Cf>;
type CfStmt  = Stmt <Cf>;

