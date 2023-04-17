use crate::cpu::{CPU, bus::Bus};

use super::{AddrMode, AddrModeResult};

impl CPU {
    pub(in crate::cpu) fn indy(&self, addr: u8, bus: &dyn Bus) 
        -> AddrModeResult {
        let low_byte_addr = addr;
        let high_byte_addr = low_byte_addr.wrapping_add(1);

        let resolved_addr = (bus.read(high_byte_addr as u16) as u16) << 8 
            | (bus.read(low_byte_addr as u16) as u16);

        let page_before = (resolved_addr >> 8) as u8;
        let resolved_addr = resolved_addr + self.y as u16;
        let page_after = (resolved_addr >> 8) as u8;
        
        let mut result = self.abs(resolved_addr, bus);
        result.cycles += 1 + ((page_before != page_after) as u8);
        result.mode = AddrMode::INDY;
        result
    }
}

#[cfg(test)]
mod indy_tests {
    use mockall::predicate::eq;

    use crate::cpu::{bus::MockBus, addr::AddrModeResult};
    use super::*;

    #[test]
    fn test_indy_addressing_mode_no_page_cross() {
        let mut cpu = CPU::new();
        let mut mock_bus = MockBus::new();

        mock_bus.expect_read()
            .with(eq(0xff))
            .return_const(0x77);
        mock_bus.expect_read()
            .with(eq(0x0))
            .return_const(0x88);

        cpu.y = 0x2;
        mock_bus.expect_read()
            .with(eq(0x8879))
            .return_const(0xbb);

        let result = cpu.indy(0xff, &mock_bus);
        assert_eq!(AddrModeResult {
            data: 0xbb,
            cycles: 3,
            mode: crate::cpu::addr::AddrMode::INDY
        }, result);
    }

    #[test]
    fn test_indy_addressing_mode_with_page_cross() {
        let mut cpu = CPU::new();
        let mut mock_bus = MockBus::new();

        mock_bus.expect_read()
            .with(eq(0xff))
            .return_const(0x77);
        mock_bus.expect_read()
            .with(eq(0x0))
            .return_const(0x88);

        cpu.y = 0xff;
        mock_bus.expect_read()
            .with(eq(0x8976))
            .return_const(0xcc);

        let result = cpu.indy(0xff, &mock_bus);
        assert_eq!(AddrModeResult {
            data: 0xcc,
            cycles: 4,
            mode: crate::cpu::addr::AddrMode::INDY
        }, result);
    }
}