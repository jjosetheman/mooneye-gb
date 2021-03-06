// This file is part of Mooneye GB.
// Copyright (C) 2014-2020 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// Mooneye GB is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Mooneye GB is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Mooneye GB.  If not, see <http://www.gnu.org/licenses/>.
#[derive(Clone)]
pub struct WorkRam {
  ram: Box<[u8; 0x2000]>,
}

impl WorkRam {
  pub fn new() -> WorkRam {
    WorkRam {
      ram: Box::new([0; 0x2000]),
    }
  }

  pub fn read_lower(&self, addr: u16) -> u8 {
    self.ram[(addr as usize) & 0x1fff]
  }
  pub fn write_lower(&mut self, addr: u16, value: u8) {
    self.ram[(addr as usize) & 0x1fff] = value;
  }

  pub fn read_upper(&self, addr: u16) -> u8 {
    self.ram[(addr as usize) & 0x1fff]
  }
  pub fn write_upper(&mut self, addr: u16, value: u8) {
    self.ram[(addr as usize) & 0x1fff] = value;
  }
}
