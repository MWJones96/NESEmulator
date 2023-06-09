/*
    DEC - Decrement Memory By One
    Operation: M - 1 → M

    This instruction subtracts 1, in two's complement,
    from the contents of the addressed memory location.

    The decrement instruction does not affect any internal
    register in the microprocessor. It does not affect the
    carry or overflow flags. If bit 7 is on as a result
    of the decrement, then the N flag is set, otherwise it
    is reset. If the result of the decrement is 0, the Z
    flag is set, other­wise it is reset.
*/

use crate::{
    bus::Bus,
    cpu::{addr::AddrModeResult, addr::AddrModeType, NESCPU},
};

impl NESCPU {
    pub(in crate::cpu) fn decc(&self, mode: &AddrModeResult) -> u8 {
        match mode.mode {
            AddrModeType::Absx => 7,
            _ => 4 + mode.cycles,
        }
    }

    pub(in crate::cpu) fn dec(&mut self, mode: &AddrModeResult, bus: &mut dyn Bus) {
        let result = bus.read(mode.addr.unwrap()).wrapping_sub(1);
        bus.write(mode.addr.unwrap(), result);

        self.n = (result & 0x80) != 0;
        self.z = result == 0;
    }
}

#[cfg(test)]
mod dec_tests {
    use mockall::predicate::eq;

    use crate::bus::MockBus;

    use super::*;

    #[test]
    fn test_dec_zp_correct_number_of_cycles() {
        let cpu = NESCPU::new();
        let mut bus = MockBus::new();

        bus.expect_read().return_const(0x0);

        bus.expect_write().return_const(());

        assert_eq!(5, cpu.decc(&cpu._zp(0x0, &bus)));
    }

    #[test]
    fn test_dec_zpx_correct_number_of_cycles() {
        let cpu = NESCPU::new();
        let mut bus = MockBus::new();

        bus.expect_read().return_const(0x0);

        bus.expect_write().return_const(());

        assert_eq!(6, cpu.decc(&cpu._zpx(0x0, &bus)));
    }

    #[test]
    fn test_dec_abs_correct_number_of_cycles() {
        let cpu = NESCPU::new();
        let mut bus = MockBus::new();

        bus.expect_read().return_const(0x0);

        bus.expect_write().return_const(());

        assert_eq!(6, cpu.decc(&cpu._abs(0x0, &bus)));
    }

    #[test]
    fn test_dec_absx_correct_number_of_cycles() {
        let cpu = NESCPU::new();
        let mut bus = MockBus::new();

        bus.expect_read().return_const(0x0);

        bus.expect_write().return_const(());

        assert_eq!(7, cpu.decc(&cpu._absx(0x0, &bus)));
    }

    #[test]
    fn test_dec() {
        let mut cpu = NESCPU::new();
        let mut bus = MockBus::new();

        bus.expect_read().with(eq(0x0)).times(1).return_const(0x10);

        bus.expect_write()
            .with(eq(0x0), eq(0xf))
            .times(1)
            .return_const(());

        cpu.dec(&cpu._zp(0x0, &bus), &mut bus);
    }

    #[test]
    fn test_dec_negative_flag() {
        let mut cpu = NESCPU::new();
        let mut bus = MockBus::new();

        bus.expect_read().return_const(0x0);

        bus.expect_write().with(eq(0x0), eq(0xff)).return_const(());

        cpu.dec(&cpu._zp(0x0, &bus), &mut bus);
        assert_eq!(true, cpu.n);
    }

    #[test]
    fn test_dec_zero_flag() {
        let mut cpu = NESCPU::new();
        let mut bus = MockBus::new();

        bus.expect_read().return_const(0x1);

        bus.expect_write().with(eq(0x0), eq(0x0)).return_const(());

        cpu.dec(&cpu._zp(0x0, &bus), &mut bus);
        assert_eq!(true, cpu.z);
    }
}
