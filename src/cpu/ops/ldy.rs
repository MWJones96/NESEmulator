/*
    LDY - Load Index Register Y From Memory
    Operation: M → Y

    Load the index register Y from memory.

    LDY does not affect the C or V flags, sets the
    N flag if the value loaded in bit 7 is a 1,
    otherwise resets N, sets Z flag if the loaded
    value is zero otherwise resets Z and only affects
    the Y register.
*/

use crate::cpu::addr::AddrModeResult;

use super::super::CPU;

impl CPU {
    #[inline]
    pub(in crate::cpu) fn ldy_cycles(&self, mode: &AddrModeResult) -> u8 {
        2 + mode.cycles
    }

    #[inline]
    pub(in crate::cpu) fn ldy(&mut self, mode: &AddrModeResult) {
        self.y = mode.data.unwrap();
        self.n = (self.y & 0x80) > 0;
        self.z = self.y == 0;
    }
}

#[cfg(test)]
mod ldy_tests {
    use crate::cpu::bus::MockCPUBus;

    use super::*;

    #[test]
    fn test_ldy_imm_correct_cycles() {
        let cpu = CPU::new();
        let cycles: u8 = cpu.ldy_cycles(&cpu._imm(0x0));
        assert_eq!(2, cycles);
    }

    #[test]
    fn test_ldy_zp_correct_cycles() {
        let cpu = CPU::new();
        let mut bus = MockCPUBus::new();
        bus.expect_read().return_const(0x0);

        let cycles: u8 = cpu.ldy_cycles(&cpu._zp(0x0, &bus));
        assert_eq!(3, cycles);
    }

    #[test]
    fn test_ldy_zpx_correct_cycles() {
        let cpu = CPU::new();
        let mut bus = MockCPUBus::new();
        bus.expect_read().return_const(0x0);

        let cycles: u8 = cpu.ldy_cycles(&cpu._zpx(0x0, &bus));
        assert_eq!(4, cycles);
    }

    #[test]
    fn test_ldy_abs_correct_cycles() {
        let cpu = CPU::new();
        let mut bus = MockCPUBus::new();
        bus.expect_read().return_const(0x0);

        let cycles: u8 = cpu.ldy_cycles(&cpu._abs(0x0, &bus));
        assert_eq!(4, cycles);
    }

    #[test]
    fn test_ldy_absx_correct_cycles_no_page_cross() {
        let cpu = CPU::new();
        let mut bus = MockCPUBus::new();
        bus.expect_read().return_const(0x0);

        let cycles: u8 = cpu.ldy_cycles(&cpu._absx(0x88, &bus));
        assert_eq!(4, cycles);
    }

    #[test]
    fn test_ldy_absx_correct_cycles_with_page_cross() {
        let mut cpu = CPU::new();
        let mut bus = MockCPUBus::new();
        bus.expect_read().return_const(0x0);

        cpu.x = 0xff;
        let cycles: u8 = cpu.ldy_cycles(&cpu._absx(0x88, &bus));
        assert_eq!(5, cycles);
    }

    #[test]
    fn test_ldy_value_goes_to_y_register() {
        let mut cpu = CPU::new();
        cpu.ldy(&cpu._imm(0xff));
        assert_eq!(0xff, cpu.y);
    }

    #[test]
    fn test_ldy_negative_flag() {
        let mut cpu = CPU::new();
        cpu.ldy(&cpu._imm(0x80));
        assert_eq!(true, cpu.n);
        cpu.ldy(&cpu._imm(0x7f));
        assert_eq!(false, cpu.n);
    }

    #[test]
    fn test_ldy_zero_flag() {
        let mut cpu = CPU::new();
        cpu.ldy(&cpu._imm(0x0));
        assert_eq!(true, cpu.z);
        cpu.ldy(&cpu._imm(0x1));
        assert_eq!(false, cpu.z);
    }
}
