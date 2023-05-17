/*
    Absolute

    In absolute addressing, the second byte of the instruction
    specifies the eight low order bits of the effective address
    while the third byte specifies the eight high order bits.
    Thus, the absolute addressing mode allows access to the
    entire 65 K bytes of addressable memory.

    Bytes: 3
*/

use crate::cpu::{bus::CPUBus, CPU};

use super::AddrModeResult;

impl CPU {
    #[inline]
    pub(in crate::cpu) fn abs(&self, addr: u16, bus: &impl CPUBus) -> AddrModeResult {
        AddrModeResult {
            data: Some(bus.read(addr)),
            cycles: 2,
            mode: super::AddrMode::ABS,
            addr: Some(addr),
        }
    }
}

#[cfg(test)]
mod abs_tests {
    use mockall::predicate::eq;

    use super::*;
    use crate::cpu::bus::MockCPUBus;

    #[test]
    fn test_abs_addressing_mode() {
        let cpu = CPU::new();
        let mut mock_bus = MockCPUBus::new();

        mock_bus.expect_read().with(eq(0xffff)).return_const(0x88);

        let result = cpu.abs(0xffff, &mock_bus);
        assert_eq!(
            AddrModeResult {
                data: Some(0x88),
                cycles: 2,
                mode: crate::cpu::addr::AddrMode::ABS,
                addr: Some(0xffff)
            },
            result
        );
    }
}
