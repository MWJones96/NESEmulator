/*
    LSR - Logical Shift Right
    Operation: 0 → /M7...M0/ → C

    This instruction shifts either the accumulator or a
    specified memory location 1 bit to the right, with the
    higher bit of the result always being set to 0, and the
    low bit which is shifted out of the field being stored
    in the carry flag.

    The shift right instruction either affects the accumulator
    by shift­ing it right 1 or is a read/modify/write instruction
    which changes a speci­fied memory location but does not affect
    any internal registers. The shift right does not affect the
    overflow flag. The N flag is always reset. The Z flag is set
    if the result of the shift is 0 and reset otherwise.
    The carry is set equal to bit 0 of the input.
*/

use crate::cpu::{
    addr::{AddrModeResult, AddrModeType},
    bus::CPUBus,
};

use super::super::CPU;

impl CPU {
    pub(in crate::cpu) fn lsrc(&self, mode: &AddrModeResult) -> u8 {
        match mode.mode {
            AddrModeType::Acc => 2,
            AddrModeType::Absx => 7,
            _ => 4 + mode.cycles,
        }
    }

    pub(in crate::cpu) fn lsr(&mut self, mode: &AddrModeResult, bus: &mut dyn CPUBus) {
        let before_shift = mode.data.unwrap();
        let after_shift = before_shift >> 1;

        self.c = (before_shift & 0x1) != 0;
        self.z = after_shift == 0;
        self.n = false;

        if let Some(addr) = mode.addr {
            bus.write(addr, after_shift);
        } else {
            self.a = after_shift;
        }
    }
}

#[cfg(test)]
mod lsr_tests {
    use mockall::predicate::eq;

    use crate::cpu::bus::MockCPUBus;

    use super::*;

    #[test]
    fn test_lsr_acc() {
        let mut cpu = CPU::new();

        cpu.a = 0x20;
        assert_eq!(2, cpu.lsrc(&cpu._acc()));
    }

    #[test]
    fn test_lsr_zp() {
        let cpu = CPU::new();
        let mut bus = MockCPUBus::new();

        bus.expect_read().with(eq(0x0)).times(1).return_const(0xff);

        assert_eq!(5, cpu.lsrc(&cpu._zp(0x0, &bus)));
    }

    #[test]
    fn test_lsr_zpx() {
        let mut cpu = CPU::new();
        let mut bus = MockCPUBus::new();

        cpu.x = 0x2;
        bus.expect_read().with(eq(0x2)).times(1).return_const(0x1);

        assert_eq!(6, cpu.lsrc(&cpu._zpx(0x0, &bus)));
    }

    #[test]
    fn test_lsr_abs() {
        let cpu = CPU::new();
        let mut bus = MockCPUBus::new();

        bus.expect_read()
            .with(eq(0xffff))
            .times(1)
            .return_const(0xaa);

        assert_eq!(6, cpu.lsrc(&cpu._abs(0xffff, &bus)));
    }

    #[test]
    fn test_lsr_absx() {
        let mut cpu = CPU::new();
        let mut bus = MockCPUBus::new();

        cpu.x = 0x2;

        bus.expect_read().with(eq(0x1)).times(1).return_const(0x88);

        assert_eq!(7, cpu.lsrc(&cpu._absx(0xffff, &bus)));
    }

    #[test]
    fn test_lsr_shift() {
        let mut cpu = CPU::new();
        let mut bus = MockCPUBus::new();

        cpu.a = 0x2;
        cpu.lsr(&cpu._acc(), &mut bus);
        assert_eq!(0x1, cpu.a);

        cpu.a = 0xff;
        cpu.lsr(&cpu._acc(), &mut bus);
        assert_eq!(0x7f, cpu.a);

        cpu.a = 0x00;
        cpu.lsr(&cpu._acc(), &mut bus);
        assert_eq!(0x00, cpu.a);
    }

    #[test]
    fn test_lsr_carry_flag() {
        let mut cpu = CPU::new();
        let mut bus = MockCPUBus::new();

        cpu.a = 0x1;
        cpu.lsr(&cpu._acc(), &mut bus);
        assert_eq!(true, cpu.c);

        cpu.a = 0x2;
        cpu.lsr(&cpu._acc(), &mut bus);
        assert_eq!(false, cpu.c);
    }

    #[test]
    fn test_lsr_acc_zero_flag() {
        let mut cpu = CPU::new();
        let mut bus = MockCPUBus::new();

        cpu.a = 0x1;
        cpu.lsr(&cpu._acc(), &mut bus);
        assert_eq!(true, cpu.z);

        cpu.a = 0x2;
        cpu.lsr(&cpu._acc(), &mut bus);
        assert_eq!(false, cpu.z);
    }

    #[test]
    fn test_lsr_acc_negative_flag() {
        let mut cpu = CPU::new();
        let mut bus = MockCPUBus::new();

        cpu.n = true;

        cpu.a = 0x80;
        cpu.lsr(&cpu._acc(), &mut bus);
        assert_eq!(false, cpu.n);
    }

    #[test]
    fn test_lsr_writes_to_memory() {
        let mut cpu = CPU::new();
        let mut bus = MockCPUBus::new();

        bus.expect_read().with(eq(0x0)).times(1).return_const(0xff);

        bus.expect_write()
            .with(eq(0x0), eq(0x7f))
            .times(1)
            .return_const(());

        cpu.lsr(&cpu._zp(0x0, &bus), &mut bus);
    }
}
