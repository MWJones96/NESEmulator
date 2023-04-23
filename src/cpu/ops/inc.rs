/*
    INC - Increment Memory By One
    Operation: M + 1 → M

    This instruction adds 1 to the contents of the addressed
    memory loca­tion.

    The increment memory instruction does not affect any
    internal registers and does not affect the carry or
    overflow flags. If bit 7 is on as the result of the
    increment,N is set, otherwise it is reset; if the
    increment causes the result to become 0, the Z flag
    is set on, otherwise it is reset.
*/

use crate::cpu::{
    addr::{AddrMode, AddrModeResult},
    bus::Bus,
    CPU,
};

impl CPU {
    fn inc(&mut self, mode: &AddrModeResult, bus: &dyn Bus) -> u8 {
        let data = mode.data.unwrap().wrapping_add(1);
        let addr = mode.addr.unwrap();

        bus.write(addr, data);

        self.n = (data & 0x80) > 0;
        self.z = data == 0;

        match mode.mode {
            AddrMode::ABSX => 7,
            _ => 4 + mode.cycles,
        }
    }
}

#[cfg(test)]
mod inc_tests {
    use mockall::predicate::eq;

    use crate::cpu::bus::MockBus;

    use super::*;

    #[test]
    fn test_inc_zp_correct_number_of_cycles() {
        let mut cpu = CPU::new();
        let mut bus = MockBus::new();

        bus.expect_read().return_const(0x0);

        bus.expect_write().return_const(());

        assert_eq!(5, cpu.inc(&cpu.zp(0x0, &bus), &bus));
    }

    #[test]
    fn test_inc_zpx_correct_number_of_cycles() {
        let mut cpu = CPU::new();
        let mut bus = MockBus::new();

        bus.expect_read().return_const(0x0);

        bus.expect_write().return_const(());

        assert_eq!(6, cpu.inc(&cpu.zpx(0x0, &bus), &bus));
    }

    #[test]
    fn test_inc_abs_correct_number_of_cycles() {
        let mut cpu = CPU::new();
        let mut bus = MockBus::new();

        bus.expect_read().return_const(0x0);

        bus.expect_write().return_const(());

        assert_eq!(6, cpu.inc(&cpu.abs(0x0, &bus), &bus));
    }

    #[test]
    fn test_inc_absx_correct_number_of_cycles() {
        let mut cpu = CPU::new();
        let mut bus = MockBus::new();

        bus.expect_read().return_const(0x0);

        bus.expect_write().return_const(());

        assert_eq!(7, cpu.inc(&cpu.absx(0x0, &bus), &bus));
    }

    #[test]
    fn test_inc_negative_flag() {
        let mut cpu = CPU::new();
        let mut bus = MockBus::new();

        bus.expect_read().with(eq(0x0)).times(1).return_const(0x7f);

        bus.expect_write()
            .with(eq(0x0), eq(0x80))
            .times(1)
            .return_const(());

        cpu.inc(&cpu.absx(0x0, &bus), &bus);

        assert_eq!(true, cpu.n);
    }

    #[test]
    fn test_inc_zero_flag() {
        let mut cpu = CPU::new();
        let mut bus = MockBus::new();

        bus.expect_read().with(eq(0x0)).times(1).return_const(0xff);

        bus.expect_write()
            .with(eq(0x0), eq(0x0))
            .times(1)
            .return_const(());

        cpu.inc(&cpu.absx(0x0, &bus), &bus);

        assert_eq!(true, cpu.z);
    }
}