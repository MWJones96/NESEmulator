/*
    LDX - Load Index Register X From Memory
    Operation: M → X

    Load the index register X from memory.

    LDX does not affect the C or V flags; sets Z if
    the value loaded was zero, otherwise resets it;
    sets N if the value loaded in bit 7 is a 1;
    otherwise N is reset, and affects only the
    X register.
*/

use crate::cpu::addr::AddrModeResult;

use super::super::CPU;

impl CPU {
    #[inline]
    pub(in crate::cpu) fn ldx_cycles(&self, mode: &AddrModeResult) -> u8 {
        2 + mode.cycles
    }

    #[inline]
    pub(in crate::cpu) fn ldx(&mut self, mode: &AddrModeResult) {
        self.x = mode.data.unwrap();
        self.n = (self.x & 0x80) > 0;
        self.z = self.x == 0;
    }
}

#[cfg(test)]
mod ldx_tests {
    use crate::cpu::bus::MockCPUBus;

    use super::*;

    #[test]
    fn test_ldx_imm_correct_cycles() {
        let cpu = CPU::new();
        let cycles: u8 = cpu.ldx_cycles(&cpu._imm(0x0));
        assert_eq!(2, cycles);
    }

    #[test]
    fn test_ldx_zp_correct_cycles() {
        let cpu = CPU::new();
        let mut bus = MockCPUBus::new();
        bus.expect_read().return_const(0x0);

        let cycles: u8 = cpu.ldx_cycles(&cpu._zp(0x0, &bus));
        assert_eq!(3, cycles);
    }

    #[test]
    fn test_ldx_zpy_correct_cycles() {
        let cpu = CPU::new();
        let mut bus = MockCPUBus::new();
        bus.expect_read().return_const(0x0);

        let cycles: u8 = cpu.ldx_cycles(&cpu._zpy(0x0, &bus));
        assert_eq!(4, cycles);
    }

    #[test]
    fn test_ldx_abs_correct_cycles() {
        let cpu = CPU::new();
        let mut bus = MockCPUBus::new();
        bus.expect_read().return_const(0x0);

        let cycles: u8 = cpu.ldx_cycles(&cpu._abs(0x0, &bus));
        assert_eq!(4, cycles);
    }

    #[test]
    fn test_ldx_absy_correct_cycles_no_page_cross() {
        let cpu = CPU::new();
        let mut bus = MockCPUBus::new();
        bus.expect_read().return_const(0x0);

        let cycles: u8 = cpu.ldx_cycles(&cpu._absy(0x88, &bus));
        assert_eq!(4, cycles);
    }

    #[test]
    fn test_ldx_absy_correct_cycles_with_page_cross() {
        let mut cpu = CPU::new();
        let mut bus = MockCPUBus::new();
        bus.expect_read().return_const(0x0);

        cpu.y = 0xff;
        let cycles: u8 = cpu.ldx_cycles(&cpu._absy(0x88, &bus));
        assert_eq!(5, cycles);
    }

    #[test]
    fn test_ldx_value_goes_to_x_register() {
        let mut cpu = CPU::new();
        cpu.ldx(&cpu._imm(0xff));
        assert_eq!(0xff, cpu.x);
    }

    #[test]
    fn test_ldx_negative_flag() {
        let mut cpu = CPU::new();
        cpu.ldx(&cpu._imm(0x80));
        assert_eq!(true, cpu.n);
        cpu.ldx(&cpu._imm(0x7f));
        assert_eq!(false, cpu.n);
    }

    #[test]
    fn test_ldx_zero_flag() {
        let mut cpu = CPU::new();
        cpu.ldx(&cpu._imm(0x0));
        assert_eq!(true, cpu.z);
        cpu.ldx(&cpu._imm(0x1));
        assert_eq!(false, cpu.z);
    }
}
