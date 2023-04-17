use crate::cpu::{bus::Bus, CPU};

use super::{AddrMode, AddrModeResult};

impl CPU {
    pub(in crate::cpu) fn indx(&self, addr: u8, bus: &dyn Bus) -> AddrModeResult {
        let low_byte_addr = addr.wrapping_add(self.x);
        let high_byte_addr = low_byte_addr.wrapping_add(1);

        let resolved_addr = (((bus.read(high_byte_addr as u16) as u16) << 8) as u16)
            | bus.read(low_byte_addr as u16) as u16;

        let mut result = self.abs(resolved_addr, bus);
        result.cycles += 2;
        result.mode = AddrMode::INDX;
        result
    }
}

#[cfg(test)]
mod indx_tests {
    use mockall::predicate::eq;

    use super::*;
    use crate::cpu::{addr::AddrModeResult, bus::MockBus};

    #[test]
    fn test_indx_addressing_mode() {
        let mut cpu = CPU::new();
        let mut mock_bus = MockBus::new();

        mock_bus.expect_read().with(eq(0x1)).return_const(0x77);
        mock_bus.expect_read().with(eq(0x2)).return_const(0x88);

        mock_bus.expect_read().with(eq(0x8877)).return_const(0xaa);

        cpu.x = 0x2;
        let result = cpu.indx(0xff, &mock_bus);
        assert_eq!(
            AddrModeResult {
                data: Some(0xaa),
                cycles: 4,
                mode: crate::cpu::addr::AddrMode::INDX,
                addr: Some(0x8877)
            },
            result
        );
    }
}
