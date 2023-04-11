use crate::cpu::bus::Bus;

use super::super::super::CPU;

impl CPU {
    pub(super) fn adc_indy(&mut self, addr: u8, bus: &dyn Bus) -> u8 {
        let low_byte_addr: u8 = bus.read(addr as u16);
        let high_byte_addr: u8 = bus.read(addr.wrapping_add(1) as u16);

        let page_before: u8 = high_byte_addr;

        let resolved_addr: u16 = ((high_byte_addr as u16) << 8) | low_byte_addr as u16;
        let resolved_addr: u16 = resolved_addr.wrapping_add(self.y as u16);

        let page_after: u8 = (resolved_addr >> 8) as u8;

        if page_before == page_after { 
            1 + self.adc_abs(resolved_addr, bus) 
        } else { 
            2 + self.adc_abs(resolved_addr, bus) 
        }
    }
}

#[cfg(test)]
mod adc_indy_tests {
    use mockall::predicate::eq;

    use crate::cpu::bus::MockBus;

    use super::*;

    #[test]
    fn test_adc_indy_correct_cycles_no_page_cross() {
        let mut cpu = CPU::new();

        let mut mock_bus = MockBus::new();
        mock_bus.expect_read().return_const(0x0);

        assert_eq!(5, cpu.adc_indy(0x00, &mock_bus));
    }

    #[test]
    fn test_adc_indy_correct_cycles_page_cross() {
        let mut cpu = CPU::new();
        let mut mock_bus = MockBus::new();

        mock_bus.expect_read()
            .with(eq(0x80))
            .return_const(0x11);

        mock_bus.expect_read()
            .with(eq(0x81))
            .return_const(0x15);

        mock_bus.expect_read().return_const(0x0);

        cpu.y = 0xff;

        assert_eq!(6, cpu.adc_indy(0x80, &mock_bus));
    }

    #[test]
    fn test_adc_indy_end_of_zero_page() {
        let mut cpu = CPU::new();
        let mut mock_bus = MockBus::new();

        cpu.y = 0x2;

        mock_bus.expect_read()
            .with(eq(0x0))
            .return_const(0xff);
        mock_bus.expect_read()
            .with(eq(0xff))
            .return_const(0xff);
        mock_bus.expect_read()
            .with(eq(0x1))
            .return_const(0xee);

        cpu.adc_indy(0xff, &mock_bus);

        assert_eq!(0xee, cpu.a);
    }

    #[test]
    fn test_adc_indy_carry_flag() {
        let mut cpu = CPU::new();
        let mut mock_bus = MockBus::new();

        mock_bus.expect_read()
            .with(eq(0x0))
            .return_const(0x10);
        mock_bus.expect_read()
            .with(eq(0x1))
            .return_const(0x10);
        mock_bus.expect_read()
            .with(eq(0x1010))
            .return_const(0x80);

        cpu.adc_indy(0x00, &mock_bus);
        assert_eq!(false, cpu.c);

        cpu.adc_indy(0x00, &mock_bus);
        assert_eq!(true, cpu.c);

        cpu.adc_indy(0x00, &mock_bus);
        assert_eq!(false, cpu.c);
    }

    #[test]
    fn test_adc_indy_zero_flag() {
        let mut cpu = CPU::new();
        let mut mock_bus = MockBus::new();

        mock_bus.expect_read()
            .with(eq(0x0))
            .return_const(0x10);
        mock_bus.expect_read()
            .with(eq(0x1))
            .return_const(0x10);
        mock_bus.expect_read()
            .with(eq(0x1010))
            .return_const(0x80);

        mock_bus.expect_read().return_const(0x0);

        cpu.adc_indy(0x00, &mock_bus);
        assert_eq!(false, cpu.z);

        cpu.adc_indy(0x00, &mock_bus);
        assert_eq!(true, cpu.z);

        cpu.adc_indy(0x00, &mock_bus);
        assert_eq!(false, cpu.z);
    }

    #[test]
    fn test_adc_indy_negative_flag() {
        let mut cpu = CPU::new();
        let mut mock_bus = MockBus::new();

        mock_bus.expect_read()
            .with(eq(0x0))
            .return_const(0x10);
        mock_bus.expect_read()
            .with(eq(0x1))
            .return_const(0x10);
        mock_bus.expect_read()
            .with(eq(0x1010))
            .return_const(0x80);

        cpu.adc_indy(0x00, &mock_bus);
        assert_eq!(true, cpu.n);

        cpu.adc_indy(0x00, &mock_bus);
        assert_eq!(false, cpu.n);

        cpu.adc_indy(0x00, &mock_bus);
        assert_eq!(true, cpu.n);
    }

    #[test]
    fn test_adc_indy_overflow_flag() {
        let mut cpu = CPU::new();
        let mut mock_bus = MockBus::new();

        cpu.a = 0x7f; //+ve
        mock_bus.expect_read()
            .with(eq(0x0))
            .return_const(0x10);
        mock_bus.expect_read()
            .with(eq(0x1))
            .return_const(0x10);
        mock_bus.expect_read()
            .with(eq(0x1010))
            .return_const(0x7f); //+ve

        cpu.adc_indy(0x0, &mock_bus);

        assert_eq!(true, cpu.n);
        assert_eq!(true, cpu.v);

        let mut mock_bus = MockBus::new();

        cpu.a = 0x81; //-ve
        mock_bus.expect_read()
            .with(eq(0x0))
            .return_const(0x10);
        mock_bus.expect_read()
            .with(eq(0x1))
            .return_const(0x10);
        mock_bus.expect_read()
            .with(eq(0x1010))
            .return_const(0x81); //-ve

        cpu.adc_indy(0x0, &mock_bus);

        assert_eq!(false, cpu.n);
        assert_eq!(true, cpu.v);

        let mut mock_bus = MockBus::new();

        cpu.a = 0x2; //+ve
        mock_bus.expect_read()
            .with(eq(0x0))
            .return_const(0x10);
        mock_bus.expect_read()
            .with(eq(0x1))
            .return_const(0x10);
        mock_bus.expect_read()
            .with(eq(0x1010))
            .return_const(0xff); //-ve

        cpu.adc_indy(0x0, &mock_bus);

        assert_eq!(false, cpu.n);
        assert_eq!(false, cpu.v);

        let mut mock_bus = MockBus::new();

        cpu.a = 0xf0; //-ve
        mock_bus.expect_read()
            .with(eq(0x0))
            .return_const(0x10);
        mock_bus.expect_read()
            .with(eq(0x1))
            .return_const(0x10);
        mock_bus.expect_read()
            .with(eq(0x1010))
            .return_const(0x1); //+ve

        cpu.adc_indy(0x0, &mock_bus);

        assert_eq!(true, cpu.n);
        assert_eq!(false, cpu.v);
    }
}
