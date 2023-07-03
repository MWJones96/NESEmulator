/*
    ORA - "OR" Memory with Accumulator
    Operation: A ∨ M → A

    The ORA instruction transfers the memory and the
    accumulator to the adder which performs a binary "OR"
    on a bit-by-bit basis and stores the result in the
    accumulator.

    This instruction affects the accumulator; sets the zero
    flag if the result in the accumulator is 0, otherwise
    resets the zero flag; sets the negative flag if the
    result in the accumulator has bit 7 on, otherwise resets
    the negative flag.
*/

use crate::{
    bus::Bus,
    cpu::{addr::AddrModeResult, NESCPU},
};

impl NESCPU {
    pub(in crate::cpu) fn orac(&self, mode: &AddrModeResult) -> u8 {
        2 + mode.cycles
    }

    pub(in crate::cpu) fn ora(&mut self, mode: &AddrModeResult, bus: &mut dyn Bus) {
        if let Some(addr) = mode.addr {
            self.a |= bus.read(addr);
        } else {
            self.a |= mode.data.unwrap();
        }

        self.n = (self.a & 0x80) != 0;
        self.z = self.a == 0;
    }
}

#[cfg(test)]
mod ora_tests {
    use crate::bus::MockBus;

    use super::*;

    #[test]
    fn test_ora_imm_correct_number_of_cycles() {
        let cpu = NESCPU::new();
        assert_eq!(2, cpu.orac(&cpu._imm(0x0)));
    }

    #[test]
    fn test_ora_zp_correct_number_of_cycles() {
        let cpu = NESCPU::new();
        let mut bus = MockBus::new();

        bus.expect_read().return_const(0x0);

        assert_eq!(3, cpu.orac(&cpu._zp(0x0, &bus)));
    }

    #[test]
    fn test_ora_zpx_correct_number_of_cycles() {
        let cpu = NESCPU::new();
        let mut bus = MockBus::new();

        bus.expect_read().return_const(0x0);

        assert_eq!(4, cpu.orac(&cpu._zpx(0x0, &bus)));
    }

    #[test]
    fn test_ora_abs_correct_number_of_cycles() {
        let cpu = NESCPU::new();
        let mut bus = MockBus::new();

        bus.expect_read().return_const(0x0);

        assert_eq!(4, cpu.orac(&cpu._abs(0x0, &bus)));
    }

    #[test]
    fn test_ora_absx_no_page_cross_correct_number_of_cycles() {
        let cpu = NESCPU::new();
        let mut bus = MockBus::new();

        bus.expect_read().return_const(0x0);

        assert_eq!(4, cpu.orac(&cpu._absx(0x0, &bus)));
    }

    #[test]
    fn test_ora_absx_with_page_cross_correct_number_of_cycles() {
        let mut cpu = NESCPU::new();
        let mut bus = MockBus::new();
        cpu.x = 0xff;

        bus.expect_read().return_const(0x0);

        assert_eq!(5, cpu.orac(&cpu._absx(0x1234, &bus)));
    }

    #[test]
    fn test_ora_absy_no_page_cross_correct_number_of_cycles() {
        let cpu = NESCPU::new();
        let mut bus = MockBus::new();

        bus.expect_read().return_const(0x0);

        assert_eq!(4, cpu.orac(&cpu._absy(0x0, &bus)));
    }

    #[test]
    fn test_ora_absy_with_page_cross_correct_number_of_cycles() {
        let mut cpu = NESCPU::new();
        let mut bus = MockBus::new();
        cpu.y = 0xff;

        bus.expect_read().return_const(0x0);

        assert_eq!(5, cpu.orac(&cpu._absy(0x1234, &bus)));
    }

    #[test]
    fn test_ora_indx_correct_number_of_cycles() {
        let cpu = NESCPU::new();
        let mut bus = MockBus::new();

        bus.expect_read().return_const(0x0);

        assert_eq!(6, cpu.orac(&cpu._indx(0x0, &bus)));
    }

    #[test]
    fn test_ora_indy_no_page_cross_correct_number_of_cycles() {
        let cpu = NESCPU::new();
        let mut bus = MockBus::new();

        bus.expect_read().return_const(0x0);

        assert_eq!(5, cpu.orac(&cpu._indy(0x0, &bus)));
    }

    #[test]
    fn test_ora_indy_with_page_cross_correct_number_of_cycles() {
        let mut cpu = NESCPU::new();
        let mut bus = MockBus::new();
        cpu.y = 0xff;

        bus.expect_read().return_const(0x80);

        assert_eq!(6, cpu.orac(&cpu._indy(0x0, &bus)));
    }

    #[test]
    fn test_ora() {
        let mut cpu = NESCPU::new();

        cpu.a = 0b1111_0000;
        cpu.ora(&cpu._imm(0b1010_1010), &mut MockBus::new());

        assert_eq!(0b1111_1010, cpu.a);
    }

    #[test]
    fn test_ora_negative_flag() {
        let mut cpu = NESCPU::new();

        cpu.a = 0x80;
        cpu.ora(&cpu._imm(0x80), &mut MockBus::new());

        assert_eq!(true, cpu.n);
    }

    #[test]
    fn test_ora_zero_flag() {
        let mut cpu = NESCPU::new();

        cpu.a = 0x0;
        cpu.ora(&cpu._imm(0x0), &mut MockBus::new());

        assert_eq!(true, cpu.z);
    }
}
