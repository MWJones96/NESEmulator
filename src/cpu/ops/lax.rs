/*
    LAX - Load Accumulator and Index Register X From Memory
    Operation: M → A, X

    The undocumented LAX instruction loads the accumulator and the
    index register X from memory.

    LAX does not affect the C or V flags; sets Z if the value loaded
    was zero, otherwise resets it; sets N if the value loaded in bit 7
    is a 1; otherwise N is reset, and affects only the X register.
*/

use crate::cpu::{addr::AddrModeResult, CPU};

impl CPU {
    #[inline]
    pub(in crate::cpu) fn lax_cycles(&self, mode: &AddrModeResult) -> u8 {
        2 + mode.cycles
    }

    #[inline]
    pub(in crate::cpu) fn lax(&mut self, mode: &AddrModeResult) {
        let data = mode.data.unwrap();

        self.a = data;
        self.x = data;

        self.n = (self.a & 0x80) != 0;
        self.z = self.a == 0;
    }
}

#[cfg(test)]
mod lax_tests {
    use mockall::predicate::eq;

    use crate::cpu::bus::MockCPUBus;

    use super::*;

    #[test]
    fn test_lax_imm_correct_number_of_cycles() {
        let cpu = CPU::new();
        assert_eq!(2, cpu.lax_cycles(&cpu._imm(0x0)));
    }

    #[test]
    fn test_lax_zp_correct_number_of_cycles() {
        let cpu = CPU::new();
        let mut bus = MockCPUBus::new();
        bus.expect_read().return_const(0x0);

        assert_eq!(3, cpu.lax_cycles(&cpu._zp(0x0, &bus)));
    }

    #[test]
    fn test_lax_zpy_correct_number_of_cycles() {
        let cpu = CPU::new();
        let mut bus = MockCPUBus::new();
        bus.expect_read().return_const(0x0);

        assert_eq!(4, cpu.lax_cycles(&cpu._zpy(0x0, &bus)));
    }

    #[test]
    fn test_lax_abs_correct_number_of_cycles() {
        let cpu = CPU::new();
        let mut bus = MockCPUBus::new();
        bus.expect_read().return_const(0x0);

        assert_eq!(4, cpu.lax_cycles(&cpu._abs(0x0, &bus)));
    }

    #[test]
    fn test_lax_cycles_absy_no_page_cross() {
        let cpu = CPU::new();
        let mut bus = MockCPUBus::new();
        bus.expect_read().return_const(0x0);

        assert_eq!(4, cpu.lax_cycles(&cpu._absy(0x0, &bus)));
    }

    #[test]
    fn test_lax_cycles_absy_with_page_cross() {
        let mut cpu = CPU::new();
        cpu.y = 0xff;

        let mut bus = MockCPUBus::new();
        bus.expect_read().return_const(0x0);

        assert_eq!(5, cpu.lax_cycles(&cpu._absy(0x34, &bus)));
    }

    #[test]
    fn test_lax_cycles_indx_correct_number_of_cycles() {
        let cpu = CPU::new();

        let mut bus = MockCPUBus::new();
        bus.expect_read().return_const(0x0);

        assert_eq!(6, cpu.lax_cycles(&cpu._indx(0x0, &bus)));
    }

    #[test]
    fn test_lax_cycles_indy_no_page_cross() {
        let cpu = CPU::new();

        let mut bus = MockCPUBus::new();
        bus.expect_read().return_const(0x0);

        assert_eq!(5, cpu.lax_cycles(&cpu._indy(0x0, &bus)));
    }

    #[test]
    fn test_lax_cycles_indy_with_page_cross() {
        let mut cpu = CPU::new();
        cpu.y = 0xff;

        let mut bus = MockCPUBus::new();
        bus.expect_read().with(eq(0x34)).return_const(0x34);
        bus.expect_read().with(eq(0x12)).return_const(0x12);
        bus.expect_read().return_const(0x0);

        assert_eq!(6, cpu.lax_cycles(&cpu._indy(0x34, &bus)));
    }

    #[test]
    fn test_lax() {
        let mut cpu = CPU::new();

        cpu.lax(&cpu._imm(0xee));

        assert_eq!(0xee, cpu.a);
        assert_eq!(0xee, cpu.x);

        assert_eq!(true, cpu.n);
        assert_eq!(false, cpu.z);

        cpu.lax(&cpu._imm(0x0));

        assert_eq!(0x0, cpu.a);
        assert_eq!(0x0, cpu.x);

        assert_eq!(false, cpu.n);
        assert_eq!(true, cpu.z);
    }
}
