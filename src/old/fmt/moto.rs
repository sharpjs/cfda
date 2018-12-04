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

use std::fmt::{Display, Error, Formatter, Write};
use ast::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct MotoStyled<'a> {
    code: &'a [Stmt],
    // ... plus context
}

impl<'a> Display for MotoStyled<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        MotoFormatter::new(f).visit_stmts(self.code)
    }
}

pub struct MotoFormatter<'a, 'b: 'a> {
    f: &'a mut Formatter<'b>,
    // ... plus context
}

impl<'a, 'b: 'a> MotoFormatter<'a, 'b> {
    pub fn new(f: &'a mut Formatter<'b>) -> Self {
        MotoFormatter { f }
    }
}

impl<'a, 'b: 'a> Visitor<(), Error> for MotoFormatter<'a, 'b> {
    fn visit_str(&mut self, node: &str) -> Result<(), Error> {
        self.f.write_char('"')?;

        for c in node.chars() {
            fmt_char(c, self.f)?;
        }

        self.f.write_char('"')
    }
}

fn fmt_char(c: char, f: &mut Formatter) -> Result<(), Error> {
    Ok(match c {
        '\x08'          => f.write_str("\\b")?,
        '\x09'          => f.write_str("\\t")?,
        '\x0A'          => f.write_str("\\n")?,
        '\x0C'          => f.write_str("\\f")?,
        '\x0D'          => f.write_str("\\r")?,
        '\"'            => f.write_str("\\\"")?,
        '\\'            => f.write_str("\\\\")?,
        '\x20'...'\x7E' => f.write_char(c)?,
        _               => fmt_esc_utf8(c, f)?
    })
}

fn fmt_esc_utf8(c: char, f: &mut Formatter) -> Result<(), Error> {
    let mut buf = [0u8; 4];
    let bytes = c.encode_utf8(&mut buf).as_bytes();
    for &b in bytes {
        write!(f, "\\x{:02X}", b)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use ops::*;

    #[test]
    pub fn foo() {
        let stmts = &[
            Stmt {
                labels: vec![],
                op:     Slot::Value(Op::Asm(&AsmOp::CharsZ)),
                args:   vec![
                    Slot::Value(Arg::Expr(Expr::Str("eh".to_string())))
                ],
            }
        ];

        let styled = MotoStyled { code: stmts };

        let code = format!("{}", styled);

        assert_eq!("\"eh\"", code);
    }
}

