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

/// An assembler pseudo-operation.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum AsmOp {
    Origin,
    Skip,
    Zero8,
    Zero16,
    Zero32,
    Zero64,
    Data8,
    Data16,
    Data32,
    Data64,
    Chars,
    CharsZ,
    Public,
    Section,
}

