/*
    ROR - Rotate Right
    Operation: C → /M7...M0/ → C

    The rotate right instruction shifts either the accumulator
    or addressed memory right 1 bit with bit 0 shifted into the
    carry and carry shifted into bit 7.

    The ROR instruction either shifts the accumulator right 1 bit
    and stores the carry in accumulator bit 7 or does not affect
    the internal regis­ ters at all. The ROR instruction sets carry
    equal to input bit 0, sets N equal to the input carry and sets
    the Z flag if the result of the rotate is 0; otherwise it resets
    Z and does not affect the overflow flag at all.
*/

use crate::cpu::{
    addr::{AddrModeResult, AddrModeType},
    bus::CPUBus,
};

use super::super::CPU;

impl CPU {
    #[inline]
    pub(in crate::cpu) fn rorc(&self, mode: &AddrModeResult) -> u8 {
        match mode.mode {
            AddrModeType::ACC => 2,
            AddrModeType::ABSX => 7,
            _ => 4 + mode.cycles,
        }
    }

    #[inline]
    pub(in crate::cpu) fn ror(&mut self, mode: &AddrModeResult, bus: &mut dyn CPUBus) {
        let before = mode.data.unwrap();
        let after: u8 = ((self.c as u8) << 7) | before >> 1;

        self.c = (before & 0x1) > 0;
        self.n = (after & 0x80) > 0;
        self.z = after == 0;

        if let Some(addr) = mode.addr {
            bus.write(addr, after);
        } else {
            self.a = after;
        }
    }
}

#[cfg(test)]
mod ror_tests {
    use mockall::predicate::eq;

    use crate::cpu::bus::MockCPUBus;

    use super::*;

    #[test]
    fn test_ror_acc() {
        let mut cpu = CPU::new();

        cpu.a = 0x20;
        assert_eq!(2, cpu.rorc(&cpu._acc()));
    }

    #[test]
    fn test_ror_zp() {
        let cpu = CPU::new();
        let mut bus = MockCPUBus::new();

        bus.expect_read().with(eq(0x0)).times(1).return_const(0xff);

        assert_eq!(5, cpu.rorc(&cpu._zp(0x0, &bus)));
    }

    #[test]
    fn test_ror_zpx() {
        let mut cpu = CPU::new();
        let mut bus = MockCPUBus::new();

        cpu.x = 0x2;
        bus.expect_read().with(eq(0x2)).times(1).return_const(0x1);

        assert_eq!(6, cpu.rorc(&cpu._zpx(0x0, &bus)));
    }

    #[test]
    fn test_ror_abs() {
        let cpu = CPU::new();
        let mut bus = MockCPUBus::new();

        bus.expect_read()
            .with(eq(0xffff))
            .times(1)
            .return_const(0xaa);

        assert_eq!(6, cpu.rorc(&cpu._abs(0xffff, &bus)));
    }

    #[test]
    fn test_ror_absx() {
        let mut cpu = CPU::new();
        let mut bus = MockCPUBus::new();

        cpu.x = 0x2;

        bus.expect_read().with(eq(0x1)).times(1).return_const(0x88);

        assert_eq!(7, cpu.rorc(&cpu._absx(0xffff, &bus)));
    }

    #[test]
    fn test_ror_carry_flag() {
        let mut cpu = CPU::new();
        let mut bus = MockCPUBus::new();

        cpu.a = 0b0000_0001;
        cpu.c = true;

        cpu.ror(&cpu._acc(), &mut bus);
        assert_eq!(0b1000_0000, cpu.a);
        assert_eq!(true, cpu.c);
    }

    #[test]
    fn test_ror_no_carry_flag() {
        let mut cpu = CPU::new();
        let mut bus = MockCPUBus::new();

        cpu.a = 0b0000_0001;

        cpu.ror(&cpu._acc(), &mut bus);
        assert_eq!(0b0000_0000, cpu.a);
        assert_eq!(true, cpu.c);
    }

    #[test]
    fn test_ror_negative_flag() {
        let mut cpu = CPU::new();
        let mut bus = MockCPUBus::new();

        cpu.a = 0x0;
        cpu.c = true;

        cpu.ror(&cpu._acc(), &mut bus);
        assert_eq!(0x80, cpu.a);
        assert_eq!(true, cpu.n);
    }

    #[test]
    fn test_ror_zero_flag() {
        let mut cpu = CPU::new();
        let mut bus = MockCPUBus::new();

        cpu.a = 0x1;
        cpu.ror(&cpu._acc(), &mut bus);

        assert_eq!(true, cpu.z);
    }
}
