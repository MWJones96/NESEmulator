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

use crate::{bus::Bus, cpu::addr::AddrModeResult};

use super::super::NESCPU;

impl NESCPU {
    pub(in crate::cpu) fn andc(&self, mode: &AddrModeResult) -> u8 {
        2 + mode.cycles
    }

    pub(in crate::cpu) fn and(&mut self, mode: &AddrModeResult, bus: &mut dyn Bus) {
        if let Some(addr) = mode.addr {
            self.a &= bus.read(addr);
        } else {
            self.a &= mode.data.unwrap();
        }

        self.z = self.a == 0;
        self.n = (self.a & 0x80) != 0;
    }
}

#[cfg(test)]
mod and_tests {
    use mockall::predicate::eq;

    use crate::bus::MockBus;

    use super::*;

    #[test]
    fn test_and_imm_correctc() {
        let cpu = NESCPU::new();
        assert_eq!(2, cpu.andc(&cpu._imm(0xff)));
    }

    #[test]
    fn test_and_zp_correctc() {
        let cpu = NESCPU::new();
        let mut bus = MockBus::new();
        bus.expect_read().return_const(0x0);

        assert_eq!(3, cpu.andc(&cpu._zp(0xff, &bus)));
    }

    #[test]
    fn test_and_zpx_correctc() {
        let cpu = NESCPU::new();
        let mut bus = MockBus::new();
        bus.expect_read().return_const(0x0);

        assert_eq!(4, cpu.andc(&cpu._zpx(0xff, &bus)));
    }

    #[test]
    fn test_and_abs_correctc() {
        let cpu = NESCPU::new();
        let mut bus = MockBus::new();
        bus.expect_read().return_const(0x0);

        assert_eq!(4, cpu.andc(&cpu._abs(0xff, &bus)));
    }

    #[test]
    fn test_and_absx_correct_cycles_no_page_cross() {
        let cpu = NESCPU::new();
        let mut bus = MockBus::new();
        bus.expect_read().return_const(0x0);

        assert_eq!(4, cpu.andc(&cpu._absx(0xff, &bus)));
    }

    #[test]
    fn test_and_absx_correct_cycles_with_page_cross() {
        let mut cpu = NESCPU::new();
        let mut bus = MockBus::new();
        bus.expect_read().return_const(0x0);

        cpu.x = 0xff;
        assert_eq!(5, cpu.andc(&cpu._absx(0xff, &bus)));
    }

    #[test]
    fn test_and_absy_correct_cycles_no_page_cross() {
        let cpu = NESCPU::new();
        let mut bus = MockBus::new();
        bus.expect_read().return_const(0x0);

        assert_eq!(4, cpu.andc(&cpu._absy(0xff, &bus)));
    }

    #[test]
    fn test_and_absy_correct_cycles_with_page_cross() {
        let mut cpu = NESCPU::new();
        let mut bus = MockBus::new();
        bus.expect_read().return_const(0x0);

        cpu.y = 0xff;
        assert_eq!(5, cpu.andc(&cpu._absy(0xff, &bus)));
    }

    #[test]
    fn test_and_indx_correctc() {
        let cpu = NESCPU::new();
        let mut bus = MockBus::new();
        bus.expect_read().return_const(0x0);

        assert_eq!(6, cpu.andc(&cpu._indx(0xff, &bus)));
    }

    #[test]
    fn test_and_indy_correct_cycles_no_page_cross() {
        let cpu = NESCPU::new();
        let mut bus = MockBus::new();
        bus.expect_read().return_const(0x0);

        assert_eq!(5, cpu.andc(&cpu._indy(0xff, &bus)));
    }

    #[test]
    fn test_and_indy_correct_cycles_with_page_cross() {
        let mut cpu = NESCPU::new();
        let mut bus = MockBus::new();
        bus.expect_read().with(eq(0x88)).return_const(0x11);
        bus.expect_read().with(eq(0x89)).return_const(0x22);

        bus.expect_read().with(eq(0x2310)).return_const(0x0);

        cpu.y = 0xff;
        assert_eq!(6, cpu.andc(&cpu._indy(0x88, &bus)));
    }

    #[test]
    fn test_and() {
        let mut cpu = NESCPU::new();
        cpu.a = 0b1010_1010_u8;

        cpu.and(&cpu._imm(0b0101_0101_u8), &mut MockBus::new());

        assert_eq!(0x0, cpu.a);
    }

    #[test]
    fn test_and_all_ones() {
        let mut cpu = NESCPU::new();
        cpu.a = 0xff;

        cpu.and(&cpu._imm(0xff), &mut MockBus::new());

        assert_eq!(0xff, cpu.a);
    }

    #[test]
    fn test_and_half_ones() {
        let mut cpu = NESCPU::new();
        cpu.a = 0b0000_1111_u8;

        cpu.and(&cpu._imm(0b0000_1111_u8), &mut MockBus::new());

        assert_eq!(0xf, cpu.a);
    }

    #[test]
    fn test_and_zero_flag() {
        let mut cpu = NESCPU::new();
        cpu.a = 0b0000_1111_u8;

        cpu.and(&cpu._imm(0b0000_1111_u8), &mut MockBus::new());

        assert_eq!(false, cpu.z);

        cpu.and(&cpu._imm(0b0000_0000_u8), &mut MockBus::new());

        assert_eq!(true, cpu.z);
    }

    #[test]
    fn test_and_negative_flag() {
        let mut cpu = NESCPU::new();
        cpu.a = 0xff;

        cpu.and(&cpu._imm(0xff), &mut MockBus::new());

        assert_eq!(true, cpu.n)
    }
}
