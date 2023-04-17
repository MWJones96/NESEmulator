use crate::cpu::{CPU, bus::Bus};

use super::AddrModeResult;

impl CPU {
    pub (in crate::cpu) fn zp(&self, addr: u8, bus: &dyn Bus) 
        -> AddrModeResult {
        AddrModeResult { 
            data: bus.read(addr as u16), 
            cycles: 1, 
            mode: super::AddrMode::ZP,
            addr: Some(addr as u16)
        }
    }
}

#[cfg(test)]
mod zp_tests {
    use mockall::predicate::eq;

    use crate::cpu::{bus::MockBus, addr::AddrModeResult};

    use super::*;

    #[test]
    fn test_zp_addressing_mode() {
        let cpu = CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read()
            .with(eq(0x0))
            .return_const(0x77);

        let result = cpu.zp(0x0, &mock_bus);
        assert_eq!(AddrModeResult {
            data: 0x77,
            cycles: 1,
            mode: crate::cpu::addr::AddrMode::ZP,
            addr: Some(0x0)
        }, result);
    }
}