use crate::cpu::{bus::Bus, CPU};

use super::{AddrMode, AddrModeResult};

impl CPU {
    pub(in crate::cpu) fn zpy(&self, addr: u8, bus: &dyn Bus) -> AddrModeResult {
        let resolved_addr = addr.wrapping_add(self.y) as u16;

        AddrModeResult {
            data: Some(bus.read(resolved_addr)),
            cycles: 2,
            mode: AddrMode::ZPY,
            addr: Some(resolved_addr),
        }
    }
}

#[cfg(test)]
mod zpy_tests {
    use mockall::predicate::eq;

    use super::*;
    use crate::cpu::{addr::AddrModeResult, bus::MockBus};

    #[test]
    fn test_zpy_addressing_mode() {
        let mut cpu = CPU::new();
        let mut mock_bus = MockBus::new();
        cpu.y = 0x2;

        mock_bus.expect_read().with(eq(0x1)).return_const(0x77);

        let result = cpu.zpy(0xff, &mock_bus);
        assert_eq!(
            AddrModeResult {
                data: Some(0x77),
                cycles: 2,
                mode: AddrMode::ZPY,
                addr: Some(0x1)
            },
            result
        );
    }
}
