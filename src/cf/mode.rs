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

//pub enum Mode {
//    DataReg         (DataReg),
//    AddrReg         (AddrReg),
//    AddrRegInd      (AddrReg),
//    AddrRegIndInc   (AddrReg),
//    AddrRegIndDec   (AddrReg),
//    AddrRegDisp     (AddrReg, i16),
//    AddrRegIdx      (AddrReg, i8, Index),
//    PcDisp          (PcReg),
//    PcIdx           (PcReg),
//    AbsShort        (u32),
//    AbsLong         (u32),
//    Imm             (u32),
//}

static MODES: [Mode; 12] = [
    Mode { bits: 0b_000_000, mask: 0b_111_000, words: [0, 0, 0], f: mode_data         },
    Mode { bits: 0b_001_000, mask: 0b_111_000, words: [0, 0, 0], f: mode_addr         },
    Mode { bits: 0b_010_000, mask: 0b_111_000, words: [0, 0, 0], f: mode_addr_ind     },
    Mode { bits: 0b_011_000, mask: 0b_111_000, words: [0, 0, 0], f: mode_addr_ind_inc },
    Mode { bits: 0b_100_000, mask: 0b_111_000, words: [0, 0, 0], f: mode_addr_ind_dec },
    Mode { bits: 0b_101_000, mask: 0b_111_000, words: [1, 1, 1], f: mode_addr_disp    },
    Mode { bits: 0b_110_000, mask: 0b_111_000, words: [1, 1, 1], f: mode_addr_index   },
    Mode { bits: 0b_111_000, mask: 0b_111_111, words: [1, 1, 1], f: mode_abs_short    },
    Mode { bits: 0b_111_001, mask: 0b_111_111, words: [2, 2, 2], f: mode_abs_long     },
    Mode { bits: 0b_111_010, mask: 0b_111_111, words: [1, 1, 1], f: mode_pc_disp      },
    Mode { bits: 0b_111_011, mask: 0b_111_111, words: [1, 1, 1], f: mode_pc_index     },
    Mode { bits: 0b_111_100, mask: 0b_111_111, words: [1, 1, 2], f: mode_imm          },
];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Mode {
    pub bits:  u8,
    pub mask:  u8,
    pub words: [u8; 3], // byte, word, long
    pub f:     fn(),
}

fn mode_data() {}
fn mode_addr() {}
fn mode_addr_ind() {}
fn mode_addr_ind_inc() {}
fn mode_addr_ind_dec() {}
fn mode_addr_disp() {}
fn mode_addr_index() {}
fn mode_pc_disp() {}
fn mode_pc_index() {}
fn mode_abs_short() {}
fn mode_abs_long() {}
fn mode_imm() {}

