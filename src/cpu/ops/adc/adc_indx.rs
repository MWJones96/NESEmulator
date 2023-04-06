use super::super::super::CPU;

impl CPU {
    pub(super) fn adc_indx(&mut self, addr: u8, mem: &[u8]) -> u8 {
        let low_byte_addr: u8 = addr.wrapping_add(self.x);
        let high_byte_addr: u8 = low_byte_addr.wrapping_add(1);

        let resolved_addr: u16 = ((mem[high_byte_addr as usize] as u16) << 8) 
            | (mem[low_byte_addr as usize] as u16);

        2 + self.adc_abs(resolved_addr, mem)
    }
}

#[cfg(test)]
mod adc_indx_tests {
    use super::*;

    #[test]
    fn test_adc_indx_correct_cycles() {
        let mut cpu = CPU::new();
        let mem = [0; 0x10000];

        assert_eq!(6, cpu.adc_indx(0x00, &mem));
    }

    #[test]
    fn test_adc_indx() {
        let mut cpu = CPU::new();
        let mut mem = [0; 0x10000];

        //Lower byte
        mem[0x00] = 0x40;
        //Upper byte
        mem[0x01] = 0x20;

        mem[0x2040] = 0x77;

        cpu.adc_indx(0x00, &mem);
        assert_eq!(0x77, cpu.a);
    }

    #[test]
    fn test_adc_indx_wrap() {
        let mut cpu = CPU::new();
        let mut mem = [0; 0x10000];

        mem[0xff] = 0x77;
        mem[0x00] = 0x88;

        mem[0x8877] = 0x11;

        cpu.adc_indx(0xff, &mem);
        assert_eq!(0x11, cpu.a);
    }

    #[test]
    fn test_adc_indx_wrap_with_x() {
        let mut cpu = CPU::new();
        let mut mem = [0; 0x10000];
        cpu.x = 0x1;

        mem[0x0] = 0x77;
        mem[0x1] = 0x88;

        mem[0x8877] = 0x22;

        cpu.adc_indx(0xff, &mem);
        assert_eq!(0x22, cpu.a);
    }

    #[test]
    fn test_adc_indx_wrap_with_x_twice() {
        let mut cpu = CPU::new();
        let mut mem = [0; 0x10000];
        cpu.x = 0x1;

        mem[0x0] = 0x77;
        mem[0x1] = 0x88;

        mem[0x8877] = 0x22;

        cpu.adc_indx(0xff, &mem);
        cpu.adc_indx(0xff, &mem);

        assert_eq!(0x44, cpu.a);
    }

    #[test]
    fn test_adc_indx_carry_flag() {
        let mut cpu = CPU::new();
        let mut mem = [0; 0x10000];
        cpu.x = 0x1;

        mem[0x0] = 0x77;
        mem[0x1] = 0x88;

        mem[0x8877] = 0x80;

        cpu.adc_indx(0xff, &mem);
        assert_eq!(false, cpu.c);

        cpu.adc_indx(0xff, &mem);

        assert_eq!(true, cpu.c);

        cpu.adc_indx(0xff, &mem);

        assert_eq!(false, cpu.c);
    }

    #[test]
    fn test_adc_indx_zero_flag() {
        let mut cpu = CPU::new();
        let mut mem = [0; 0x10000];
        cpu.x = 0x1;

        mem[0x0] = 0x77;
        mem[0x1] = 0x88;

        mem[0x8877] = 0x80;

        cpu.adc_indx(0xff, &mem);
        assert_eq!(false, cpu.z);

        cpu.adc_indx(0xff, &mem);

        assert_eq!(true, cpu.z);

        cpu.adc_indx(0xff, &mem);

        assert_eq!(false, cpu.z);
    }

    #[test]
    fn test_adc_indx_negative_flag() {
        let mut cpu = CPU::new();
        let mut mem = [0; 0x10000];
        cpu.x = 0x1;

        mem[0x0] = 0x77;
        mem[0x1] = 0x88;

        mem[0x8877] = 0x80;

        cpu.adc_indx(0xff, &mem);
        assert_eq!(true, cpu.n);

        cpu.adc_indx(0xff, &mem);

        assert_eq!(false, cpu.n);

        cpu.adc_indx(0xff, &mem);

        assert_eq!(true, cpu.n);
    }

    #[test]
    fn test_adc_indx_overflow_flag() {
        let mut cpu = CPU::new();
        let mut mem = [0; 0x10000];

        mem[0x0] = 0x77;
        mem[0x1] = 0x88;

        cpu.a = 0x7f; //+ve
        mem[0x8877] = 0x7f; //+ve

        cpu.adc_indx(0x00, &mem);

        assert_eq!(true, cpu.n);
        assert_eq!(true, cpu.v);

        cpu.a = 0x81; //-ve
        mem[0x8877] = 0x81; //-ve
        cpu.adc_indx(0x00, &mem);

        assert_eq!(false, cpu.n);
        assert_eq!(true, cpu.v);

        cpu.a = 0x1; //+ve
        mem[0x8877] = 0xf0; //-ve
        cpu.adc_indx(0x00, &mem);

        assert_eq!(true, cpu.n);
        assert_eq!(false, cpu.v);

        cpu.a = 0xff; //-ve
        mem[0x8877] = 0x2; //+ve
        cpu.adc_indx(0x00, &mem);

        assert_eq!(false, cpu.n);
        assert_eq!(false, cpu.v);
    }
}
