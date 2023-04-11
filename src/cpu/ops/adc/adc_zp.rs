use crate::cpu::bus::Bus;

use super::super::super::CPU;

impl CPU {
    pub(super) fn adc_zp(&mut self, addr: u8, bus: &dyn Bus) -> u8 {
        1 + self.adc_imm(bus.read(addr as u16))
    }
}

#[cfg(test)]
mod adc_zp_tests {
    use mockall::predicate::eq;

    use crate::cpu::bus::MockBus;

    use super::*;

    #[test]
    fn test_adc_zp_number_of_cycles() {
        let mut cpu = CPU::new();

        let mut mock_bus = MockBus::new();
        mock_bus.expect_read()
            .return_const(0x0);

        let cycles: u8 = cpu.adc_zp(0x00_u8, &mock_bus);
        assert_eq!(3, cycles);
    }

    #[test]
    fn test_adc_zp() {
        let mut cpu = CPU::new();

        let mut mock_bus = MockBus::new();
        mock_bus.expect_read()
            .with(eq(0x0))
            .return_const(0x81);

        cpu.adc_zp(0x00_u8, &mock_bus);

        assert_eq!(0x81_u8, cpu.a);
    }

    #[test]
    fn test_adc_zp_carry_flag() {
        let mut cpu = CPU::new();

        let mut mock_bus = MockBus::new();
        mock_bus.expect_read()
            .with(eq(0x0))
            .return_const(0x80);

        cpu.adc_zp(0x00_u8, &mock_bus);

        assert_eq!(false, cpu.c);

        let mut mock_bus = MockBus::new();
        mock_bus.expect_read()
            .with(eq(0x0))
            .return_const(0x81);

        cpu.adc_zp(0x00_u8, &mock_bus);

        assert_eq!(true, cpu.c);
    }

    #[test]
    fn test_adc_zp_zero_flag() {
        let mut cpu = CPU::new();

        let mut mock_bus = MockBus::new();
        mock_bus.expect_read()
            .with(eq(0x0))
            .return_const(0x80);

        cpu.adc_zp(0x00_u8, &mock_bus);
        cpu.adc_zp(0x00_u8, &mock_bus);

        assert_eq!(true, cpu.z);

        let mut mock_bus = MockBus::new();
        mock_bus.expect_read()
            .with(eq(0x0))
            .return_const(0x01);

        cpu.adc_zp(0x00_u8, &mock_bus);

        assert_eq!(false, cpu.z);
    }

    #[test]
    fn test_adc_zp_negative_flag() {
        let mut cpu = CPU::new();

        let mut mock_bus = MockBus::new();
        mock_bus.expect_read()
            .with(eq(0x0))
            .return_const(0b_1000_0000_u8);

        cpu.adc_zp(0x00_u8, &mock_bus);

        assert_eq!(true, cpu.n);

        cpu.adc_zp(0x00_u8, &mock_bus);

        assert_eq!(false, cpu.n);
    }

    #[test]
    fn test_adc_zp_overflow_flag() {
        let mut cpu = CPU::new();

        cpu.a = 0x7f; //+ve
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read()
            .with(eq(0x0))
            .return_const(0x1_u8); //+ve
        cpu.adc_zp(0x00_u8, &mock_bus);

        assert_eq!(true, cpu.n);
        assert_eq!(true, cpu.v);

        cpu.a = 0x80; //-ve
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read()
            .with(eq(0x0))
            .return_const(0x80_u8); //-ve
        cpu.adc_zp(0x00_u8, &mock_bus);

        assert_eq!(false, cpu.n);
        assert_eq!(true, cpu.v);

        cpu.a = 0x1; //+ve
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read()
            .with(eq(0x0))
            .return_const(0xf0_u8); //-ve
        cpu.adc_zp(0x00_u8, &mock_bus);

        assert_eq!(true, cpu.n);
        assert_eq!(false, cpu.v);

        cpu.a = 0xff; //-ve

        let mut mock_bus = MockBus::new();
        mock_bus.expect_read()
            .with(eq(0x0))
            .return_const(0x2_u8); //+ve

        cpu.adc_zp(0x00_u8, &mock_bus);

        assert_eq!(false, cpu.n);
        assert_eq!(false, cpu.v);
    }

    #[test]
    fn test_adc_zp_different_mem_address() {
        let mut cpu = CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read()
            .with(eq(0x0))
            .return_const(0b_1111_1111_u8);
        mock_bus.expect_read()
            .with(eq(0x1))
            .return_const(0b_1010_1010_u8);

        cpu.adc_zp(0x01_u8, &mock_bus);

        assert_eq!(0b_1010_1010_u8, cpu.a);
    }
}
