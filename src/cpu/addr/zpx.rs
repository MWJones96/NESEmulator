use crate::cpu::{CPU, bus::Bus};

impl CPU {
    pub (in crate::cpu) fn zpx(&self, addr: u8, bus: &dyn Bus) -> (u8, u8) {
        (2, bus.read(addr.wrapping_add(self.x) as u16))
    }
}

#[cfg(test)]
mod zpx_tests {
    use mockall::predicate::eq;

    use crate::cpu::bus::MockBus;
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
        assert_eq!((2, 0x77), result);
    }
}