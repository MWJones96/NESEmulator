/*
    LDA - Load Accumulator with Memory
    Operation: M → A

    When instruction LDA is executed by the microprocessor,
    data is transferred from memory to the accumulator and
    stored in the accumulator.

    LDA affects the contents of the accumulator, does not
    affect the carry or overflow flags; sets the zero flag
    if the accumulator is zero as a result of the LDA,
    otherwise resets the zero flag; sets the negative flag
    if bit 7 of the accumulator is a 1, other­ wise resets
    the negative flag.
*/

use crate::cpu::{addr::AddrModeResult, bus::CPUBus};

use super::super::CPU;

impl CPU {
    #[inline]
    pub(in crate::cpu) fn ldac(&self, mode: &AddrModeResult) -> u8 {
        2 + mode.cycles
    }

    #[inline]
    pub(in crate::cpu) fn lda(&mut self, mode: &AddrModeResult, _bus: &mut dyn CPUBus) {
        self.a = mode.data.unwrap();
        self.n = (self.a & 0x80) > 0;
        self.z = self.a == 0;
    }
}

#[cfg(test)]
mod lda_tests {
    use mockall::predicate::eq;

    use crate::cpu::bus::MockCPUBus;

    use super::*;

    #[test]
    fn test_lda_imm_correctc() {
        let cpu = CPU::new();
        let cycles: u8 = cpu.ldac(&cpu._imm(0x0));
        assert_eq!(2, cycles);
    }

    #[test]
    fn test_lda_zp_correctc() {
        let cpu = CPU::new();
        let mut bus = MockCPUBus::new();
        bus.expect_read().return_const(0x0);

        let cycles: u8 = cpu.ldac(&cpu._zp(0x0, &bus));
        assert_eq!(3, cycles);
    }

    #[test]
    fn test_lda_zpx_correctc() {
        let cpu = CPU::new();
        let mut bus = MockCPUBus::new();
        bus.expect_read().return_const(0x0);

        let cycles: u8 = cpu.ldac(&cpu._zpx(0x0, &bus));
        assert_eq!(4, cycles);
    }

    #[test]
    fn test_lda_abs_correctc() {
        let cpu = CPU::new();
        let mut bus = MockCPUBus::new();
        bus.expect_read().return_const(0x0);

        let cycles: u8 = cpu.ldac(&cpu._abs(0x0, &bus));
        assert_eq!(4, cycles);
    }

    #[test]
    fn test_lda_absx_correct_cycles_no_page_cross() {
        let cpu = CPU::new();
        let mut bus = MockCPUBus::new();
        bus.expect_read().return_const(0x0);

        let cycles: u8 = cpu.ldac(&cpu._absx(0x0, &bus));
        assert_eq!(4, cycles);
    }

    #[test]
    fn test_lda_absx_correct_cycles_with_page_cross() {
        let mut cpu = CPU::new();
        let mut bus = MockCPUBus::new();
        bus.expect_read().return_const(0x0);
        cpu.x = 0xff;

        let cycles: u8 = cpu.ldac(&cpu._absx(0x88, &bus));
        assert_eq!(5, cycles);
    }

    #[test]
    fn test_lda_absy_correct_cycles_no_page_cross() {
        let cpu = CPU::new();
        let mut bus = MockCPUBus::new();
        bus.expect_read().return_const(0x0);

        let cycles: u8 = cpu.ldac(&cpu._absy(0x88, &bus));
        assert_eq!(4, cycles);
    }

    #[test]
    fn test_lda_absy_correct_cycles_with_page_cross() {
        let mut cpu = CPU::new();
        let mut bus = MockCPUBus::new();
        bus.expect_read().return_const(0x0);

        cpu.y = 0xff;
        let cycles: u8 = cpu.ldac(&cpu._absy(0x88, &bus));
        assert_eq!(5, cycles);
    }

    #[test]
    fn test_lda_indx_correctc() {
        let cpu = CPU::new();
        let mut bus = MockCPUBus::new();
        bus.expect_read().return_const(0x0);

        let cycles: u8 = cpu.ldac(&cpu._indx(0x88, &bus));
        assert_eq!(6, cycles);
    }

    #[test]
    fn test_lda_indy_correct_cycles_no_page_cross() {
        let cpu = CPU::new();
        let mut bus = MockCPUBus::new();
        bus.expect_read().return_const(0x0);

        let cycles: u8 = cpu.ldac(&cpu._indy(0x88, &bus));
        assert_eq!(5, cycles);
    }

    #[test]
    fn test_lda_indy_correct_cycles_with_page_cross() {
        let mut cpu = CPU::new();
        let mut bus = MockCPUBus::new();
        bus.expect_read().with(eq(0x88)).return_const(0x11);
        bus.expect_read().with(eq(0x89)).return_const(0x22);

        bus.expect_read().with(eq(0x2310)).return_const(0x0);

        cpu.y = 0xff;
        let cycles: u8 = cpu.ldac(&cpu._indy(0x88, &bus));
        assert_eq!(6, cycles);
    }

    #[test]
    fn test_lda_value_goes_to_accumulator() {
        let mut cpu = CPU::new();
        cpu.lda(&cpu._imm(0xff), &mut MockCPUBus::new());
        assert_eq!(0xff, cpu.a);
    }

    #[test]
    fn test_lda_negative_flag() {
        let mut cpu = CPU::new();
        cpu.lda(&cpu._imm(0x80), &mut MockCPUBus::new());
        assert_eq!(true, cpu.n);
        cpu.lda(&cpu._imm(0x7f), &mut MockCPUBus::new());
        assert_eq!(false, cpu.n);
    }

    #[test]
    fn test_lda_zero_flag() {
        let mut cpu = CPU::new();
        cpu.lda(&cpu._imm(0x0), &mut MockCPUBus::new());
        assert_eq!(true, cpu.z);
        cpu.lda(&cpu._imm(0x1), &mut MockCPUBus::new());
        assert_eq!(false, cpu.z);
    }
}
