use super::super::super::CPU;

impl CPU {
    pub(super) fn adc_zpx(&mut self, addr: u8, mem: &[u8]) -> u8 {
        1 + self.adc_zp(addr.wrapping_add(self.x), mem)
    }
}

#[cfg(test)]
mod adc_zpx_tests {
    use super::*;

    #[test]
    fn test_adc_zpx_correct_cycles() {
        let mut cpu = CPU::new();

        assert_eq!(4, cpu.adc_zpx(0x00_u8, &[0x00_u8]));
    }

    #[test]
    fn test_adc_zpx_with_x_set_to_zero() {
        let mut cpu = CPU::new();
        cpu.x = 0;

        cpu.adc_zpx(0x00_u8, &[0x77_u8]);

        assert_eq!(0x77_u8, cpu.a);
    }

    #[test]
    fn test_adc_zpx_with_x_overflow() {
        let mut cpu = CPU::new();
        cpu.x = 0xff_u8;

        cpu.adc_zpx(0x01_u8, &[0x77_u8]);

        assert_eq!(0x77_u8, cpu.a);
    }

    #[test]
    fn test_adc_zpx_with_carry_flag() {
        let mut cpu = CPU::new();

        cpu.adc_zpx(0x00_u8, &[0x80_u8]);

        assert_eq!(false, cpu.c);

        cpu.adc_zpx(0x00_u8, &[0x80_u8]);

        assert_eq!(true, cpu.c);

        cpu.adc_zpx(0x00_u8, &[0x80_u8]);

        assert_eq!(false, cpu.c);
    }

    #[test]
    fn test_adc_zpx_with_zero_flag() {
        let mut cpu = CPU::new();

        cpu.adc_zpx(0x00_u8, &[0x00_u8]);
        assert_eq!(true, cpu.z);

        cpu.adc_zpx(0x00_u8, &[0x80_u8]);
        assert_eq!(false, cpu.z);

        cpu.adc_zpx(0x00_u8, &[0x80_u8]);
        assert_eq!(true, cpu.z);
    }

    #[test]
    fn test_adc_zpx_with_negative_flag() {
        let mut cpu = CPU::new();

        cpu.adc_zpx(0x00_u8, &[0x80_u8]);
        assert_eq!(true, cpu.n);

        cpu.adc_zpx(0x00_u8, &[0x80_u8]);
        assert_eq!(false, cpu.n);
    }

    #[test]
    fn test_adc_zpx_with_overflow_flag() {
        let mut cpu = CPU::new();
        cpu.x = 0x1;

        cpu.a = 0x7f; //+ve
        cpu.adc_zpx(0x00_u8, &[0x0, 0x1_u8]); //+ve

        assert_eq!(true, cpu.n);
        assert_eq!(true, cpu.v);

        cpu.a = 0x80; //-ve
        cpu.adc_zpx(0x00_u8, &[0x0, 0x80_u8]); //-ve

        assert_eq!(false, cpu.n);
        assert_eq!(true, cpu.v);

        cpu.a = 0x1; //+ve
        cpu.adc_zpx(0x00_u8, &[0x0, 0xf0_u8]); //-ve

        assert_eq!(true, cpu.n);
        assert_eq!(false, cpu.v);

        cpu.a = 0xff; //-ve
        cpu.adc_zpx(0x00_u8, &[0x0, 0x2_u8]); //+ve

        assert_eq!(false, cpu.n);
        assert_eq!(false, cpu.v);
    }

    #[test]
    fn test_adc_zpx_with_different_memory_address() {
        let mut cpu = CPU::new();
        cpu.x = 1;

        cpu.adc_zpx(0x00_u8, &[0x00_u8, 0x10_u8]);

        assert_eq!(0x10_u8, cpu.a);
    }
}
