/*
    NOP - No Operation
    Operation: No operation
*/

use crate::cpu::addr::AddrModeResult;

use super::super::CPU;

impl CPU {
    #[inline]
    pub(in crate::cpu) fn nop_cycles(&self, mode: &AddrModeResult) -> u8 {
        2 + mode.cycles
    }

    #[inline]
    pub(in crate::cpu) fn nop(&self, _mode: &AddrModeResult) {
        //No operation
    }
}

#[cfg(test)]
mod nop_tests {
    use crate::cpu::bus::MockCPUBus;

    use super::*;

    #[test]
    fn test_nop_imp_correct_number_of_cycles() {
        let cpu = CPU::new();
        assert_eq!(2, cpu.nop_cycles(&cpu._imp()));
    }

    #[test]
    fn test_nop_imm_correct_number_of_cycles() {
        let cpu = CPU::new();
        assert_eq!(2, cpu.nop_cycles(&cpu._imm(0x0)));
    }

    #[test]
    fn test_nop_zp_correct_number_of_cycles() {
        let cpu = CPU::new();
        let mut bus = MockCPUBus::new();
        bus.expect_read().return_const(0x0);

        assert_eq!(3, cpu.nop_cycles(&cpu._zp(0x0, &bus)));
    }

    #[test]
    fn test_nop_zpx_correct_number_of_cycles() {
        let cpu = CPU::new();
        let mut bus = MockCPUBus::new();
        bus.expect_read().return_const(0x0);

        assert_eq!(4, cpu.nop_cycles(&cpu._zpx(0x0, &bus)));
    }

    #[test]
    fn test_nop_abs_correct_number_of_cycles() {
        let cpu = CPU::new();
        let mut bus = MockCPUBus::new();
        bus.expect_read().return_const(0x0);

        assert_eq!(4, cpu.nop_cycles(&cpu._abs(0x0, &bus)));
    }

    #[test]
    fn test_nop_absx_no_page_cross_correct_number_of_cycles() {
        let cpu = CPU::new();
        let mut bus = MockCPUBus::new();
        bus.expect_read().return_const(0x0);

        assert_eq!(4, cpu.nop_cycles(&cpu._absx(0x0, &bus)));
    }

    #[test]
    fn test_nop_absx_with_page_cross_correct_number_of_cycles() {
        let mut cpu = CPU::new();
        cpu.x = 0xff;
        let mut bus = MockCPUBus::new();
        bus.expect_read().return_const(0x0);

        assert_eq!(5, cpu.nop_cycles(&cpu._absx(0x1234, &bus)));
    }

    #[test]
    fn test_nop_does_not_crash() {
        let cpu = CPU::new();
        cpu.nop(&cpu._imp());
    }
}
