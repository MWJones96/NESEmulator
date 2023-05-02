/*
    STA - Store Accumulator in Memory
    Operation: A → M

    This instruction transfers the contents of the accumulator to memory.

    This instruction affects none of the flags in the processor status
    register and does not affect the accumulator.
*/

use crate::cpu::{
    addr::{AddrMode, AddrModeResult},
    bus::CPUBus,
};

use super::super::CPU;

impl CPU {
    pub(in crate::cpu) fn sta_cycles(&self, mode: &AddrModeResult) -> u8 {
        match mode.mode {
            AddrMode::ABSX | AddrMode::ABSY => 5,
            AddrMode::INDY => 6,
            _ => 2 + mode.cycles,
        }
    }

    pub(in crate::cpu) fn sta(&self, mode: &AddrModeResult, bus: &mut dyn CPUBus) {
        bus.write(mode.addr.unwrap(), self.a);
    }
}

#[cfg(test)]
mod sta_tests {
    use mockall::predicate::eq;

    use crate::cpu::bus::MockCPUBus;

    use super::*;

    #[test]
    fn test_sta_zp_correct_number_of_cycles() {
        let cpu = CPU::new();
        let mut bus = MockCPUBus::new();

        bus.expect_read().return_const(0x0);

        assert_eq!(3, cpu.sta_cycles(&cpu.zp(0x0, &bus)));
    }

    #[test]
    fn test_sta_zpx_correct_number_of_cycles() {
        let cpu = CPU::new();
        let mut bus = MockCPUBus::new();

        bus.expect_read().return_const(0x0);

        assert_eq!(4, cpu.sta_cycles(&cpu.zpx(0x0, &bus)));
    }

    #[test]
    fn test_sta_abs_correct_number_of_cycles() {
        let cpu = CPU::new();
        let mut bus = MockCPUBus::new();

        bus.expect_read().return_const(0x0);

        assert_eq!(4, cpu.sta_cycles(&cpu.abs(0x0, &bus)));
    }

    #[test]
    fn test_sta_absx_correct_number_of_cycles() {
        let cpu = CPU::new();
        let mut bus = MockCPUBus::new();

        bus.expect_read().return_const(0x0);

        assert_eq!(5, cpu.sta_cycles(&cpu.absx(0x0, &bus)));
    }

    #[test]
    fn test_sta_absy_correct_number_of_cycles() {
        let cpu = CPU::new();
        let mut bus = MockCPUBus::new();

        bus.expect_read().return_const(0x0);

        assert_eq!(5, cpu.sta_cycles(&cpu.absy(0x0, &bus)));
    }

    #[test]
    fn test_sta_indx_correct_number_of_cycles() {
        let cpu = CPU::new();
        let mut bus = MockCPUBus::new();

        bus.expect_read().return_const(0x0);

        assert_eq!(6, cpu.sta_cycles(&cpu.indx(0x0, &bus)));
    }

    #[test]
    fn test_sta_indy_correct_number_of_cycles() {
        let cpu = CPU::new();
        let mut bus = MockCPUBus::new();

        bus.expect_read().return_const(0x0);

        assert_eq!(6, cpu.sta_cycles(&cpu.indy(0x0, &bus)));
    }

    #[test]
    fn test_sta() {
        let mut cpu = CPU::new();
        let mut bus = MockCPUBus::new();
        cpu.a = 0xee;

        bus.expect_read().return_const(0x0);

        bus.expect_write()
            .with(eq(0x8000), eq(0xee))
            .times(1)
            .return_const(());

        cpu.sta(&cpu.abs(0x8000, &bus), &mut bus);
    }
}
