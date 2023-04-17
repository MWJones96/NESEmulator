/*
    ADC - Add Memory to Accumulator with Carry

    Operation: A + M + C → A, C

    This instruction adds the value of memory and carry from the previous operation to the value
    of the accumulator and stores the result in the accumulator.

    This instruction affects the accumulator; sets the carry flag when the sum of a binary add
    exceeds 255 or when the sum of a decimal add exceeds 99, otherwise carry is reset.

    The overflow flag is set when the sign or bit 7 is changed due to the result exceeding
    +127 or -128, otherwise overflow is reset.

    The negative flag is set if the accumulator result contains bit 7 on, otherwise the
    negative flag is reset.

    The zero flag is set if the accumulator result is 0, otherwise the zero flag is reset.
*/

use crate::cpu::addr::AddrModeResult;

use super::super::CPU;

impl CPU {
    fn adc(&mut self, mode: AddrModeResult) -> u8 {
        let adc_cycles = self._adc(mode.data.unwrap());
        adc_cycles + mode.cycles
    }

    fn _adc(&mut self, imm: u8) -> u8 {
        let a: u16 = self.a as u16;
        let v: u16 = imm as u16;

        let s: u16 = a + v + self.c as u16;

        self.a = s as u8;

        self.c = s > 0xff;
        self.z = self.a == 0_u8;
        self.n = (self.a & 0b_1000_0000_u8) > 0;
        self.v = ((a ^ s) & (v ^ s) & 0x80) > 0;

        2
    }
}

#[cfg(test)]
mod adc_tests {
    use mockall::predicate::eq;

    use crate::cpu::bus::MockBus;

    use super::*;

    #[test]
    fn test_adc_imm_correct_cycles() {
        let mut cpu = CPU::new();
        let cycles: u8 = cpu.adc(cpu.imm(0x0));
        assert_eq!(2, cycles);
    }

    #[test]
    fn test_adc_zp_correct_cycles() {
        let mut cpu = CPU::new();
        let mut bus = MockBus::new();
        bus.expect_read().return_const(0x0);

        let cycles: u8 = cpu.adc(cpu.zp(0x0, &bus));
        assert_eq!(3, cycles);
    }

    #[test]
    fn test_adc_zpx_correct_cycles() {
        let mut cpu = CPU::new();
        let mut bus = MockBus::new();
        bus.expect_read().return_const(0x0);

        let cycles: u8 = cpu.adc(cpu.zpx(0x0, &bus));
        assert_eq!(4, cycles);
    }

    #[test]
    fn test_adc_abs_correct_cycles() {
        let mut cpu = CPU::new();
        let mut bus = MockBus::new();
        bus.expect_read().return_const(0x0);

        let cycles: u8 = cpu.adc(cpu.abs(0x0, &bus));
        assert_eq!(4, cycles);
    }

    #[test]
    fn test_adc_absx_correct_cycles_no_page_cross() {
        let mut cpu = CPU::new();
        let mut bus = MockBus::new();
        bus.expect_read().return_const(0x0);

        let cycles: u8 = cpu.adc(cpu.absx(0x0, &bus));
        assert_eq!(4, cycles);
    }

    #[test]
    fn test_adc_absx_correct_cycles_with_page_cross() {
        let mut cpu = CPU::new();
        let mut bus = MockBus::new();
        bus.expect_read().return_const(0x0);

        cpu.x = 0xff;
        let cycles: u8 = cpu.adc(cpu.absx(0x88, &bus));
        assert_eq!(5, cycles);
    }

    #[test]
    fn test_adc_absy_correct_cycles_no_page_cross() {
        let mut cpu = CPU::new();
        let mut bus = MockBus::new();
        bus.expect_read().return_const(0x0);

        let cycles: u8 = cpu.adc(cpu.absy(0x88, &bus));
        assert_eq!(4, cycles);
    }

    #[test]
    fn test_adc_absy_correct_cycles_with_page_cross() {
        let mut cpu = CPU::new();
        let mut bus = MockBus::new();
        bus.expect_read().return_const(0x0);

        cpu.y = 0xff;
        let cycles: u8 = cpu.adc(cpu.absy(0x88, &bus));
        assert_eq!(5, cycles);
    }

    #[test]
    fn test_adc_indx_correct_cycles() {
        let mut cpu = CPU::new();
        let mut bus = MockBus::new();
        bus.expect_read().return_const(0x0);

        let cycles: u8 = cpu.adc(cpu.indx(0x88, &bus));
        assert_eq!(6, cycles);
    }

    #[test]
    fn test_adc_indy_correct_cycles_no_page_cross() {
        let mut cpu = CPU::new();
        let mut bus = MockBus::new();
        bus.expect_read().return_const(0x0);

        let cycles: u8 = cpu.adc(cpu.indy(0x88, &bus));
        assert_eq!(5, cycles);
    }

    #[test]
    fn test_adc_indy_correct_cycles_with_page_cross() {
        let mut cpu = CPU::new();
        let mut bus = MockBus::new();
        bus.expect_read().with(eq(0x88)).return_const(0x11);
        bus.expect_read().with(eq(0x89)).return_const(0x22);

        bus.expect_read().with(eq(0x2310)).return_const(0x0);

        cpu.y = 0xff;
        let cycles: u8 = cpu.adc(cpu.indy(0x88, &bus));
        assert_eq!(6, cycles);
    }

    #[test]
    fn test_adc_no_carry() {
        let mut cpu = CPU::new();

        cpu._adc(0x01_u8);

        assert_eq!(0x01_u8, cpu.a);
        assert_eq!(false, cpu.c);
    }

    #[test]
    fn test_adc_with_carry() {
        let mut cpu = CPU::new();
        cpu.a = 0x80_u8;

        cpu._adc(0x80_u8);

        assert_eq!(0x00_u8, cpu.a);
        assert_eq!(true, cpu.c);

        cpu._adc(0x80_u8);

        assert_eq!(0x81, cpu.a);
        assert_eq!(false, cpu.c);

        cpu._adc(0x01_u8);

        assert_eq!(0x82, cpu.a);
        assert_eq!(false, cpu.c);
    }

    #[test]
    fn test_adc_with_carry_zero_flag() {
        let mut cpu = CPU::new();

        cpu._adc(0x00_u8);

        assert_eq!(true, cpu.z);

        cpu._adc(0x80_u8);

        assert_eq!(false, cpu.z);

        cpu._adc(0x80_u8);

        assert_eq!(true, cpu.z);
    }

    #[test]
    fn test_adc_with_negative_flag() {
        let mut cpu = CPU::new();

        cpu._adc(0b_0111_1111_u8);

        assert_eq!(false, cpu.n);

        cpu._adc(0b_0000_0001_u8);

        assert_eq!(true, cpu.n);
    }

    #[test]
    fn test_adc_with_overflow_flag() {
        let mut cpu = CPU::new();

        cpu.a = 0x7f; //+ve
        cpu._adc(0x1_u8); //+ve

        assert_eq!(true, cpu.n);
        assert_eq!(true, cpu.v);

        cpu.a = 0x80; //-ve
        cpu._adc(0x80_u8); //-ve

        assert_eq!(false, cpu.n);
        assert_eq!(true, cpu.v);

        cpu.a = 0x1; //+ve
        cpu._adc(0xf0_u8); //-ve

        assert_eq!(true, cpu.n);
        assert_eq!(false, cpu.v);

        cpu.a = 0xff; //-ve
        cpu._adc(0x2_u8); //+ve

        assert_eq!(false, cpu.n);
        assert_eq!(false, cpu.v);
    }
}
