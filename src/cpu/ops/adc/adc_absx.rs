use super::super::super::CPU;

impl CPU {
    pub(super) fn adc_absx(&mut self, addr: u16, mem: &[u8]) -> u8 {
        super::_adc_abs_helper(self, addr, mem, self.x)
    }
}

#[cfg(test)]
mod adc_absx_tests {
    use super::*;

    #[test]
    fn test_adc_absx_no_page_boundary_crossed() {
        let mut cpu = CPU::new();
        let mem = [0; 0x10000];
        cpu.x = 0xff_u8;

        assert_eq!(4, cpu.adc_absx(0x0000_u16, &mem));
    }

    #[test]
    fn test_adc_absx_page_boundary_crossed() {
        let mut cpu = CPU::new();
        let mem = [0; 0x10000];
        cpu.x = 0xff_u8;

        assert_eq!(5, cpu.adc_absx(0x0001_u16, &mem));
    }

    #[test]
    fn test_adc_absx_page_boundary_crossed_at_end_of_memory() {
        let mut cpu = CPU::new();
        let mem = [0; 0x10000];
        cpu.x = 0xff_u8;

        assert_eq!(5, cpu.adc_absx(0xffff_u16, &mem));
    }

    #[test]
    fn test_adc_absx_carry_flag() {
        let mut cpu = CPU::new();
        let mut mem = [0; 0x10000];
        cpu.x = 0x01_u8;
        mem[0x1] = 0x80;

        cpu.adc_absx(0x0000_u16, &mem);

        assert_eq!(false, cpu.c);

        cpu.adc_absx(0x0000_u16, &mem);

        assert_eq!(true, cpu.c);

        cpu.adc_absx(0x0000_u16, &mem);

        assert_eq!(false, cpu.c);
    }

    #[test]
    fn test_adc_absx_zero_flag() {
        let mut cpu = CPU::new();
        let mut mem = [0; 0x10000];
        cpu.x = 0x01_u8;
        mem[0x1] = 0x80;

        cpu.adc_absx(0x0000_u16, &mem);

        assert_eq!(false, cpu.z);

        cpu.adc_absx(0x0000_u16, &mem);

        assert_eq!(true, cpu.z);

        cpu.adc_absx(0x0000_u16, &mem);

        assert_eq!(false, cpu.z);
    }

    #[test]
    fn test_adc_absx_negative_flag() {
        let mut cpu = CPU::new();
        let mut mem = [0; 0x10000];
        cpu.x = 0x01_u8;
        mem[0x1] = 0x80;

        cpu.adc_absx(0x0000_u16, &mem);

        assert_eq!(true, cpu.n);

        cpu.adc_absx(0x0000_u16, &mem);

        assert_eq!(false, cpu.n);

        cpu.adc_absx(0x0000_u16, &mem);

        assert_eq!(true, cpu.n);
    }

    #[test]
    fn test_adc_absx_overflow_flag() {
        let mut cpu = CPU::new();
        let mut mem = [0; 0x10000];

        cpu.a = 0x81; //-ve
        mem[0x0] = 0x81; //-ve

        cpu.adc_absx(0x0, &mem);

        assert_eq!(false, cpu.n);
        assert_eq!(true, cpu.v);

        cpu.a = 0x7f; //+ve
        mem[0x0] = 0x7f; //+ve

        cpu.adc_absx(0x0, &mem);

        assert_eq!(true, cpu.n);
        assert_eq!(true, cpu.v);

        cpu.a = 0x1; //+ve
        mem[0x0] = 0x80; //-ve

        cpu.adc_absx(0x0, &mem);

        assert_eq!(true, cpu.n);
        assert_eq!(false, cpu.v);

        cpu.a = 0xf0; //-ve
        mem[0x0] = 0x20; //+ve

        cpu.adc_absx(0x0, &mem);

        assert_eq!(false, cpu.n);
        assert_eq!(false, cpu.v);
    }
}
