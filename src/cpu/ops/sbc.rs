/*
    SBC - Subtract Memory from Accumulator with Borrow
    Operation: A - M - ~C → A

    This instruction subtracts the value of memory and borrow
    from the value of the accumulator, using two's complement
    arithmetic, and stores the result in the accumulator. Borrow
    is defined as the carry flag complemented; therefore, a
    resultant carry flag indicates that a borrow has not occurred.

    This instruction affects the accumulator. The carry flag is set
    if the result is greater than or equal to 0. The carry flag is
    reset when the result is less than 0, indicating a borrow.
    The over­flow flag is set when the result exceeds +127 or -128,
    otherwise it is reset. The negative flag is set if the result
    in the accumulator has bit 7 on, otherwise it is reset. The Z
    flag is set if the result in the accumulator is 0, otherwise
    it is reset.
*/

use crate::cpu::{addr::AddrModeResult, bus::CPUBus};

use super::super::NESCPU;

impl NESCPU {
    pub(in crate::cpu) fn sbcc(&self, mode: &AddrModeResult) -> u8 {
        2 + mode.cycles
    }

    pub(in crate::cpu) fn sbc(&mut self, mode: &AddrModeResult, bus: &mut dyn CPUBus) {
        self.adc(&self._imm(!mode.data.unwrap()), bus)
    }
}

#[cfg(test)]
mod sbc_tests {
    use mockall::predicate::eq;

    use crate::cpu::bus::MockCPUBus;

    use super::*;

    #[test]
    fn test_sbc_imm_correctc() {
        let cpu = NESCPU::new();
        let cycles: u8 = cpu.sbcc(&cpu._imm(0x0));
        assert_eq!(2, cycles);
    }

    #[test]
    fn test_sbc_zp_correctc() {
        let cpu = NESCPU::new();
        let mut bus = MockCPUBus::new();
        bus.expect_read().return_const(0x0);

        let cycles: u8 = cpu.sbcc(&cpu._zp(0x0, &bus));
        assert_eq!(3, cycles);
    }

    #[test]
    fn test_sbc_zpx_correctc() {
        let cpu = NESCPU::new();
        let mut bus = MockCPUBus::new();
        bus.expect_read().return_const(0x0);

        let cycles: u8 = cpu.sbcc(&cpu._zpx(0x0, &bus));
        assert_eq!(4, cycles);
    }

    #[test]
    fn test_sbc_abs_correctc() {
        let cpu = NESCPU::new();
        let mut bus = MockCPUBus::new();
        bus.expect_read().return_const(0x0);

        let cycles: u8 = cpu.sbcc(&cpu._abs(0x0, &bus));
        assert_eq!(4, cycles);
    }

    #[test]
    fn test_sbc_absx_correct_cycles_no_page_cross() {
        let cpu = NESCPU::new();
        let mut bus = MockCPUBus::new();
        bus.expect_read().return_const(0x0);

        let cycles: u8 = cpu.sbcc(&cpu._absx(0x0, &bus));
        assert_eq!(4, cycles);
    }

    #[test]
    fn test_sbc_absx_correct_cycles_with_page_cross() {
        let mut cpu = NESCPU::new();
        let mut bus = MockCPUBus::new();
        bus.expect_read().return_const(0x0);
        cpu.x = 0xff;

        let cycles: u8 = cpu.sbcc(&cpu._absx(0x88, &bus));
        assert_eq!(5, cycles);
    }

    #[test]
    fn test_sbc_absy_correct_cycles_no_page_cross() {
        let cpu = NESCPU::new();
        let mut bus = MockCPUBus::new();
        bus.expect_read().return_const(0x0);

        let cycles: u8 = cpu.sbcc(&cpu._absy(0x88, &bus));
        assert_eq!(4, cycles);
    }

    #[test]
    fn test_sbc_absy_correct_cycles_with_page_cross() {
        let mut cpu = NESCPU::new();
        let mut bus = MockCPUBus::new();
        bus.expect_read().return_const(0x0);

        cpu.y = 0xff;
        let cycles: u8 = cpu.sbcc(&cpu._absy(0x88, &bus));
        assert_eq!(5, cycles);
    }

    #[test]
    fn test_sbc_indx_correctc() {
        let cpu = NESCPU::new();
        let mut bus = MockCPUBus::new();
        bus.expect_read().return_const(0x0);

        let cycles: u8 = cpu.sbcc(&cpu._indx(0x88, &bus));
        assert_eq!(6, cycles);
    }

    #[test]
    fn test_sbc_indy_correct_cycles_no_page_cross() {
        let cpu = NESCPU::new();
        let mut bus = MockCPUBus::new();
        bus.expect_read().return_const(0x0);

        let cycles: u8 = cpu.sbcc(&cpu._indy(0x88, &bus));
        assert_eq!(5, cycles);
    }

    #[test]
    fn test_sbc_indy_correct_cycles_with_page_cross() {
        let mut cpu = NESCPU::new();
        let mut bus = MockCPUBus::new();
        bus.expect_read().with(eq(0x88)).return_const(0x11);
        bus.expect_read().with(eq(0x89)).return_const(0x22);

        bus.expect_read().with(eq(0x2310)).return_const(0x0);

        cpu.y = 0xff;
        let cycles: u8 = cpu.sbcc(&cpu._indy(0x88, &bus));
        assert_eq!(6, cycles);
    }

    #[test]
    fn test_sbc_no_borrow() {
        let mut cpu = NESCPU::new();

        cpu.c = true; //No borrow
        cpu.a = 0x1;
        cpu.sbc(&cpu._imm(0x2), &mut MockCPUBus::new());

        assert_eq!(0xff, cpu.a);
    }

    #[test]
    fn test_sbc_with_borrow() {
        let mut cpu = NESCPU::new();

        cpu.c = false; //Borrow
        cpu.a = 0x1;
        cpu.sbc(&cpu._imm(0x2), &mut MockCPUBus::new());

        assert_eq!(0xfe, cpu.a);
    }

    #[test]
    fn test_sbc_with_negative_flag() {
        let mut cpu = NESCPU::new();

        cpu.c = true; //No borrow
        cpu.a = 0x1;
        cpu.sbc(&cpu._imm(0x2), &mut MockCPUBus::new());

        assert_eq!(true, cpu.n);
    }

    #[test]
    fn test_sbc_with_zero_flag() {
        let mut cpu = NESCPU::new();

        cpu.c = true; //No borrow
        cpu.a = 0x1;
        cpu.sbc(&cpu._imm(0x1), &mut MockCPUBus::new());

        assert_eq!(true, cpu.z);
    }

    #[test]
    fn test_sbc_with_carry_flag() {
        let mut cpu = NESCPU::new();

        cpu.c = true; //No borrow
        cpu.a = 0x1;
        cpu.sbc(&cpu._imm(0x2), &mut MockCPUBus::new());

        assert_eq!(false, cpu.c);

        cpu.c = true; //No borrow
        cpu.a = 0x1;
        cpu.sbc(&cpu._imm(0x1), &mut MockCPUBus::new());

        assert_eq!(true, cpu.c);

        cpu.c = true; //No borrow
        cpu.a = 0x2;
        cpu.sbc(&cpu._imm(0x1), &mut MockCPUBus::new());

        assert_eq!(true, cpu.c);
    }

    #[test]
    fn test_sbc_with_overflow_flag() {
        let mut cpu = NESCPU::new();

        cpu.c = true;
        cpu.a = 0x7f;

        cpu.sbc(&cpu._imm(0xff), &mut MockCPUBus::new());

        assert_eq!(true, cpu.v);

        cpu.c = true;
        cpu.a = 0x80;

        cpu.sbc(&cpu._imm(0x1), &mut MockCPUBus::new());
        assert_eq!(true, cpu.v);

        cpu.c = true;
        cpu.a = 0x1;

        cpu.sbc(&cpu._imm(0x1), &mut MockCPUBus::new());
        assert_eq!(false, cpu.v);
    }
}
