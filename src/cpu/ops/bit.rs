/*
    BIT - Test Bits in Memory with Accumulator
    Operation: A ∧ M, M7 → N, M6 → V

    This instruction performs an AND between a memory
    location and the accumulator but does not store the
    result of the AND into the accumulator.

    The bit instruction affects the N flag with N being
    set to the value of bit 7 of the memory being tested,
    the V flag with V being set equal to bit 6 of the
    memory being tested and Z being set by the result of
    the AND operation between the accumulator and the
    memory if the result is Zero, Z is reset otherwise.
    It does not affect the accumulator.
*/

use crate::cpu::{addr::AddrModeResult, CPU};

impl CPU {
    #[inline]
    pub(in crate::cpu) fn bitc(&self, mode: &AddrModeResult) -> u8 {
        2 + mode.cycles
    }

    #[inline]
    pub(in crate::cpu) fn bit(&mut self, mode: &AddrModeResult) {
        let data = mode.data.unwrap();

        self.n = (data & 0b1000_0000) > 0;
        self.v = (data & 0b0100_0000) > 0;
        self.z = (self.a & data) == 0;
    }
}

#[cfg(test)]
mod bit_tests {
    use mockall::predicate::eq;

    use crate::cpu::bus::MockCPUBus;

    use super::*;

    #[test]
    fn test_bit_zp_correct_number_ofc() {
        let cpu = CPU::new();
        let mut bus = MockCPUBus::new();

        bus.expect_read().with(eq(0x0)).times(1).return_const(0x0);

        assert_eq!(3, cpu.bitc(&cpu._zp(0x0, &bus)));
    }

    #[test]
    fn test_bit_abs_correct_number_ofc() {
        let cpu = CPU::new();
        let mut bus = MockCPUBus::new();

        bus.expect_read().with(eq(0x0)).times(1).return_const(0x0);

        assert_eq!(4, cpu.bitc(&cpu._abs(0x0, &bus)));
    }

    #[test]
    fn test_bit_negative_flag() {
        let mut cpu = CPU::new();
        let mut bus = MockCPUBus::new();
        cpu.a = 0x80;

        bus.expect_read().with(eq(0x0)).times(1).return_const(0x80);

        cpu.bit(&cpu._zp(0x0, &bus));

        assert_eq!(true, cpu.n);
        assert_eq!(false, cpu.v);
        assert_eq!(false, cpu.z);
    }

    #[test]
    fn test_bit_overflow_flag() {
        let mut cpu = CPU::new();
        let mut bus = MockCPUBus::new();
        cpu.a = 0x40;

        bus.expect_read().with(eq(0x0)).times(1).return_const(0x40);

        cpu.bit(&cpu._zp(0x0, &bus));

        assert_eq!(false, cpu.n);
        assert_eq!(true, cpu.v);
        assert_eq!(false, cpu.z);
    }

    #[test]
    fn test_bit_zero_flag() {
        let mut cpu = CPU::new();
        let mut bus = MockCPUBus::new();

        bus.expect_read().with(eq(0x0)).times(1).return_const(0x0);

        cpu.bit(&cpu._zp(0x0, &bus));

        assert_eq!(false, cpu.n);
        assert_eq!(false, cpu.v);
        assert_eq!(true, cpu.z);
    }
}
