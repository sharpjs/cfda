// This file is part of cfda, a fun little disassembler project.
// Copyright (C) 2017 Jeffrey Sharp
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

use ops::AsmOp;
use cf::{CfOp, CfArg};

/// An assembly statement.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Stmt {
    /// Labels preceding the statement.
    pub labels: Vec<Ident>,

    /// Operation indicated by the statement.
    pub op: Slot<Op>,

    /// Arguments to the operation.
    pub args: Vec<Slot<Arg>>,
}

/// An assembly operation.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Op {
    /// A platform-agnostic pseudo-op.
    Asm(&'static AsmOp),

    /// A ColdFire instruction.
    Cf(&'static CfOp),
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum Arg {
    /// A platform-agnostic argument.
    Expr(Expr),

    /// A ColdFire-specific argument.
    Cf(CfArg),
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum Expr {
    Ident(Ident), // when used as the identifier itself, not in reference to something else
    Num(Num),
    Str(String),
    Char(Char),
    Unary(i32, Box<Slot<Expr>>),
    Binary(i32, Box<Slot<Expr>>, Box<Slot<Expr>>),
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Ident (usize);

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Num (i64);

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Char (char);

/// A slot containing a value and/or an identifier that resolves to that value.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Slot<V> {
    /// Unresolved identifier
    Ident(Ident),

    /// Identifier resolved to a value.
    Resolved(Ident, V),

    /// Value resolved without an identifier.
    Value(V),
}

impl<V> Slot<V> {
    /// Returns the identifier reference, if any, in the slot.
    #[inline]
    pub fn ident(&self) -> Option<Ident> {
        match *self {
            Slot::Ident    (i   ) => Some(i),
            Slot::Resolved (i, _) => Some(i),
            Slot::Value    (   _) => None,
        }
    }

    /// Returns a reference to the resolved value, if any, in the slot.
    #[inline]
    pub fn value(&self) -> Option<&V> {
        match *self {
            Slot::Ident    (_       ) => None,
            Slot::Resolved (_, ref v) => Some(v),
            Slot::Value    (   ref v) => Some(v),
        }
    }

    /// Returns a copy of the resolved value, if any, in the slot.
    #[inline]
    pub fn value_copy(&self) -> Option<V> where V: Copy {
        match *self {
            Slot::Ident    (_   ) => None,
            Slot::Resolved (_, v) => Some(v),
            Slot::Value    (   v) => Some(v),
        }
    }
}

