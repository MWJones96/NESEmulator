use crate::cpu::{bus::Bus, CPU};

use super::AddrModeResult;

impl CPU {
    pub(in crate::cpu) fn abs(&self, addr: u16, bus: &dyn Bus) -> AddrModeResult {
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
    use crate::cpu::bus::MockBus;

    #[test]
    fn test_abs_addressing_mode() {
        let cpu = CPU::new();
        let mut mock_bus = MockBus::new();

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
