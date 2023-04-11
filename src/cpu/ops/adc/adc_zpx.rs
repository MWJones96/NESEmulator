use crate::cpu::bus::Bus;

use super::super::super::CPU;

impl CPU {
    pub(super) fn adc_zpx(&mut self, addr: u8, bus: &dyn Bus) -> u8 {
        1 + self.adc_zp(addr.wrapping_add(self.x), bus)
    }
}

#[cfg(test)]
mod adc_zpx_tests {
    use mockall::predicate::eq;

    use crate::cpu::bus::MockBus;

    use super::*;

    #[test]
    fn test_adc_zpx_correct_cycles() {
        let mut cpu = CPU::new();

        let mut mock_bus = MockBus::new();
        mock_bus.expect_read()
            .with(eq(0x0))
            .return_const(0x00);

        assert_eq!(4, cpu.adc_zpx(0x00_u8, &mock_bus));
    }

    #[test]
    fn test_adc_zpx_with_x_set_to_zero() {
        let mut cpu = CPU::new();
        cpu.x = 0;

        let mut mock_bus = MockBus::new();
        mock_bus.expect_read()
            .with(eq(0x0))
            .return_const(0x77);

        cpu.adc_zpx(0x00_u8, &mock_bus);

        assert_eq!(0x77_u8, cpu.a);
    }

    #[test]
    fn test_adc_zpx_with_x_overflow() {
        let mut cpu = CPU::new();
        cpu.x = 0xff_u8;

        let mut mock_bus = MockBus::new();
        mock_bus.expect_read()
            .with(eq(0x0))
            .return_const(0x77);

        cpu.adc_zpx(0x01_u8, &mock_bus);

        assert_eq!(0x77_u8, cpu.a);
    }

    #[test]
    fn test_adc_zpx_with_carry_flag() {
        let mut cpu = CPU::new();

        let mut mock_bus = MockBus::new();
        mock_bus.expect_read()
            .with(eq(0x0))
            .return_const(0x80);

        cpu.adc_zpx(0x00_u8, &mock_bus);

        assert_eq!(false, cpu.c);

        cpu.adc_zpx(0x00_u8, &mock_bus);

        assert_eq!(true, cpu.c);

        cpu.adc_zpx(0x00_u8, &mock_bus);

        assert_eq!(false, cpu.c);
    }

    #[test]
    fn test_adc_zpx_with_zero_flag() {
        let mut cpu = CPU::new();

        let mut mock_bus = MockBus::new();
        mock_bus.expect_read()
            .with(eq(0x0))
            .return_const(0x00);

        cpu.adc_zpx(0x00_u8, &mock_bus);
        assert_eq!(true, cpu.z);

        let mut mock_bus = MockBus::new();
        mock_bus.expect_read()
            .with(eq(0x0))
            .return_const(0x80);

        cpu.adc_zpx(0x00_u8, &mock_bus);
        assert_eq!(false, cpu.z);

        cpu.adc_zpx(0x00_u8, &mock_bus);
        assert_eq!(true, cpu.z);
    }

    #[test]
    fn test_adc_zpx_with_negative_flag() {
        let mut cpu = CPU::new();

        let mut mock_bus = MockBus::new();
        mock_bus.expect_read()
            .with(eq(0x0))
            .return_const(0x80);

        cpu.adc_zpx(0x00_u8, &mock_bus);
        assert_eq!(true, cpu.n);

        cpu.adc_zpx(0x00_u8, &mock_bus);
        assert_eq!(false, cpu.n);
    }

    #[test]
    fn test_adc_zpx_with_overflow_flag() {
        let mut cpu = CPU::new();
        cpu.x = 0x1;

        cpu.a = 0x7f; //+ve
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read()
            .with(eq(0x0))
            .return_const(0x00);
        mock_bus.expect_read()
            .with(eq(0x1))
            .return_const(0x1);
        cpu.adc_zpx(0x00_u8, &mock_bus); //+ve

        assert_eq!(true, cpu.n);
        assert_eq!(true, cpu.v);

        let mut mock_bus = MockBus::new();
        mock_bus.expect_read()
            .with(eq(0x0))
            .return_const(0x00);
        mock_bus.expect_read()
            .with(eq(0x1))
            .return_const(0x80);

        cpu.a = 0x80; //-ve
        cpu.adc_zpx(0x00_u8, &mock_bus); //-ve

        assert_eq!(false, cpu.n);
        assert_eq!(true, cpu.v);

        let mut mock_bus = MockBus::new();
        mock_bus.expect_read()
            .with(eq(0x0))
            .return_const(0x00);
        mock_bus.expect_read()
            .with(eq(0x1))
            .return_const(0xf0);

        cpu.a = 0x1; //+ve
        cpu.adc_zpx(0x00_u8, &mock_bus); //-ve

        assert_eq!(true, cpu.n);
        assert_eq!(false, cpu.v);

        let mut mock_bus = MockBus::new();
        mock_bus.expect_read()
            .with(eq(0x0))
            .return_const(0x00);
        mock_bus.expect_read()
            .with(eq(0x1))
            .return_const(0x2);

        cpu.a = 0xff; //-ve
        cpu.adc_zpx(0x00_u8, &mock_bus); //+ve

        assert_eq!(false, cpu.n);
        assert_eq!(false, cpu.v);
    }

    #[test]
    fn test_adc_zpx_with_different_memory_address() {
        let mut cpu = CPU::new();
        cpu.x = 1;

        let mut mock_bus = MockBus::new();
        mock_bus.expect_read()
            .with(eq(0x0))
            .return_const(0x00);
        mock_bus.expect_read()
            .with(eq(0x1))
            .return_const(0x10);

        cpu.adc_zpx(0x00_u8, &mock_bus);

        assert_eq!(0x10_u8, cpu.a);
    }
}
