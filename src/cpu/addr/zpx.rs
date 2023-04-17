use crate::cpu::{CPU, bus::Bus};

use super::AddrModeResult;

impl CPU {
    pub (in crate::cpu) fn zpx(&self, addr: u8, bus: &dyn Bus) 
        -> AddrModeResult {
            let resolved_addr = addr.wrapping_add(self.x) as u16;
        AddrModeResult {
            data: Some(bus.read(resolved_addr)),
            cycles: 2,
            mode: super::AddrMode::ZPX,
            addr: Some(resolved_addr)
        }
    }
}

#[cfg(test)]
mod zpx_tests {
    use mockall::predicate::eq;

    use crate::cpu::{bus::MockBus, addr::AddrModeResult};
    use super::*;

    #[test]
    fn test_zpx_addressing_mode() {
        let mut cpu = CPU::new();
        let mut mock_bus = MockBus::new();
        cpu.x = 0x2;

        mock_bus.expect_read()
            .with(eq(0x1))
            .return_const(0x77);

        let result = cpu.zpx(0xff, &mock_bus);
        assert_eq!(AddrModeResult {
            data: Some(0x77),
            cycles: 2,
            mode: crate::cpu::addr::AddrMode::ZPX,
            addr: Some(0x1)
        }, result);
    }
}