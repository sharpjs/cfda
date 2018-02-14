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

mod char;
mod slot;
mod visitor;

use cf;
use ops::AsmOp;
use num_bigint::BigInt;

pub use self::char::Char;
pub use self::slot::Slot;
pub use self::visitor::Visitor;

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
    Cf(&'static cf::Op),
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum Arg {
    /// A platform-agnostic argument.
    Expr(Expr),

    /// A ColdFire-specific argument.
    Cf(cf::Arg),
}

/// An assembly expression.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum Expr {
    Ident(Ident), // when used as the identifier itself, not in reference to something else
    Int(Int),
    Str(String),
    Char(Char),
    Unary(Unary),
    Binary(Binary),
}

/// An assembly identifier.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Ident (usize);

/// An assembly integer literal.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Int (BigInt);

/// An assembly unary operator expression.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Unary { 
    op:   i32,
    expr: Box<Slot<Expr>>,
}

/// An assembly binary operator expression.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Binary {
    op:  i32,
    lhs: Box<Slot<Expr>>,
    rhs: Box<Slot<Expr>>,
}

