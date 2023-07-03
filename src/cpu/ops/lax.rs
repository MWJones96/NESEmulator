/*
    LAX - Load Accumulator and Index Register X From Memory
    Operation: M → A, X

    The undocumented LAX instruction loads the accumulator and the
    index register X from memory.

    LAX does not affect the C or V flags; sets Z if the value loaded
    was zero, otherwise resets it; sets N if the value loaded in bit 7
    is a 1; otherwise N is reset, and affects only the X register.
*/

use crate::{
    bus::Bus,
    cpu::{addr::AddrModeResult, NESCPU},
};

impl NESCPU {
    pub(in crate::cpu) fn laxc(&self, mode: &AddrModeResult) -> u8 {
        2 + mode.cycles
    }

    pub(in crate::cpu) fn lax(&mut self, mode: &AddrModeResult, bus: &mut dyn Bus) {
        let data = match mode.addr {
            Some(addr) => bus.read(addr),
            None => mode.data.unwrap(),
        };

        self.a = data;
        self.x = data;

        self.n = (self.a & 0x80) != 0;
        self.z = self.a == 0;
    }
}

#[cfg(test)]
mod lax_tests {
    use mockall::predicate::eq;

    use crate::bus::MockBus;

    use super::*;

    #[test]
    fn test_lax_imm_correct_number_of_cycles() {
        let cpu = NESCPU::new();
        assert_eq!(2, cpu.laxc(&cpu._imm(0x0)));
    }

    #[test]
    fn test_lax_zp_correct_number_of_cycles() {
        let cpu = NESCPU::new();
        let mut bus = MockBus::new();
        bus.expect_read().return_const(0x0);

        assert_eq!(3, cpu.laxc(&cpu._zp(0x0, &bus)));
    }

    #[test]
    fn test_lax_zpy_correct_number_of_cycles() {
        let cpu = NESCPU::new();
        let mut bus = MockBus::new();
        bus.expect_read().return_const(0x0);

        assert_eq!(4, cpu.laxc(&cpu._zpy(0x0, &bus)));
    }

    #[test]
    fn test_lax_abs_correct_number_of_cycles() {
        let cpu = NESCPU::new();
        let mut bus = MockBus::new();
        bus.expect_read().return_const(0x0);

        assert_eq!(4, cpu.laxc(&cpu._abs(0x0, &bus)));
    }

    #[test]
    fn test_lax_cycles_absy_no_page_cross() {
        let cpu = NESCPU::new();
        let mut bus = MockBus::new();
        bus.expect_read().return_const(0x0);

        assert_eq!(4, cpu.laxc(&cpu._absy(0x0, &bus)));
    }

    #[test]
    fn test_lax_cycles_absy_with_page_cross() {
        let mut cpu = NESCPU::new();
        cpu.y = 0xff;

        let mut bus = MockBus::new();
        bus.expect_read().return_const(0x0);

        assert_eq!(5, cpu.laxc(&cpu._absy(0x34, &bus)));
    }

    #[test]
    fn test_lax_cycles_indx_correct_number_of_cycles() {
        let cpu = NESCPU::new();

        let mut bus = MockBus::new();
        bus.expect_read().return_const(0x0);

        assert_eq!(6, cpu.laxc(&cpu._indx(0x0, &bus)));
    }

    #[test]
    fn test_lax_cycles_indy_no_page_cross() {
        let cpu = NESCPU::new();

        let mut bus = MockBus::new();
        bus.expect_read().return_const(0x0);

        assert_eq!(5, cpu.laxc(&cpu._indy(0x0, &bus)));
    }

    #[test]
    fn test_lax_cycles_indy_with_page_cross() {
        let mut cpu = NESCPU::new();
        cpu.y = 0xff;

        let mut bus = MockBus::new();
        bus.expect_read().with(eq(0x34)).return_const(0x34);
        bus.expect_read().with(eq(0x12)).return_const(0x12);
        bus.expect_read().return_const(0x0);

        assert_eq!(6, cpu.laxc(&cpu._indy(0x34, &bus)));
    }

    #[test]
    fn test_lax() {
        let mut cpu = NESCPU::new();

        cpu.lax(&cpu._imm(0xee), &mut MockBus::new());

        assert_eq!(0xee, cpu.a);
        assert_eq!(0xee, cpu.x);

        assert_eq!(true, cpu.n);
        assert_eq!(false, cpu.z);

        cpu.lax(&cpu._imm(0x0), &mut MockBus::new());

        assert_eq!(0x0, cpu.a);
        assert_eq!(0x0, cpu.x);

        assert_eq!(false, cpu.n);
        assert_eq!(true, cpu.z);
    }
}
