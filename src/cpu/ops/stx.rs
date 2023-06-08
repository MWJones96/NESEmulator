/*
    STX - Store Index Register X In Memory
    Operation: X → M

    Transfers value of X register to addressed memory location.

    No flags or registers in the microprocessor are affected by
    the store operation.
*/

use crate::cpu::{
    addr::{AddrModeResult, AddrModeType},
    bus::CPUBus,
};

use super::super::CPU;

impl CPU {
    pub(in crate::cpu) fn stxc(&self, mode: &AddrModeResult) -> u8 {
        match mode.mode {
            AddrModeType::Absy => 4,
            _ => 2 + mode.cycles,
        }
    }

    pub(in crate::cpu) fn stx(&mut self, mode: &AddrModeResult, bus: &mut dyn CPUBus) {
        bus.write(mode.addr.unwrap(), self.x);
    }
}

#[cfg(test)]
mod stx_tests {
    use mockall::predicate::eq;

    use crate::cpu::bus::MockCPUBus;

    use super::*;

    #[test]
    fn test_stx_zp_correct_number_of_cycles() {
        let cpu = CPU::new();
        let mut bus = MockCPUBus::new();

        bus.expect_read().return_const(0x0);

        assert_eq!(3, cpu.stxc(&cpu._zp(0x0, &bus)));
    }

    #[test]
    fn test_stx_abs_correct_number_of_cycles() {
        let cpu = CPU::new();
        let mut bus = MockCPUBus::new();

        bus.expect_read().return_const(0x0);

        assert_eq!(4, cpu.stxc(&cpu._abs(0x0, &bus)));
    }

    #[test]
    fn test_stx_absy_correct_number_of_cycles() {
        let mut cpu = CPU::new();
        let mut bus = MockCPUBus::new();
        cpu.y = 0xff;

        bus.expect_read().return_const(0x0);

        assert_eq!(4, cpu.stxc(&cpu._absy(0xffff, &bus)));
    }

    #[test]
    fn test_stx() {
        let mut cpu = CPU::new();
        let mut bus = MockCPUBus::new();
        cpu.x = 0xcc;

        bus.expect_read().return_const(0x0);

        bus.expect_write()
            .with(eq(0xffff), eq(0xcc))
            .times(1)
            .return_const(());

        cpu.stx(&cpu._abs(0xffff, &bus), &mut bus);
    }
}
