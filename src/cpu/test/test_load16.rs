use cpu::registers::Reg16;
use cpu::test::run_test;
use std::num::Wrapping;

fn test_load16(opcode: u8, x: u16, reg: Reg16) -> bool {
  let h = (x >> 8) as u8;
  let l = x as u8;
  let cpu = run_test(
    &[opcode, l, h],
    |_| {}
  );
  cpu.clock_cycles() == 12 &&
    cpu.regs.read16(reg) == x
}

#[quickcheck]
fn test_01(x: u16) -> bool {
  test_load16(0x01, x, Reg16::BC)
}

#[quickcheck]
fn test_11(x: u16) -> bool {
  test_load16(0x11, x, Reg16::DE)
}

#[quickcheck]
fn test_21(x: u16) -> bool {
  test_load16(0x21, x, Reg16::HL)
}

#[quickcheck]
fn test_31(x: u16) -> bool {
  test_load16(0x31, x, Reg16::SP)
}