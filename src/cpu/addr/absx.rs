use crate::cpu::{CPU, bus::Bus};

use super::{AddrModeResult, AddrMode};

impl CPU {
    pub(in crate::cpu) fn absx(&self, addr: u16, bus: &dyn Bus) 
        -> AddrModeResult {
        self.abs_helper(addr, self.x, AddrMode::ABSX, bus)
    }
}

#[cfg(test)]
mod absx_tests {
    use mockall::predicate::eq;

    use crate::cpu::{bus::MockBus, addr::AddrModeResult};
    use super::*;

    #[test]
    fn test_absx_addressing_mode_no_page_cross() {
        let mut cpu = CPU::new();
        let mut mock_bus = MockBus::new();

        cpu.x = 0x2;

        mock_bus.expect_read()
            .with(eq(0x2))
            .return_const(0x88);

        let result = cpu.absx(0x0, &mock_bus);
        assert_eq!(AddrModeResult {
            data: 0x88,
            cycles: 2,
            mode: crate::cpu::addr::AddrMode::ABSX,
            addr: Some(0x2)
        }, result);
    }

    #[test]
    fn test_absx_addressing_mode_with_page_cross() {
        let mut cpu = CPU::new();
        let mut mock_bus = MockBus::new();

        cpu.x = 0x2;

        mock_bus.expect_read()
            .with(eq(0x1))
            .return_const(0x88);

        let result = cpu.absx(0xffff, &mock_bus);
        assert_eq!(AddrModeResult {
            data: 0x88,
            cycles: 3,
            mode: AddrMode::ABSX,
            addr: Some(0x1)
        }, result);
    }
}