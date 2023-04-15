use crate::cpu::{CPU, bus::Bus};

impl CPU {
    fn abs(&self, addr: u16, bus: &dyn Bus) -> (u8, u8) {
        (2, bus.read(addr))
    }
}

#[cfg(test)]
mod abs_tests {
    use mockall::predicate::eq;

    use crate::cpu::bus::MockBus;
    use super::*;

    #[test]
    fn test_abs_addressing_mode() {
        let cpu = CPU::new();
        let mut mock_bus = MockBus::new();

        mock_bus.expect_read()
            .with(eq(0xffff))
            .return_const(0x88);

        let result = cpu.abs(0xffff, &mock_bus);
        assert_eq!((2, 0x88), result);
    }
}