/*
    AND - "AND" Memory with Accumulator

    Operation: A ∧ M → A

    The AND instruction transfer the accumulator and memory to the adder
    which performs a bit-by-bit AND operation and stores the result back
    in the accumulator.

    This instruction affects the accumulator; sets the zero flag if the
    result in the accumulator is 0, otherwise resets the zero flag;
    sets the negative flag if the result in the accumulator has bit 7 on,
    otherwise resets the negative flag.
*/

use crate::cpu::addr::AddrModeResult;

use super::super::CPU;

impl CPU {
    pub(in crate::cpu) fn and_cycles(&self, mode: &AddrModeResult) -> u8 {
        2 + mode.cycles
    }

    pub(in crate::cpu) fn and(&mut self, mode: &AddrModeResult) {
        self.a &= mode.data.unwrap();

        self.z = self.a == 0;
        self.n = (self.a & 0x80) > 0;
    }
}

#[cfg(test)]
mod and_tests {
    use mockall::predicate::eq;

    use crate::cpu::bus::MockBus;

    use super::*;

    #[test]
    fn test_and_imm_correct_cycles() {
        let mut cpu = CPU::new();
        assert_eq!(2, cpu.and_cycles(&cpu.imm(0xff)));
    }

    #[test]
    fn test_and_zp_correct_cycles() {
        let mut cpu = CPU::new();
        let mut bus = MockBus::new();
        bus.expect_read().return_const(0x0);

        assert_eq!(3, cpu.and_cycles(&cpu.zp(0xff, &bus)));
    }

    #[test]
    fn test_and_zpx_correct_cycles() {
        let mut cpu = CPU::new();
        let mut bus = MockBus::new();
        bus.expect_read().return_const(0x0);

        assert_eq!(4, cpu.and_cycles(&cpu.zpx(0xff, &bus)));
    }

    #[test]
    fn test_and_abs_correct_cycles() {
        let mut cpu = CPU::new();
        let mut bus = MockBus::new();
        bus.expect_read().return_const(0x0);

        assert_eq!(4, cpu.and_cycles(&cpu.abs(0xff, &bus)));
    }

    #[test]
    fn test_and_absx_correct_cycles_no_page_cross() {
        let mut cpu = CPU::new();
        let mut bus = MockBus::new();
        bus.expect_read().return_const(0x0);

        assert_eq!(4, cpu.and_cycles(&cpu.absx(0xff, &bus)));
    }

    #[test]
    fn test_and_absx_correct_cycles_with_page_cross() {
        let mut cpu = CPU::new();
        let mut bus = MockBus::new();
        bus.expect_read().return_const(0x0);

        cpu.x = 0xff;
        assert_eq!(5, cpu.and_cycles(&cpu.absx(0xff, &bus)));
    }

    #[test]
    fn test_and_absy_correct_cycles_no_page_cross() {
        let mut cpu = CPU::new();
        let mut bus = MockBus::new();
        bus.expect_read().return_const(0x0);

        assert_eq!(4, cpu.and_cycles(&cpu.absy(0xff, &bus)));
    }

    #[test]
    fn test_and_absy_correct_cycles_with_page_cross() {
        let mut cpu = CPU::new();
        let mut bus = MockBus::new();
        bus.expect_read().return_const(0x0);

        cpu.y = 0xff;
        assert_eq!(5, cpu.and_cycles(&cpu.absy(0xff, &bus)));
    }

    #[test]
    fn test_and_indx_correct_cycles() {
        let mut cpu = CPU::new();
        let mut bus = MockBus::new();
        bus.expect_read().return_const(0x0);

        assert_eq!(6, cpu.and_cycles(&cpu.indx(0xff, &bus)));
    }

    #[test]
    fn test_and_indy_correct_cycles_no_page_cross() {
        let mut cpu = CPU::new();
        let mut bus = MockBus::new();
        bus.expect_read().return_const(0x0);

        assert_eq!(5, cpu.and_cycles(&cpu.indy(0xff, &bus)));
    }

    #[test]
    fn test_and_indy_correct_cycles_with_page_cross() {
        let mut cpu = CPU::new();
        let mut bus = MockBus::new();
        bus.expect_read().with(eq(0x88)).return_const(0x11);
        bus.expect_read().with(eq(0x89)).return_const(0x22);

        bus.expect_read().with(eq(0x2310)).return_const(0x0);

        cpu.y = 0xff;
        assert_eq!(6, cpu.and_cycles(&cpu.indy(0x88, &bus)));
    }

    #[test]
    fn test_and() {
        let mut cpu = CPU::new();
        cpu.a = 0b1010_1010_u8;

        cpu.and(&cpu.imm(0b0101_0101_u8));

        assert_eq!(0x0, cpu.a);
    }

    #[test]
    fn test_and_all_ones() {
        let mut cpu = CPU::new();
        cpu.a = 0xff;

        cpu.and(&cpu.imm(0xff));

        assert_eq!(0xff, cpu.a);
    }

    #[test]
    fn test_and_half_ones() {
        let mut cpu = CPU::new();
        cpu.a = 0b0000_1111_u8;

        cpu.and(&cpu.imm(0b0000_1111_u8));

        assert_eq!(0xf, cpu.a);
    }

    #[test]
    fn test_and_zero_flag() {
        let mut cpu = CPU::new();
        cpu.a = 0b0000_1111_u8;

        cpu.and(&cpu.imm(0b0000_1111_u8));

        assert_eq!(false, cpu.z);

        cpu.and(&cpu.imm(0b0000_0000_u8));

        assert_eq!(true, cpu.z);
    }

    #[test]
    fn test_and_negative_flag() {
        let mut cpu = CPU::new();
        cpu.a = 0xff;

        cpu.and(&cpu.imm(0xff));

        assert_eq!(true, cpu.n)
    }
}
