/*
    SHS - Transfer Accumulator "AND" Index Register X to Stack Pointer
    then Store Stack Pointer "AND" Hi-Byte In Memory
    Operation: A ∧ X → S, S ∧ (H + 1) → M

    The undocumented SHS instruction performs a bit-by-bit AND operation
    of the value of the accumulator and the value of the index register X
    and stores the result in the stack pointer. It then performs a
    bit-by-bit AND operation of the resulting stack pointer and the upper
    8 bits of the given address (ignoring the addressing mode's Y offset),
    plus 1, and transfers the result to the addressed memory location.

    No flags or registers in the microprocessor are affected by the store
    operation.
*/

use crate::cpu::{addr::AddrModeResult, bus::CPUBus, CPU};

impl CPU {
    #[inline]
    pub(in crate::cpu) fn shsc(&self, _mode: &AddrModeResult) -> u8 {
        5
    }

    #[inline]
    pub(in crate::cpu) fn shs(&mut self, mode: &AddrModeResult, bus: &mut dyn CPUBus) {
        self.sp = self.a & self.x;
        let write_addr = mode.addr.unwrap();
        let h = ((write_addr.wrapping_sub(self.y as u16)) >> 8) as u8;
        bus.write(write_addr, self.sp & h.wrapping_add(1));
    }
}

#[cfg(test)]
mod shs_tests {
    use mockall::predicate::eq;

    use crate::cpu::bus::MockCPUBus;

    use super::*;

    #[test]
    fn test_shs_absy_correct_number_ofc() {
        let cpu = CPU::new();
        let mut bus = MockCPUBus::new();
        bus.expect_read().return_const(0x0);

        assert_eq!(5, cpu.shsc(&cpu._absy(0x0, &bus)));
    }

    #[test]
    fn test_shs() {
        let mut cpu = CPU::new();
        cpu.a = 0b1;
        cpu.x = 0b11;

        cpu.y = 0xff;

        let mut bus = MockCPUBus::new();
        bus.expect_write()
            .with(eq(0x1333), eq(0x1 & 0x13))
            .once()
            .return_const(());
        bus.expect_read().return_const(0x0);

        cpu.shs(&cpu._absy(0x1234, &bus), &mut bus);

        assert_eq!(0x1, cpu.sp);
    }
}
