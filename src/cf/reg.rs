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

use std::fmt::{self, Display, Formatter};

// -----------------------------------------------------------------------------
// Data Registers

/// A ColdFire data register.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct DataReg (u8);

static DATA_REG_NAMES: [&str; 8] = [
    "d0", "d1", "d2", "d3", "d4", "d5", "d6", "d7"
];

impl DataReg {
    pub const MAX_NUM: u8 = 7;

    #[inline]
    pub fn with_num(n: u8) -> Option<Self> {
        if n <= Self::MAX_NUM {
            Some(DataReg(n))
        } else {
            None
        }
    }

    #[inline]
    pub fn num(self) -> u8 {
        self.0
    }

    #[inline]
    pub fn name(self) -> &'static str {
        DATA_REG_NAMES[self.num() as usize]
    }
}

impl Display for DataReg {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str(self.name())
    }
}

// -----------------------------------------------------------------------------
// Address Registers

/// A ColdFire address register.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct AddrReg (u8);

static ADDR_REG_NAMES: [&str; 8] = [
    "a0", "a1", "a2", "a3", "a4", "a5", "fp", "sp"
];

impl AddrReg {
    pub const MAX_NUM: u8 = 7;

    #[inline]
    pub fn with_num(n: u8) -> Option<Self> {
        if n <= Self::MAX_NUM {
            Some(AddrReg(n))
        } else {
            None
        }
    }

    #[inline]
    pub fn num(self) -> u8 {
        self.0
    }

    #[inline]
    pub fn name(self) -> &'static str {
        ADDR_REG_NAMES[self.num() as usize]
    }
}

impl Display for AddrReg {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str(self.name())
    }
}

// -----------------------------------------------------------------------------
// Miscellaneous Registers

/// The ColdFire program counter register.
pub struct PcReg;

impl Display for PcReg {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str("pc")
    }
}

/// The ColdFire condition code register.
pub struct CcrReg;

impl Display for CcrReg {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str("ccr")
    }
}

/// The ColdFire status register.
pub struct SrReg;

impl Display for SrReg {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str("sr")
    }
}

// -----------------------------------------------------------------------------
// Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn data_reg_with_num_ok() {
        assert_eq!( DataReg::with_num(7), Some(DataReg(7)) );
    }

    #[test]
    pub fn data_reg_with_num_overflow() {
        assert_eq!( DataReg::with_num(8), None );
    }

    #[test]
    pub fn data_reg_num() {
        assert_eq!( DataReg(3).num(), 3 );
    }

    #[test]
    pub fn data_reg_name() {
        assert_eq!( DataReg(3).name(), "d3" );
    }

    #[test]
    pub fn data_reg_fmt() {
        assert_eq!( format!("{}", DataReg(3)), "d3" );
    }

    #[test]
    pub fn addr_reg_with_num_ok() {
        assert_eq!( AddrReg::with_num(7), Some(AddrReg(7)) );
    }

    #[test]
    pub fn addr_reg_with_num_overflow() {
        assert_eq!( AddrReg::with_num(8), None );
    }

    #[test]
    pub fn addr_reg_num() {
        assert_eq!( AddrReg(3).num(), 3 );
    }

    #[test]
    pub fn addr_reg_name() {
        assert_eq!( AddrReg(3).name(), "a3" );
    }

    #[test]
    pub fn addr_reg_name_fp() {
        assert_eq!( AddrReg(6).name(), "fp" );
    }

    #[test]
    pub fn addr_reg_name_sp() {
        assert_eq!( AddrReg(7).name(), "sp" );
    }

    #[test]
    pub fn addr_reg_fmt() {
        assert_eq!( format!("{}", AddrReg(3)), "a3" );
    }

    #[test]
    pub fn pc_reg_fmt() {
        assert_eq!( format!("{}", PcReg), "pc" );
    }

    #[test]
    pub fn ccr_reg_fmt() {
        assert_eq!( format!("{}", CcrReg), "ccr" );
    }

    #[test]
    pub fn sr_reg_fmt() {
        assert_eq!( format!("{}", SrReg), "sr" );
    }
}

