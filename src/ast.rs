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
use cf::CfOp;

/// An assembly statement.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Stmt {
    /// Labels preceding the statement.
    pub labels: Vec<Ident>,

    /// Operation indicated by the statement.
    pub op: Op,

//  /// Arguments to the operation.
//  pub args: Vec<Arg>,
}

/// An assembly operation.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Op {
    /// A platform-agnostic pseudo-op.
    Asm(&'static AsmOp),

    /// A ColdFire instruction.
    Cf(&'static CfOp),
}

// #[derive(Clone, PartialEq, Eq, Hash, Debug)]
// pub enum Arg {
//     /// A platform-agnostic argument.
//     Asm(AsmArg),
// 
//     /// A ColdFire-specific argument.
//     Cf(CfArg),
// }

//#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
//pub enum Arg {
//    Expr(Expr),
//    // string
//    // character
//    // number          >-- immediate?
//    // identifier      >-- refers to something
//    // addressing mode \
//    // cache specifier  >- platform-specific
//    // register-pair   /
//}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Expr {
    Ident(Ident),
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Ident (usize);

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Num (i64);

