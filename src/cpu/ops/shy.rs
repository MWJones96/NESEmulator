/*
    SHY - Store Index Register Y "AND" Value
    Operation: Y ∧ (H + 1) → M

    The undocumented SHY instruction performs a bit-by-bit AND operation
    of the index register Y and the upper 8 bits of the given address
    (ignoring the the addressing mode's X offset), plus 1. It then
    transfers the result to the addressed memory location.

    No flags or registers in the microprocessor are affected by the store
    operation.
*/

use crate::cpu::{addr::AddrModeResult, bus::CPUBus, CPU};

impl CPU {
    #[inline]
    pub(in crate::cpu) fn shy_cycles(&self, _mode: &AddrModeResult) -> u8 {
        5
    }

    #[inline]
    pub(in crate::cpu) fn shy(&mut self, mode: &AddrModeResult, bus: &mut impl CPUBus) {
        let write_addr = mode.addr.unwrap();
        let h = (write_addr.wrapping_sub(self.x as u16) >> 8) as u8;
        bus.write(write_addr, self.y & h.wrapping_add(1));
    }
}

#[cfg(test)]
mod shy_tests {
    use mockall::predicate::eq;

    use crate::cpu::bus::MockCPUBus;

    use super::*;

    #[test]
    fn test_shy_cycles() {
        let cpu = CPU::new();
        let mut bus = MockCPUBus::new();
        bus.expect_read().return_const(0x0);

        assert_eq!(5, cpu.shy_cycles(&cpu.absx(0x0, &bus)));
    }

    #[test]
    fn test_shy() {
        let mut cpu = CPU::new();
        cpu.y = 0xff;
        cpu.x = 0xff;

        let mut bus = MockCPUBus::new();
        bus.expect_read().return_const(0x0);
        bus.expect_write()
            .with(eq(0x1333), eq(0xff & 0x13))
            .once()
            .return_const(());

        cpu.shy(&cpu.absx(0x1234, &bus), &mut bus);
    }
}
