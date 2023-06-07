/*
    CMP - Compare Memory and Accumulator
    Operation: A - M

    This instruction subtracts the contents of memory
    from the contents of the accumulator.

    The use of the CMP affects the following flags:
    Z flag is set on an equal comparison, reset otherwise;
    the N flag is set or reset by the result bit 7,
    the carry flag is set when the value in memory is
    less than or equal to the accumulator, reset when
    it is greater than the accumulator.

    The accumulator is not affected.
*/

use crate::cpu::addr::AddrModeResult;

use super::super::CPU;

impl CPU {
    #[inline]
    pub(in crate::cpu) fn cmpc(&self, mode: &AddrModeResult) -> u8 {
        2 + mode.cycles
    }

    #[inline]
    pub(in crate::cpu) fn cmp(&mut self, mode: &AddrModeResult) {
        let data = mode.data.unwrap();
        let result = self.a.wrapping_add(!data).wrapping_add(1);

        self.n = (result & 0x80) > 0;
        self.z = self.a == data;
        self.c = self.a >= data;
    }
}

#[cfg(test)]
mod cmp_tests {
    use crate::cpu::bus::MockCPUBus;

    use super::*;

    #[test]
    fn test_cmp_imm_correct_number_ofc() {
        let cpu = CPU::new();

        assert_eq!(2, cpu.cmpc(&cpu._imm(0x0)));
    }

    #[test]
    fn test_cmp_zp_correct_number_ofc() {
        let cpu = CPU::new();
        let mut bus = MockCPUBus::new();
        bus.expect_read().return_const(0x0);

        assert_eq!(3, cpu.cmpc(&cpu._zp(0x0, &bus)));
    }

    #[test]
    fn test_cmp_zpx_correct_number_ofc() {
        let cpu = CPU::new();
        let mut bus = MockCPUBus::new();
        bus.expect_read().return_const(0x0);

        assert_eq!(4, cpu.cmpc(&cpu._zpx(0x0, &bus)));
    }

    #[test]
    fn test_cmp_abs_correct_number_ofc() {
        let cpu = CPU::new();
        let mut bus = MockCPUBus::new();
        bus.expect_read().return_const(0x0);

        assert_eq!(4, cpu.cmpc(&cpu._abs(0x0, &bus)));
    }

    #[test]
    fn test_cmp_absx_no_page_cross_correct_number_ofc() {
        let cpu = CPU::new();
        let mut bus = MockCPUBus::new();
        bus.expect_read().return_const(0x0);

        assert_eq!(4, cpu.cmpc(&cpu._absx(0x0, &bus)));
    }

    #[test]
    fn test_cmp_absx_with_page_cross_correct_number_ofc() {
        let mut cpu = CPU::new();
        let mut bus = MockCPUBus::new();
        cpu.x = 0xff;

        bus.expect_read().return_const(0x0);

        assert_eq!(5, cpu.cmpc(&cpu._absx(0x1234, &bus)));
    }

    #[test]
    fn test_cmp_absy_no_page_cross_correct_number_ofc() {
        let cpu = CPU::new();
        let mut bus = MockCPUBus::new();
        bus.expect_read().return_const(0x0);

        assert_eq!(4, cpu.cmpc(&cpu._absy(0x0, &bus)));
    }

    #[test]
    fn test_cmp_absy_with_page_cross_correct_number_ofc() {
        let mut cpu = CPU::new();
        let mut bus = MockCPUBus::new();
        cpu.y = 0xff;

        bus.expect_read().return_const(0x0);

        assert_eq!(5, cpu.cmpc(&cpu._absy(0x1234, &bus)));
    }

    #[test]
    fn test_cmp_indx_correct_number_ofc() {
        let cpu = CPU::new();
        let mut bus = MockCPUBus::new();

        bus.expect_read().return_const(0x0);

        assert_eq!(6, cpu.cmpc(&cpu._indx(0x0, &bus)));
    }

    #[test]
    fn test_cmp_indy_no_page_cross_correct_number_ofc() {
        let cpu = CPU::new();
        let mut bus = MockCPUBus::new();

        bus.expect_read().return_const(0x0);

        assert_eq!(5, cpu.cmpc(&cpu._indy(0x0, &bus)));
    }

    #[test]
    fn test_cmp_indy_with_page_cross_correct_number_ofc() {
        let mut cpu = CPU::new();
        let mut bus = MockCPUBus::new();
        cpu.y = 0xff;

        bus.expect_read().return_const(0x80);

        assert_eq!(6, cpu.cmpc(&cpu._indy(0x0, &bus)));
    }

    #[test]
    fn test_cmp_negative_flag() {
        let mut cpu = CPU::new();

        cpu.a = 0x10;
        cpu.cmp(&cpu._imm(0x11));

        assert_eq!(true, cpu.n);
        assert_eq!(0x10, cpu.a);
    }

    #[test]
    fn test_cmp_zero_flag() {
        let mut cpu = CPU::new();

        cpu.a = 0x20;
        cpu.cmp(&cpu._imm(0x20));

        assert_eq!(true, cpu.z);
        assert_eq!(0x20, cpu.a);
    }

    #[test]
    fn test_cmp_carry_flag() {
        let mut cpu = CPU::new();

        cpu.a = 0x20;
        cpu.cmp(&cpu._imm(0x20));

        assert_eq!(true, cpu.c);
        assert_eq!(0x20, cpu.a);

        cpu.a = 0x20;
        cpu.cmp(&cpu._imm(0x10));

        assert_eq!(true, cpu.c);
        assert_eq!(0x20, cpu.a);

        cpu.a = 0x20;
        cpu.cmp(&cpu._imm(0x21));

        assert_eq!(false, cpu.c);
        assert_eq!(0x20, cpu.a);
    }
}
