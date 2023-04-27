/* 
    STY - Store Index Register Y In Memory
    Operation: Y → M

    Transfer the value of the Y register to the addressed memory location.

    STY does not affect any flags or registers in the microprocessor.
*/

use crate::cpu::{addr::{AddrMode, AddrModeResult}, bus::Bus};

use super::super::CPU;

impl CPU {
    pub(in crate::cpu) fn sty_cycles(&self, mode: &AddrModeResult) -> u8 {
        match mode.mode {
            AddrMode::ABSX => 4,
            _ => 2 + mode.cycles
        }
    }

    pub(in crate::cpu) fn sty(&self, mode: &AddrModeResult, bus: &dyn Bus) {
        bus.write(mode.addr.unwrap(), self.y);
    }
}

#[cfg(test)]
mod sty_tests {
    use mockall::predicate::eq;

    use crate::cpu::bus::MockBus;

    use super::*;

    #[test]
    fn test_sty_zp_correct_number_of_cycles() {
        let cpu = CPU::new();
        let mut bus = MockBus::new();

        bus.expect_read()
            .return_const(0x0);

        assert_eq!(3, cpu.sty_cycles(&cpu.zp(0x0, &bus)));
    }

    #[test]
    fn test_sty_abs_correct_number_of_cycles() {
        let cpu = CPU::new();
        let mut bus = MockBus::new();

        bus.expect_read()
            .return_const(0x0);

        assert_eq!(4, cpu.sty_cycles(&cpu.abs(0x0, &bus)));
    }


    #[test]
    fn test_sty_absx_correct_number_of_cycles() {
        let mut cpu = CPU::new();
        let mut bus = MockBus::new();
        cpu.x = 0xff;

        bus.expect_read()
            .return_const(0x0);

        assert_eq!(4, cpu.sty_cycles(&cpu.absx(0xffff, &bus)));
    }

    #[test]
    fn test_sty() {
        let mut cpu = CPU::new();
        let mut bus = MockBus::new();
        cpu.y = 0xbb;

        bus.expect_read()
            .return_const(0x0);

        bus.expect_write()
            .with(eq(0xffff), eq(0xbb))
            .times(1)
            .return_const(());

        cpu.sty(&cpu.abs(0xffff, &bus), &bus);
    }
}