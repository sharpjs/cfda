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

pub trait Visitor<R: Default = ()> {
    fn visit_stmt
        (&mut self, node: &Stmt) -> R { R::default() }

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

pub trait Accept<R> {
    fn accept(&self, v: &mut Visitor<R>) -> R;
}

impl<R: Default> Accept<R> for Stmt {
    fn accept(&self, v: &mut Visitor<R>) -> R { v.visit_stmt(self) }
}

impl<R: Default> Accept<R> for Op {
    fn accept(&self, v: &mut Visitor<R>) -> R { v.visit_op(self) }
}

impl<R: Default> Accept<R> for Arg {
    fn accept(&self, v: &mut Visitor<R>) -> R { v.visit_arg(self) }
}

impl<R: Default> Accept<R> for Expr {
    fn accept(&self, v: &mut Visitor<R>) -> R { v.visit_expr(self) }
}

impl<R: Default> Accept<R> for Ident {
    fn accept(&self, v: &mut Visitor<R>) -> R { v.visit_ident(self) }
}

impl<R: Default> Accept<R> for Int {
    fn accept(&self, v: &mut Visitor<R>) -> R { v.visit_int(self) }
}

impl<R: Default> Accept<R> for String {
    fn accept(&self, v: &mut Visitor<R>) -> R { v.visit_str(self) }
}

