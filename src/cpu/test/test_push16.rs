use cpu::registers::Reg16;
use cpu::test::run_test;

fn test_push16(opcode: u8, reg: Reg16, x: u16) -> bool {
  let cpu = run_test(
    &[opcode, 0xed, 0x00, 0x00],
    |cpu| {
      cpu.regs.write16(reg, x);
      cpu.regs.sp = 0x0004;
    }
  );
  cpu.clock_cycles() == 16 &&
    cpu.regs.sp == 0x0002 &&
    cpu.hardware.memory[0x03] == (x >> 8) as u8 &&
    cpu.hardware.memory[0x02] == (x as u8)
}

#[quickcheck]
fn test_c5(x: u16) -> bool {
  test_push16(0xc5, Reg16::BC, x)
}

#[quickcheck]
fn test_d5(x: u16) -> bool {
  test_push16(0xd5, Reg16::DE, x)
}

#[quickcheck]
fn test_e5(x: u16) -> bool {
  test_push16(0xe5, Reg16::HL, x)
}

#[quickcheck]
fn test_f5(a: u8, f: u8) -> bool {
  let cpu = run_test(
    &[0xf5, 0xed, 0x00, 0x00],
    |cpu| {
      cpu.regs.a = a;
      cpu.regs.f.set(f);
      cpu.regs.sp = 0x0004;
    }
  );
  cpu.clock_cycles() == 16 &&
    cpu.regs.sp == 0x0002 &&
    cpu.hardware.memory[0x03] == a &&
    cpu.hardware.memory[0x02] == (f & 0xF0)
}