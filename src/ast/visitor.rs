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

use super::*;

pub trait VisitorResult {
    fn default() -> Self;
    fn and(self, other: Self) -> Self;
    fn is_err(&self) -> bool;
}

impl VisitorResult for () {
    fn default() -> Self { () }
    fn and(self, other: Self) -> Self { () }
    fn is_err(&self) -> bool { false }
}

pub trait Visitor<R: VisitorResult = ()> {

    fn visit_stmt(&mut self, node: &Stmt) -> R {
        let mut r = R::default();

        for label in node.labels.iter() {
            r = r.and(self.visit_ident(label));
            if r.is_err() { return r }
        }

        r
    }

    fn visit_op
        (&mut self, node: &Op) -> R { R::default() }

    fn visit_arg
        (&mut self, node: &Arg) -> R { R::default() }

    fn visit_expr
        (&mut self, node: &Expr) -> R { R::default() }

    fn visit_ident
        (&mut self, node: &Ident) -> R { R::default() }

    fn visit_int
        (&mut self, node: &Int) -> R { R::default() }

    fn visit_str
        (&mut self, node: &str) -> R { R::default() }
}


