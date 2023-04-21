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

use core::panic;

use crate::cpu::{addr::AddrMode, addr::AddrModeResult, bus::Bus, CPU};

impl CPU {
    fn dec(&mut self, mode: &AddrModeResult, bus: &dyn Bus) -> u8 {
        let result = mode.data.unwrap().wrapping_sub(1);
        bus.write(mode.addr.unwrap(), result);

        self.n = (result & 0x80) > 0;
        self.z = result == 0;

        match mode.mode {
            AddrMode::ZP => 5,
            AddrMode::ZPX => 6,
            AddrMode::ABS => 6,
            AddrMode::ABSX => 7,
            _ => panic!("Unimplemented"),
        }
    }
}

#[cfg(test)]
mod dec_tests {
    use mockall::predicate::eq;

    use crate::cpu::bus::MockBus;

    use super::*;

    #[test]
    fn test_dec_zp_correct_number_of_cycles() {
        let mut cpu = CPU::new();
        let mut bus = MockBus::new();

        bus.expect_read().return_const(0x0);

        bus.expect_write().return_const(());

        assert_eq!(5, cpu.dec(&cpu.zp(0x0, &bus), &bus));
    }

    #[test]
    fn test_dec_zpx_correct_number_of_cycles() {
        let mut cpu = CPU::new();
        let mut bus = MockBus::new();

        bus.expect_read().return_const(0x0);

        bus.expect_write().return_const(());

        assert_eq!(6, cpu.dec(&cpu.zpx(0x0, &bus), &bus));
    }

    #[test]
    fn test_dec_abs_correct_number_of_cycles() {
        let mut cpu = CPU::new();
        let mut bus = MockBus::new();

        bus.expect_read().return_const(0x0);

        bus.expect_write().return_const(());

        assert_eq!(6, cpu.dec(&cpu.abs(0x0, &bus), &bus));
    }

    #[test]
    fn test_dec_absx_correct_number_of_cycles() {
        let mut cpu = CPU::new();
        let mut bus = MockBus::new();

        bus.expect_read().return_const(0x0);

        bus.expect_write().return_const(());

        assert_eq!(7, cpu.dec(&cpu.absx(0x0, &bus), &bus));
    }

    #[test]
    fn test_dec() {
        let mut cpu = CPU::new();
        let mut bus = MockBus::new();

        bus.expect_read().with(eq(0x0)).times(1).return_const(0x10);

        bus.expect_write()
            .with(eq(0x0), eq(0xf))
            .times(1)
            .return_const(());

        cpu.dec(&cpu.zp(0x0, &bus), &bus);
    }

    #[test]
    fn test_dec_negative_flag() {
        let mut cpu = CPU::new();
        let mut bus = MockBus::new();

        bus.expect_read().return_const(0x0);

        bus.expect_write().with(eq(0x0), eq(0xff)).return_const(());

        cpu.dec(&cpu.zp(0x0, &bus), &bus);
        assert_eq!(true, cpu.n);
    }

    #[test]
    fn test_dec_zero_flag() {
        let mut cpu = CPU::new();
        let mut bus = MockBus::new();

        bus.expect_read().return_const(0x1);

        bus.expect_write().with(eq(0x0), eq(0x0)).return_const(());

        cpu.dec(&cpu.zp(0x0, &bus), &bus);
        assert_eq!(true, cpu.z);
    }
}
