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

pub trait Visitor<T: Default = (), E = ()> {
    fn visit_stmts(&mut self, nodes: &[Stmt]) -> Result<T, E> {
        for stmt in nodes { self.visit_stmt(stmt)?; }
        Ok(T::default())
    }

    fn visit_stmt(&mut self, node: &Stmt) -> Result<T, E> {
        self.visit_labels(&node.labels)?;
        self.visit_op(&node.op)?;
        self.visit_args(&node.args)?;
        Ok(T::default())
    }

    fn visit_labels(&mut self, nodes: &[Ident]) -> Result<T, E> {
        for node in nodes { self.visit_label(*node)?; }
        Ok(T::default())
    }

    fn visit_label(&mut self, node: Ident) -> Result<T, E> {
        Ok(T::default())
    }

    fn visit_op(&mut self, node: &Slot<Op>) -> Result<T, E> {
        Ok(T::default())
    }

    fn visit_args(&mut self, nodes: &[Slot<Arg>]) -> Result<T, E> {
        for node in nodes { self.visit_arg(node)?; }
        Ok(T::default())
    }

    fn visit_arg(&mut self, node: &Slot<Arg>) -> Result<T, E> {
        Ok(T::default())
    }

    fn visit_expr(&mut self, node: &Expr) -> Result<T, E> {
        Ok(match *node {
            Expr::Ident  (    i) => self.visit_ident(i)?,
            Expr::Int    (ref i) => self.visit_int(i)?,
            Expr::Str    (ref s) => self.visit_str(s)?,
            Expr::Char   (    c) => self.visit_char(c)?,
            Expr::Unary  (ref e) => self.visit_unary(e)?,
            Expr::Binary (ref e) => self.visit_binary(e)?,
        })
    }

    fn visit_ident(&mut self, node: Ident) -> Result<T, E> {
        Ok(T::default())
    }

    fn visit_int(&mut self, node: &Int) -> Result<T, E> {
        Ok(T::default())
    }

    fn visit_str(&mut self, node: &str) -> Result<T, E> {
        Ok(T::default())
    }

    fn visit_char(&mut self, node: Char) -> Result<T, E> {
        Ok(T::default())
    }

    fn visit_unary(&mut self, node: &Unary) -> Result<T, E> {
        self.visit_subexpr(&node.expr)?;
        Ok(T::default())
    }

    fn visit_binary(&mut self, node: &Binary) -> Result<T, E> {
        self.visit_subexpr(&node.lhs)?;
        self.visit_subexpr(&node.rhs)?;
        Ok(T::default())
    }

    fn visit_subexpr(&mut self, node: &Slot<Expr>) -> Result<T, E> {
        Ok(T::default())
    }
}

