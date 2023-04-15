use crate::cpu::{CPU, bus::Bus};

impl CPU {
    fn zp(&self, addr: u8, bus: &dyn Bus) -> (u8, u8) {
        (1, bus.read(addr as u16))
    }
}

#[cfg(test)]
mod zp_tests {
    use mockall::predicate::eq;

    use crate::cpu::bus::MockBus;

    use super::*;

    #[test]
    fn test_zp_addressing_mode() {
        let cpu = CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read()
            .with(eq(0x0))
            .return_const(0x77);

        let result = cpu.zp(0x0, &mock_bus);
        assert_eq!((1, 0x77), result);
    }
}