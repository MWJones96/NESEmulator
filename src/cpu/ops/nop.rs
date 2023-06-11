/*
    NOP - No Operation
    Operation: No operation
*/

use crate::{bus::Bus, cpu::addr::AddrModeResult};

use super::super::NESCPU;

impl NESCPU {
    pub(in crate::cpu) fn nopc(&self, mode: &AddrModeResult) -> u8 {
        2 + mode.cycles
    }

    pub(in crate::cpu) fn nop(&mut self, _mode: &AddrModeResult, _bus: &mut dyn Bus) {
        //No operation
    }
}

#[cfg(test)]
mod nop_tests {
    use crate::bus::MockBus;

    use super::*;

    #[test]
    fn test_nop_imp_correct_number_of_cycles() {
        let cpu = NESCPU::new();
        assert_eq!(2, cpu.nopc(&cpu._imp()));
    }

    #[test]
    fn test_nop_imm_correct_number_of_cycles() {
        let cpu = NESCPU::new();
        assert_eq!(2, cpu.nopc(&cpu._imm(0x0)));
    }

    #[test]
    fn test_nop_zp_correct_number_of_cycles() {
        let cpu = NESCPU::new();
        let mut bus = MockBus::new();
        bus.expect_read().return_const(0x0);

        assert_eq!(3, cpu.nopc(&cpu._zp(0x0, &bus)));
    }

    #[test]
    fn test_nop_zpx_correct_number_of_cycles() {
        let cpu = NESCPU::new();
        let mut bus = MockBus::new();
        bus.expect_read().return_const(0x0);

        assert_eq!(4, cpu.nopc(&cpu._zpx(0x0, &bus)));
    }

    #[test]
    fn test_nop_abs_correct_number_of_cycles() {
        let cpu = NESCPU::new();
        let mut bus = MockBus::new();
        bus.expect_read().return_const(0x0);

        assert_eq!(4, cpu.nopc(&cpu._abs(0x0, &bus)));
    }

    #[test]
    fn test_nop_absx_no_page_cross_correct_number_of_cycles() {
        let cpu = NESCPU::new();
        let mut bus = MockBus::new();
        bus.expect_read().return_const(0x0);

        assert_eq!(4, cpu.nopc(&cpu._absx(0x0, &bus)));
    }

    #[test]
    fn test_nop_absx_with_page_cross_correct_number_of_cycles() {
        let mut cpu = NESCPU::new();
        cpu.x = 0xff;
        let mut bus = MockBus::new();
        bus.expect_read().return_const(0x0);

        assert_eq!(5, cpu.nopc(&cpu._absx(0x1234, &bus)));
    }

    #[test]
    fn test_nop_does_not_crash() {
        let mut cpu = NESCPU::new();
        cpu.nop(&cpu._imp(), &mut MockBus::new());
    }
}
