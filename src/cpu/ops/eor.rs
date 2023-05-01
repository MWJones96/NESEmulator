/*
    EOR - "Exclusive OR" Memory with Accumulator
    Operation: A ⊻ M → A

    The EOR instruction transfers the memory and the
    accumulator to the adder which performs a binary
    "EXCLUSIVE OR" on a bit-by-bit basis and stores
    the result in the accumulator.

    This instruction affects the accumulator; sets
    the zero flag if the result in the accumulator
    is 0, otherwise resets the zero flag sets the
    negative flag if the result in the accumulator
    has bit 7 on, otherwise resets the negative flag.
*/

use crate::cpu::{addr::AddrModeResult, CPU};

impl CPU {
    pub(in crate::cpu) fn eor_cycles(&self, mode: &AddrModeResult) -> u8 {
        2 + mode.cycles
    }

    pub(in crate::cpu) fn eor(&mut self, mode: &AddrModeResult) {
        self.a = self.a ^ mode.data.unwrap();

        self.n = (self.a & 0x80) > 0;
        self.z = self.a == 0;
    }
}

#[cfg(test)]
mod eor_tests {
    use crate::cpu::bus::MockCPUBus;

    use super::*;

    #[test]
    fn test_eor_imm_correct_number_of_cycles() {
        let mut cpu = CPU::new();
        assert_eq!(2, cpu.eor_cycles(&cpu.imm(0x0)));
    }

    #[test]
    fn test_eor_zp_correct_number_of_cycles() {
        let mut cpu = CPU::new();
        let mut bus = MockCPUBus::new();

        bus.expect_read().return_const(0x0);

        assert_eq!(3, cpu.eor_cycles(&cpu.zp(0x0, &bus)));
    }

    #[test]
    fn test_eor_zpx_correct_number_of_cycles() {
        let mut cpu = CPU::new();
        let mut bus = MockCPUBus::new();

        bus.expect_read().return_const(0x0);

        assert_eq!(4, cpu.eor_cycles(&cpu.zpx(0x0, &bus)));
    }

    #[test]
    fn test_eor_abs_correct_number_of_cycles() {
        let mut cpu = CPU::new();
        let mut bus = MockCPUBus::new();

        bus.expect_read().return_const(0x0);

        assert_eq!(4, cpu.eor_cycles(&cpu.abs(0x0, &bus)));
    }

    #[test]
    fn test_eor_absx_no_page_cross_correct_number_of_cycles() {
        let mut cpu = CPU::new();
        let mut bus = MockCPUBus::new();

        bus.expect_read().return_const(0x0);

        assert_eq!(4, cpu.eor_cycles(&cpu.absx(0x0, &bus)));
    }

    #[test]
    fn test_eor_absx_with_page_cross_correct_number_of_cycles() {
        let mut cpu = CPU::new();
        let mut bus = MockCPUBus::new();
        cpu.x = 0xff;

        bus.expect_read().return_const(0x0);

        assert_eq!(5, cpu.eor_cycles(&cpu.absx(0x1234, &bus)));
    }

    #[test]
    fn test_eor_absy_no_page_cross_correct_number_of_cycles() {
        let mut cpu = CPU::new();
        let mut bus = MockCPUBus::new();

        bus.expect_read().return_const(0x0);

        assert_eq!(4, cpu.eor_cycles(&cpu.absy(0x0, &bus)));
    }

    #[test]
    fn test_eor_absy_with_page_cross_correct_number_of_cycles() {
        let mut cpu = CPU::new();
        let mut bus = MockCPUBus::new();
        cpu.y = 0xff;

        bus.expect_read().return_const(0x0);

        assert_eq!(5, cpu.eor_cycles(&cpu.absy(0x1234, &bus)));
    }

    #[test]
    fn test_eor_indx_correct_number_of_cycles() {
        let mut cpu = CPU::new();
        let mut bus = MockCPUBus::new();

        bus.expect_read().return_const(0x0);

        assert_eq!(6, cpu.eor_cycles(&cpu.indx(0x0, &bus)));
    }

    #[test]
    fn test_eor_indy_no_page_cross_correct_number_of_cycles() {
        let mut cpu = CPU::new();
        let mut bus = MockCPUBus::new();

        bus.expect_read().return_const(0x0);

        assert_eq!(5, cpu.eor_cycles(&cpu.indy(0x0, &bus)));
    }

    #[test]
    fn test_eor_indy_with_page_cross_correct_number_of_cycles() {
        let mut cpu = CPU::new();
        let mut bus = MockCPUBus::new();
        cpu.y = 0xff;

        bus.expect_read().return_const(0x80);

        assert_eq!(6, cpu.eor_cycles(&cpu.indy(0x0, &bus)));
    }

    #[test]
    fn test_eor() {
        let mut cpu = CPU::new();

        cpu.a = 0b1111_0000;
        cpu.eor(&cpu.imm(0b1010_1010));

        assert_eq!(0b0101_1010, cpu.a);
    }

    #[test]
    fn test_eor_negative_flag() {
        let mut cpu = CPU::new();

        cpu.a = 0b1111_1111;
        cpu.eor(&cpu.imm(0b0000_0000));

        assert_eq!(true, cpu.n);
    }

    #[test]
    fn test_eor_zero_flag() {
        let mut cpu = CPU::new();

        cpu.a = 0xff;
        cpu.eor(&cpu.imm(0xff));

        assert_eq!(true, cpu.z);
    }
}
