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

use crate::arch::Arch;
use super::Ident;
use super::Slot;

/// A block of assembly statements.
#[derive(Debug)]
pub struct Block<A: Arch> {
    /// Assembly statements.
    pub stmts: Vec<Stmt<A>>,
}

/// An assembly statement.
#[derive(Debug)]
pub struct Stmt<A: Arch> {
    /// Labels preceding the statement.
    pub labels: Vec<Ident>,

    /// Operation requested by the statement.
    pub op: Slot<A::Op>,

    /// Arguments to the operation.
    pub args: Vec<Slot<A::Arg>>,
}

