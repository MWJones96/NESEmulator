use crate::cpu::{CPU, bus::Bus};

impl CPU {
    pub(in crate::cpu) fn absx(&self, addr: u16, bus: &dyn Bus) -> (u8, u8) {
        self.abs_helper(addr, self.x, bus)
    }
}

#[cfg(test)]
mod absx_tests {
    use mockall::predicate::eq;

    use crate::cpu::bus::MockBus;
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
        assert_eq!((2, 0x88), result);
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
            assert_eq!((3, 0x88), result);
    }
}