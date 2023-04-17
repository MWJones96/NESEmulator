use crate::cpu::{bus::Bus, CPU};

use super::AddrModeResult;

impl CPU {
    pub(in crate::cpu) fn absy(&self, addr: u16, bus: &dyn Bus) -> AddrModeResult {
        self.abs_helper(addr, self.y, super::AddrMode::ABSY, bus)
    }
}

#[cfg(test)]
mod absy_tests {
    use mockall::predicate::eq;

    use super::*;
    use crate::cpu::{addr::AddrModeResult, bus::MockBus};

    #[test]
    fn test_absy_addressing_mode_no_page_cross() {
        let mut cpu = CPU::new();
        let mut mock_bus = MockBus::new();

        cpu.y = 0x2;

        mock_bus.expect_read().with(eq(0x2)).return_const(0x88);

        let result = cpu.absy(0x0, &mock_bus);
        assert_eq!(
            AddrModeResult {
                data: Some(0x88),
                cycles: 2,
                mode: crate::cpu::addr::AddrMode::ABSY,
                addr: Some(0x2)
            },
            result
        );
    }

    #[test]
    fn test_absy_addressing_mode_with_page_cross() {
        let mut cpu = CPU::new();
        let mut mock_bus = MockBus::new();

        cpu.y = 0x2;

        mock_bus.expect_read().with(eq(0x1)).return_const(0x88);

        let result = cpu.absy(0xffff, &mock_bus);
        assert_eq!(
            AddrModeResult {
                data: Some(0x88),
                cycles: 3,
                mode: crate::cpu::addr::AddrMode::ABSY,
                addr: Some(0x1)
            },
            result
        );
    }
}
