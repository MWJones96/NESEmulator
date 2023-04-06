use super::super::super::CPU;

impl CPU {
    pub(super) fn adc_imm(&mut self, imm: u8) -> u8 {
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
mod adc_imm_tests {
    use super::*;

    #[test]
    fn test_adc_imm_no_carry() {
        let mut cpu = CPU::new();

        cpu.adc_imm(0x01_u8);

        assert_eq!(0x01_u8, cpu.a);
        assert_eq!(false, cpu.c);
    }

    #[test]
    fn test_adc_imm_with_carry() {
        let mut cpu = CPU::new();
        cpu.a = 0x80_u8;

        cpu.adc_imm(0x80_u8);

        assert_eq!(0x00_u8, cpu.a);
        assert_eq!(true, cpu.c);

        cpu.adc_imm(0x80_u8);

        assert_eq!(0x81, cpu.a);
        assert_eq!(false, cpu.c);

        cpu.adc_imm(0x01_u8);

        assert_eq!(0x82, cpu.a);
        assert_eq!(false, cpu.c);
    }

    #[test]
    fn test_adc_imm_with_carry_zero_flag() {
        let mut cpu = CPU::new();

        cpu.adc_imm(0x00_u8);

        assert_eq!(true, cpu.z);

        cpu.adc_imm(0x80_u8);

        assert_eq!(false, cpu.z);

        cpu.adc_imm(0x80_u8);

        assert_eq!(true, cpu.z);
    }

    #[test]
    fn test_adc_imm_with_negative_flag() {
        let mut cpu = CPU::new();

        cpu.adc_imm(0b_0111_1111_u8);

        assert_eq!(false, cpu.n);

        cpu.adc_imm(0b_0000_0001_u8);

        assert_eq!(true, cpu.n);
    }

    #[test]
    fn test_adc_imm_with_overflow_flag() {
        let mut cpu = CPU::new();

        cpu.a = 0x7f; //+ve
        cpu.adc_imm(0x1_u8); //+ve

        assert_eq!(true, cpu.n);
        assert_eq!(true, cpu.v);

        cpu.a = 0x80; //-ve
        cpu.adc_imm(0x80_u8); //-ve

        assert_eq!(false, cpu.n);
        assert_eq!(true, cpu.v);

        cpu.a = 0x1; //+ve
        cpu.adc_imm(0xf0_u8); //-ve

        assert_eq!(true, cpu.n);
        assert_eq!(false, cpu.v);

        cpu.a = 0xff; //-ve
        cpu.adc_imm(0x2_u8); //+ve

        assert_eq!(false, cpu.n);
        assert_eq!(false, cpu.v);
    }

    #[test]
    fn test_adc_imm_get_cycles() {
        let mut cpu = CPU::new();
        let cycles: u8 = cpu.adc_imm(0b_0000_0000_u8);
        assert_eq!(2, cycles);
    }
}
