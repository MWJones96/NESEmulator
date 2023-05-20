/* 
    SAX - Store Accumulator "AND" Index Register X in Memory
    Operation: A ∧ X → M

    The undocumented SAX instruction performs a bit-by-bit AND 
    operation of the value of the accumulator and the value of 
    the index register X and stores the result in memory.

    No flags or registers in the microprocessor are affected by 
    the store operation.
*/

use crate::cpu::{
    addr::{AddrMode, AddrModeResult},
    bus::CPUBus,
    CPU,
};

impl CPU {
    #[inline]
    pub(in crate::cpu) fn sax_cycles(&self, mode: &AddrModeResult) -> u8 {
        2 + mode.cycles
    }

    #[inline]
    pub(in crate::cpu) fn sax(&mut self, mode: &AddrModeResult, bus: &mut impl CPUBus) {
        bus.write(mode.addr.unwrap(), self.a & self.x);
    }
}

#[cfg(test)]
mod sax_tests {
    use mockall::predicate::eq;

    use crate::cpu::bus::MockCPUBus;

    use super::*;

    #[test]
    fn test_sax_zp_correct_number_of_cycles() {
        let cpu = CPU::new();
        let mut bus = MockCPUBus::new();
        bus.expect_read().return_const(0x0);

        assert_eq!(3, cpu.sax_cycles(&cpu.zp(0x0, &bus)));
    }

    #[test]
    fn test_sax_zpy_correct_number_of_cycles() {
        let cpu = CPU::new();
        let mut bus = MockCPUBus::new();
        bus.expect_read().return_const(0x0);

        assert_eq!(4, cpu.sax_cycles(&cpu.zpy(0x0, &bus)));
    }

    #[test]
    fn test_sax_abs_correct_number_of_cycles() {
        let cpu = CPU::new();
        let mut bus = MockCPUBus::new();
        bus.expect_read().return_const(0x0);

        assert_eq!(4, cpu.sax_cycles(&cpu.abs(0x0, &bus)));
    }

    #[test]
    fn test_sax_indx_correct_number_of_cycles() {
        let cpu = CPU::new();
        let mut bus = MockCPUBus::new();
        bus.expect_read().return_const(0x0);

        assert_eq!(6, cpu.sax_cycles(&cpu.indx(0x0, &bus)));
    }

    #[test]
    fn test_sax() {
        let mut cpu = CPU::new();
        cpu.a = 0b1010_1010;
        cpu.x = 0b0101_0101;

        let mut bus = MockCPUBus::new();
        bus.expect_read().return_const(0x0);
        bus.expect_write().with(eq(0x0), eq(0x0)).once().return_const(());

        cpu.sax(&cpu.zp(0x0, &bus), &mut bus);
    }
}