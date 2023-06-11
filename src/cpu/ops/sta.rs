/*
    STA - Store Accumulator in Memory
    Operation: A → M

    This instruction transfers the contents of the accumulator to memory.

    This instruction affects none of the flags in the processor status
    register and does not affect the accumulator.
*/

use crate::cpu::{
    addr::{AddrModeResult, AddrModeType},
    bus::CPUBus,
};

use super::super::NESCPU;

impl NESCPU {
    pub(in crate::cpu) fn stac(&self, mode: &AddrModeResult) -> u8 {
        match mode.mode {
            AddrModeType::Absx | AddrModeType::Absy => 5,
            AddrModeType::Indy => 6,
            _ => 2 + mode.cycles,
        }
    }

    pub(in crate::cpu) fn sta(&mut self, mode: &AddrModeResult, bus: &mut dyn CPUBus) {
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
        let cpu = NESCPU::new();
        let mut bus = MockCPUBus::new();

        bus.expect_read().return_const(0x0);

        assert_eq!(3, cpu.stac(&cpu._zp(0x0, &bus)));
    }

    #[test]
    fn test_sta_zpx_correct_number_of_cycles() {
        let cpu = NESCPU::new();
        let mut bus = MockCPUBus::new();

        bus.expect_read().return_const(0x0);

        assert_eq!(4, cpu.stac(&cpu._zpx(0x0, &bus)));
    }

    #[test]
    fn test_sta_abs_correct_number_of_cycles() {
        let cpu = NESCPU::new();
        let mut bus = MockCPUBus::new();

        bus.expect_read().return_const(0x0);

        assert_eq!(4, cpu.stac(&cpu._abs(0x0, &bus)));
    }

    #[test]
    fn test_sta_absx_correct_number_of_cycles() {
        let cpu = NESCPU::new();
        let mut bus = MockCPUBus::new();

        bus.expect_read().return_const(0x0);

        assert_eq!(5, cpu.stac(&cpu._absx(0x0, &bus)));
    }

    #[test]
    fn test_sta_absy_correct_number_of_cycles() {
        let cpu = NESCPU::new();
        let mut bus = MockCPUBus::new();

        bus.expect_read().return_const(0x0);

        assert_eq!(5, cpu.stac(&cpu._absy(0x0, &bus)));
    }

    #[test]
    fn test_sta_indx_correct_number_of_cycles() {
        let cpu = NESCPU::new();
        let mut bus = MockCPUBus::new();

        bus.expect_read().return_const(0x0);

        assert_eq!(6, cpu.stac(&cpu._indx(0x0, &bus)));
    }

    #[test]
    fn test_sta_indy_correct_number_of_cycles() {
        let cpu = NESCPU::new();
        let mut bus = MockCPUBus::new();

        bus.expect_read().return_const(0x0);

        assert_eq!(6, cpu.stac(&cpu._indy(0x0, &bus)));
    }

    #[test]
    fn test_sta() {
        let mut cpu = NESCPU::new();
        let mut bus = MockCPUBus::new();
        cpu.a = 0xee;

        bus.expect_read().return_const(0x0);

        bus.expect_write()
            .with(eq(0x8000), eq(0xee))
            .times(1)
            .return_const(());

        cpu.sta(&cpu._abs(0x8000, &bus), &mut bus);
    }
}
