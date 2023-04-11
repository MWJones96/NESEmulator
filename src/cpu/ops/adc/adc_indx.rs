use crate::cpu::bus::Bus;

use super::super::super::CPU;

impl CPU {
    pub(super) fn adc_indx(&mut self, addr: u8, bus: &dyn Bus) -> u8 {
        let low_byte_addr: u8 = addr.wrapping_add(self.x);
        let high_byte_addr: u8 = low_byte_addr.wrapping_add(1);

        let resolved_addr: u16 = 
            ((bus.read(high_byte_addr as u16) as u16) << 8) 
                | (bus.read(low_byte_addr as u16) as u16);

        2 + self.adc_abs(resolved_addr, bus)
    }
}

#[cfg(test)]
mod adc_indx_tests {
    use mockall::predicate::eq;

    use crate::cpu::bus::MockBus;

    use super::*;

    #[test]
    fn test_adc_indx_correct_cycles() {
        let mut cpu = CPU::new();

        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().return_const(0x0);

        assert_eq!(6, cpu.adc_indx(0x00, &mock_bus));
    }

    #[test]
    fn test_adc_indx() {
        let mut cpu = CPU::new();
        let mut mock_bus = MockBus::new();

        //Lower byte
        mock_bus.expect_read()
            .with(eq(0x0))
            .return_const(0x40);

        //Upper byte
        mock_bus.expect_read()
            .with(eq(0x1))
            .return_const(0x20);

        mock_bus.expect_read()
            .with(eq(0x2040))
            .return_const(0x77);

        cpu.adc_indx(0x00, &mock_bus);
        assert_eq!(0x77, cpu.a);
    }

    #[test]
    fn test_adc_indx_wrap() {
        let mut cpu = CPU::new();
        let mut mock_bus = MockBus::new();

        mock_bus.expect_read()
            .with(eq(0xff))
            .return_const(0x77);

        mock_bus.expect_read()
            .with(eq(0x0))
            .return_const(0x88);

        mock_bus.expect_read()
            .with(eq(0x8877))
            .return_const(0x11);

        cpu.adc_indx(0xff, &mock_bus);
        assert_eq!(0x11, cpu.a);
    }

    #[test]
    fn test_adc_indx_wrap_with_x() {
        let mut cpu = CPU::new();
        let mut mock_bus = MockBus::new();

        cpu.x = 0x1;

        mock_bus.expect_read()
            .with(eq(0x0))
            .return_const(0x77);

        mock_bus.expect_read()
            .with(eq(0x1))
            .return_const(0x88);

        mock_bus.expect_read()
            .with(eq(0x8877))
            .return_const(0x22);

        cpu.adc_indx(0xff, &mock_bus);
        assert_eq!(0x22, cpu.a);
    }

    #[test]
    fn test_adc_indx_wrap_with_x_twice() {
        let mut cpu = CPU::new();
        let mut mock_bus = MockBus::new();

        cpu.x = 0x1;

        mock_bus.expect_read()
            .with(eq(0x0))
            .return_const(0x77);
        mock_bus.expect_read()
            .with(eq(0x1))
            .return_const(0x88);
        mock_bus.expect_read()
            .with(eq(0x8877))
            .return_const(0x22);

        cpu.adc_indx(0xff, &mock_bus);
        cpu.adc_indx(0xff, &mock_bus);

        assert_eq!(0x44, cpu.a);
    }

    #[test]
    fn test_adc_indx_carry_flag() {
        let mut cpu = CPU::new();
        let mut mock_bus = MockBus::new();
        cpu.x = 0x1;

        mock_bus.expect_read()
            .with(eq(0x0))
            .return_const(0x77);
        mock_bus.expect_read()
            .with(eq(0x1))
            .return_const(0x88);
        mock_bus.expect_read()
            .with(eq(0x8877))
            .return_const(0x80);

        cpu.adc_indx(0xff, &mock_bus);
        assert_eq!(false, cpu.c);

        cpu.adc_indx(0xff, &mock_bus);

        assert_eq!(true, cpu.c);

        cpu.adc_indx(0xff, &mock_bus);

        assert_eq!(false, cpu.c);
    }

    #[test]
    fn test_adc_indx_zero_flag() {
        let mut cpu = CPU::new();
        let mut mock_bus = MockBus::new();
        cpu.x = 0x1;

        mock_bus.expect_read()
            .with(eq(0x0))
            .return_const(0x77);
        mock_bus.expect_read()
            .with(eq(0x1))
            .return_const(0x88);
        mock_bus.expect_read()
            .with(eq(0x8877))
            .return_const(0x80);

        cpu.adc_indx(0xff, &mock_bus);
        assert_eq!(false, cpu.z);

        cpu.adc_indx(0xff, &mock_bus);
        assert_eq!(true, cpu.z);

        cpu.adc_indx(0xff, &mock_bus);
        assert_eq!(false, cpu.z);
    }

    #[test]
    fn test_adc_indx_negative_flag() {
        let mut cpu = CPU::new();
        let mut mock_bus = MockBus::new();
        cpu.x = 0x1;

        mock_bus.expect_read()
            .with(eq(0x0))
            .return_const(0x77);
        mock_bus.expect_read()
            .with(eq(0x1))
            .return_const(0x88);
        mock_bus.expect_read()
            .with(eq(0x8877))
            .return_const(0x80);

        cpu.adc_indx(0xff, &mock_bus);
        assert_eq!(true, cpu.n);

        cpu.adc_indx(0xff, &mock_bus);
        assert_eq!(false, cpu.n);

        cpu.adc_indx(0xff, &mock_bus);
        assert_eq!(true, cpu.n);
    }

    #[test]
    fn test_adc_indx_overflow_flag() {
        let mut cpu = CPU::new();
        let mut mock_bus = MockBus::new();

        mock_bus.expect_read()
            .with(eq(0x0))
            .return_const(0x77);
        mock_bus.expect_read()
            .with(eq(0x1))
            .return_const(0x88);

        cpu.a = 0x7f; //+ve
        mock_bus.expect_read()
            .with(eq(0x8877))
            .return_const(0x7f); //+ve
        cpu.adc_indx(0x00, &mock_bus);

        assert_eq!(true, cpu.n);
        assert_eq!(true, cpu.v);

        cpu.a = 0x81; //-ve
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read()
            .with(eq(0x0))
            .return_const(0x77);
        mock_bus.expect_read()
            .with(eq(0x1))
            .return_const(0x88);
        mock_bus.expect_read()
            .with(eq(0x8877))
            .return_const(0x81); //-ve
        cpu.adc_indx(0x00, &mock_bus);

        assert_eq!(false, cpu.n);
        assert_eq!(true, cpu.v);

        cpu.a = 0x1; //+ve
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read()
            .with(eq(0x0))
            .return_const(0x77);
        mock_bus.expect_read()
            .with(eq(0x1))
            .return_const(0x88);
        mock_bus.expect_read()
            .with(eq(0x8877))
            .return_const(0xf0); //-ve
        cpu.adc_indx(0x00, &mock_bus);

        assert_eq!(true, cpu.n);
        assert_eq!(false, cpu.v);

        cpu.a = 0xff; //-ve
        let mut mock_bus = MockBus::new();
        mock_bus.expect_read()
            .with(eq(0x0))
            .return_const(0x77);
        mock_bus.expect_read()
            .with(eq(0x1))
            .return_const(0x88);
        mock_bus.expect_read()
            .with(eq(0x8877))
            .return_const(0x2); //+ve
        cpu.adc_indx(0x00, &mock_bus);

        assert_eq!(false, cpu.n);
        assert_eq!(false, cpu.v);
    }
}
