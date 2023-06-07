/*
    STY - Store Index Register Y In Memory
    Operation: Y → M

    Transfer the value of the Y register to the addressed memory location.

    STY does not affect any flags or registers in the microprocessor.
*/

use crate::cpu::{
    addr::{AddrModeResult, AddrModeType},
    bus::CPUBus,
};

use super::super::CPU;

impl CPU {
    #[inline]
    pub(in crate::cpu) fn styc(&self, mode: &AddrModeResult) -> u8 {
        match mode.mode {
            AddrModeType::ABSX => 4,
            _ => 2 + mode.cycles,
        }
    }

    #[inline]
    pub(in crate::cpu) fn sty(&self, mode: &AddrModeResult, bus: &mut dyn CPUBus) {
        bus.write(mode.addr.unwrap(), self.y);
    }
}

#[cfg(test)]
mod sty_tests {
    use mockall::predicate::eq;

    use crate::cpu::bus::MockCPUBus;

    use super::*;

    #[test]
    fn test_sty_zp_correct_number_ofc() {
        let cpu = CPU::new();
        let mut bus = MockCPUBus::new();

        bus.expect_read().return_const(0x0);

        assert_eq!(3, cpu.styc(&cpu._zp(0x0, &bus)));
    }

    #[test]
    fn test_sty_abs_correct_number_ofc() {
        let cpu = CPU::new();
        let mut bus = MockCPUBus::new();

        bus.expect_read().return_const(0x0);

        assert_eq!(4, cpu.styc(&cpu._abs(0x0, &bus)));
    }

    #[test]
    fn test_sty_absx_correct_number_ofc() {
        let mut cpu = CPU::new();
        let mut bus = MockCPUBus::new();
        cpu.x = 0xff;

        bus.expect_read().return_const(0x0);

        assert_eq!(4, cpu.styc(&cpu._absx(0xffff, &bus)));
    }

    #[test]
    fn test_sty() {
        let mut cpu = CPU::new();
        let mut bus = MockCPUBus::new();
        cpu.y = 0xbb;

        bus.expect_read().return_const(0x0);

        bus.expect_write()
            .with(eq(0xffff), eq(0xbb))
            .times(1)
            .return_const(());

        cpu.sty(&cpu._abs(0xffff, &bus), &mut bus);
    }
}
