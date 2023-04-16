use crate::cpu::{CPU, bus::Bus};

impl CPU {
    pub(in crate::cpu) fn absy(&self, addr: u16, bus: &dyn Bus) -> (u8, u8) {
        self.abs_helper(addr, self.y, bus)
    }
}

#[cfg(test)]
mod absy_tests {
    use mockall::predicate::eq;

    use crate::cpu::bus::MockBus;
    use super::*;

    #[test]
    fn test_absy_addressing_mode_no_page_cross() {
        let mut cpu = CPU::new();
        let mut mock_bus = MockBus::new();

        cpu.y = 0x2;

        mock_bus.expect_read()
            .with(eq(0x2))
            .return_const(0x88);

        let result = cpu.absy(0x0, &mock_bus);
        assert_eq!((2, 0x88), result);
    }

    #[test]
    fn test_absy_addressing_mode_with_page_cross() {
        let mut cpu = CPU::new();
        let mut mock_bus = MockBus::new();

        cpu.y = 0x2;

        mock_bus.expect_read()
            .with(eq(0x1))
            .return_const(0x88);

            let result = cpu.absy(0xffff, &mock_bus);
            assert_eq!((3, 0x88), result);
    }
}