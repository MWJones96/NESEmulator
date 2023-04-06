use super::super::super::CPU;

impl CPU {
    pub(super) fn adc_abs(&mut self, addr: u16, mem: &[u8]) -> u8 {
        2 + self.adc_imm(mem[addr as usize])
    }
}

#[cfg(test)]
mod adc_abs_tests {
    use super::*;

    #[test]
    fn test_adc_abs_correct_cycles() {
        let mut cpu = CPU::new();

        assert_eq!(4, cpu.adc_abs(0x0000_u16, &[0x00_u8]));
    }

    #[test]
    fn test_adc_abs_fetch_mem_addr() {
        let mut cpu = CPU::new();
        let mut mem = [0; 0x10000];
        mem[0xffff] = 0x77_u8;

        cpu.adc_abs(0xffff_u16, &mem);

        assert_eq!(0x77_u8, cpu.a);
    }

    #[test]
    fn test_adc_abs_carry_flag() {
        let mut cpu = CPU::new();
        let mut mem = [0; 0x10000];
        mem[0x0] = 0x80_u8;

        cpu.adc_abs(0x0, &mem);

        assert_eq!(0x80_u8, cpu.a);
        assert_eq!(false, cpu.c);

        cpu.adc_abs(0x0, &mem);

        assert_eq!(0x00_u8, cpu.a);
        assert_eq!(true, cpu.c);

        cpu.adc_abs(0x0, &mem);

        assert_eq!(0x81_u8, cpu.a);
        assert_eq!(false, cpu.c);
    }

    #[test]
    fn test_adc_abs_zero_flag() {
        let mut cpu = CPU::new();
        let mut mem = [0; 0x10000];
        mem[0x0] = 0x80_u8;

        cpu.adc_abs(0x0, &mem);

        assert_eq!(false, cpu.z);

        cpu.adc_abs(0x0, &mem);

        assert_eq!(true, cpu.z);

        cpu.adc_abs(0x0, &mem);

        assert_eq!(false, cpu.z);
    }

    #[test]
    fn test_adc_abs_negative_flag() {
        let mut cpu = CPU::new();
        let mut mem = [0; 0x10000];
        mem[0x0] = 0x80_u8;

        cpu.adc_abs(0x0, &mem);

        assert_eq!(true, cpu.n);

        cpu.adc_abs(0x0, &mem);

        assert_eq!(false, cpu.n);
    }

    #[test]
    fn test_adc_abs_overflow_flag() {
        let mut cpu = CPU::new();
        let mut mem = [0; 0x10000];
        cpu.a = 0x7f; //+ve
        mem[0x0] = 0x1; //+ve

        cpu.adc_abs(0x0, &mem);

        assert_eq!(true, cpu.n);
        assert_eq!(true, cpu.v);

        cpu.a = 0x80; //-ve
        mem[0x0] = 0xff; //-ve

        cpu.adc_abs(0x0, &mem);

        assert_eq!(false, cpu.n);
        assert_eq!(true, cpu.v);

        cpu.a = 0x1; //+ve
        mem[0x0] = 0xf0; //-ve

        cpu.adc_abs(0x0, &mem);

        assert_eq!(true, cpu.n);
        assert_eq!(false, cpu.v);

        cpu.a = 0xff; //-ve
        mem[0x0] = 0x2; //+ve

        cpu.adc_abs(0x0, &mem);

        assert_eq!(false, cpu.n);
        assert_eq!(false, cpu.v);
    }
}
