use super::super::super::CPU;
use super::super::super::bus::Bus;

impl CPU {
    pub(super) fn adc_abs(&mut self, addr: u16, bus: &dyn Bus) -> u8 {
        2 + self.adc_imm(bus.read(addr))
    }
}

#[cfg(test)]
mod adc_abs_tests {
    use mockall::predicate::eq;
    use crate::cpu::bus::MockBus;

    use super::*;

    #[test]
    fn test_adc_abs_correct_cycles() {
        let mut cpu = CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().return_const(0x0);

        assert_eq!(4, cpu.adc_abs(0x0000_u16, &mock_bus));
    }

    #[test]
    fn test_adc_abs_fetch_mem_addr() {
        let mut cpu = CPU::new();

        let mut mock_bus = MockBus::new();
        mock_bus.expect_read()
            .with(eq(0xffff))
            .return_const(0x77);

        cpu.adc_abs(0xffff_u16, &mock_bus);

        assert_eq!(0x77_u8, cpu.a);
    }

    #[test]
    fn test_adc_abs_carry_flag() {
        let mut cpu = CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read()
            .with(eq(0x0))
            .return_const(0x80);

        cpu.adc_abs(0x0, &mock_bus);

        assert_eq!(0x80_u8, cpu.a);
        assert_eq!(false, cpu.c);

        cpu.adc_abs(0x0, &mock_bus);

        assert_eq!(0x00_u8, cpu.a);
        assert_eq!(true, cpu.c);

        cpu.adc_abs(0x0, &mock_bus);

        assert_eq!(0x81_u8, cpu.a);
        assert_eq!(false, cpu.c);
    }

    #[test]
    fn test_adc_abs_zero_flag() {
        let mut cpu = CPU::new();

        let mut mock_bus = MockBus::new();
        mock_bus.expect_read()
            .with(eq(0x0))
            .return_const(0x80);

        cpu.adc_abs(0x0, &mock_bus);

        assert_eq!(false, cpu.z);

        cpu.adc_abs(0x0, &mock_bus);

        assert_eq!(true, cpu.z);

        cpu.adc_abs(0x0, &mock_bus);

        assert_eq!(false, cpu.z);
    }

    #[test]
    fn test_adc_abs_negative_flag() {
        let mut cpu = CPU::new();
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read()
            .with(eq(0x0))
            .return_const(0x80);

        cpu.adc_abs(0x0, &mock_bus);

        assert_eq!(true, cpu.n);

        cpu.adc_abs(0x0, &mock_bus);

        assert_eq!(false, cpu.n);
    }

    #[test]
    fn test_adc_abs_overflow_flag() {
        let mut cpu = CPU::new();
        let mut mock_bus = MockBus::new();

        cpu.a = 0x7f; //+ve
        mock_bus.expect_read()
            .with(eq(0x0))
            .returning(|_| 0x1); //+ve

        cpu.adc_abs(0x0, &mock_bus);

        assert_eq!(true, cpu.n);
        assert_eq!(true, cpu.v);

        cpu.a = 0x80; //-ve
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read()
            .with(eq(0x0))
            .returning(|_| 0xff); //-ve

        cpu.adc_abs(0x0, &mock_bus);

        assert_eq!(false, cpu.n);
        assert_eq!(true, cpu.v);

        cpu.a = 0x1; //+ve
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read()
            .with(eq(0x0))
            .returning(|_| 0xf0); //-ve

        cpu.adc_abs(0x0, &mock_bus);

        assert_eq!(true, cpu.n);
        assert_eq!(false, cpu.v);

        cpu.a = 0xff; //-ve
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read()
            .with(eq(0x0))
            .returning(|_| 0x2); //+ve

        cpu.adc_abs(0x0, &mock_bus);

        assert_eq!(false, cpu.n);
        assert_eq!(false, cpu.v);
    }
}
