/*
    SHX - Store Index Register X "AND" Value
    Operation: X ∧ (H + 1) → M

    The undocumented SHX instruction performs a bit-by-bit AND operation
    of the index register X and the upper 8 bits of the given address
    (ignoring the the addressing mode's Y offset), plus 1. It then
    transfers the result to the addressed memory location.

    No flags or registers in the microprocessor are affected by the store
    operation.
*/

use crate::cpu::{addr::AddrModeResult, bus::CPUBus, NESCPU};

impl NESCPU {
    pub(in crate::cpu) fn shxc(&self, _mode: &AddrModeResult) -> u8 {
        5
    }

    pub(in crate::cpu) fn shx(&mut self, mode: &AddrModeResult, bus: &mut dyn CPUBus) {
        let write_addr = mode.addr.unwrap();
        let h = (write_addr.wrapping_sub(self.y as u16) >> 8) as u8;
        bus.write(write_addr, self.x & h.wrapping_add(1));
    }
}

#[cfg(test)]
mod shx_tests {
    use mockall::predicate::eq;

    use crate::cpu::bus::MockCPUBus;

    use super::*;

    #[test]
    fn test_shxc() {
        let cpu = NESCPU::new();
        let mut bus = MockCPUBus::new();
        bus.expect_read().return_const(0x0);

        assert_eq!(5, cpu.shxc(&cpu._absy(0x0, &bus)));
    }

    #[test]
    fn test_shx() {
        let mut cpu = NESCPU::new();
        cpu.x = 0xff;
        cpu.y = 0xff;

        let mut bus = MockCPUBus::new();
        bus.expect_read().return_const(0x0);
        bus.expect_write()
            .with(eq(0x1333), eq(0xff & 0x13))
            .once()
            .return_const(());

        cpu.shx(&cpu._absy(0x1234, &bus), &mut bus);
    }
}
