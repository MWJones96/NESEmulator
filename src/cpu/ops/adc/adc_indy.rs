use super::super::super::CPU;

impl CPU {
    pub(super) fn adc_indy(&mut self, addr: u8, mem: &[u8]) -> u8 {
        let low_byte_addr: u8 = mem[addr as usize];
        let high_byte_addr: u8 = mem[addr.wrapping_add(1) as usize];

        let page_before: u8 = high_byte_addr;

        let resolved_addr: u16 = ((high_byte_addr as u16) << 8) | low_byte_addr as u16;
        let resolved_addr: u16 = resolved_addr.wrapping_add(self.y as u16);

        let page_after: u8 = (resolved_addr >> 8) as u8;

        if page_before == page_after { 
            1 + self.adc_abs(resolved_addr, mem) 
        } else { 
            2 + self.adc_abs(resolved_addr, mem) 
        }
    }
}

#[cfg(test)]
mod adc_indy_tests {
    use super::*;

    #[test]
    fn test_adc_indy_correct_cycles_no_page_cross() {
        let mut cpu = CPU::new();
        let mem = [0; 0x10000];

        assert_eq!(5, cpu.adc_indy(0x00, &mem));
    }

    #[test]
    fn test_adc_indy_correct_cycles_page_cross() {
        let mut cpu = CPU::new();
        let mut mem = [0; 0x10000];
        mem[0x80] = 0x11;
        mem[0x81] = 0x15;

        cpu.y = 0xff;

        assert_eq!(6, cpu.adc_indy(0x80, &mem));
    }

    #[test]
    fn test_adc_indy_end_of_zero_page() {
        let mut cpu = CPU::new();
        let mut mem = [0; 0x10000];

        cpu.y = 0x2;

        mem[0x0] = 0xff;
        mem[0xff] = 0xff;

        mem[0x1] = 0xee;

        cpu.adc_indy(0xff, &mem);

        assert_eq!(0xee, cpu.a);
    }

    #[test]
    fn test_adc_indy_carry_flag() {
        let mut cpu = CPU::new();
        let mut mem = [0; 0x10000];

        mem[0x0] = 0x10;
        mem[0x1] = 0x10;

        mem[0x1010] = 0x80;

        cpu.adc_indy(0x00, &mem);
        assert_eq!(false, cpu.c);

        cpu.adc_indy(0x00, &mem);
        assert_eq!(true, cpu.c);

        cpu.adc_indy(0x00, &mem);
        assert_eq!(false, cpu.c);
    }

    #[test]
    fn test_adc_indy_zero_flag() {
        let mut cpu = CPU::new();
        let mut mem = [0; 0x10000];

        mem[0x0] = 0x10;
        mem[0x1] = 0x10;

        mem[0x1010] = 0x80;

        cpu.adc_indy(0x00, &mem);
        assert_eq!(false, cpu.z);

        cpu.adc_indy(0x00, &mem);
        assert_eq!(true, cpu.z);

        cpu.adc_indy(0x00, &mem);
        assert_eq!(false, cpu.z);
    }

    #[test]
    fn test_adc_indy_negative_flag() {
        let mut cpu = CPU::new();
        let mut mem = [0; 0x10000];

        mem[0x0] = 0x10;
        mem[0x1] = 0x10;

        mem[0x1010] = 0x80;

        cpu.adc_indy(0x00, &mem);
        assert_eq!(true, cpu.n);

        cpu.adc_indy(0x00, &mem);
        assert_eq!(false, cpu.n);

        cpu.adc_indy(0x00, &mem);
        assert_eq!(true, cpu.n);
    }

    #[test]
    fn test_adc_indy_overflow_flag() {
        let mut cpu = CPU::new();
        let mut mem = [0; 0x10000];

        mem[0x0] = 0x10;
        mem[0x1] = 0x10;

        cpu.a = 0x7f; //+ve
        mem[0x1010] = 0x7f; //+ve
        cpu.adc_indy(0x0, &mem);

        assert_eq!(true, cpu.n);
        assert_eq!(true, cpu.v);

        cpu.a = 0x81; //-ve
        mem[0x1010] = 0x81; //-ve
        cpu.adc_indy(0x0, &mem);

        assert_eq!(false, cpu.n);
        assert_eq!(true, cpu.v);

        cpu.a = 0x2; //+ve
        mem[0x1010] = 0xff; //-ve
        cpu.adc_indy(0x0, &mem);

        assert_eq!(false, cpu.n);
        assert_eq!(false, cpu.v);

        cpu.a = 0xf0; //-ve
        mem[0x1010] = 0x1; //+ve
        cpu.adc_indy(0x0, &mem);

        assert_eq!(true, cpu.n);
        assert_eq!(false, cpu.v);
    }
}
