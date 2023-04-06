use super::super::super::CPU;

impl CPU {
    pub(super) fn adc_absy(&mut self, addr: u16, mem: &[u8]) -> u8 {
        super::_adc_abs_helper(self, addr, mem, self.y)
    }
}

#[cfg(test)]
mod adc_absy_tests {
    use super::*;

    #[test]
    fn test_adc_absy_no_page_boundary_crossed() {
        let mut cpu = CPU::new();
        let mem = [0; 0x10000];
        cpu.y = 0xff_u8;

        assert_eq!(4, cpu.adc_absy(0x0000_u16, &mem));
    }

    #[test]
    fn test_adc_absy_page_boundary_crossed() {
        let mut cpu = CPU::new();
        let mem = [0; 0x10000];
        cpu.y = 0xff_u8;

        assert_eq!(5, cpu.adc_absy(0x0001_u16, &mem));
    }

    #[test]
    fn test_adc_absy_page_boundary_crossed_at_end_of_memory() {
        let mut cpu = CPU::new();
        let mem = [0; 0x10000];
        cpu.y = 0xff_u8;

        assert_eq!(5, cpu.adc_absy(0xffff_u16, &mem));
    }

    #[test]
    fn test_adc_absy_carry_flag() {
        let mut cpu = CPU::new();
        let mut mem = [0; 0x10000];
        cpu.y = 0x01_u8;
        mem[0x1] = 0x80;

        cpu.adc_absy(0x0000_u16, &mem);

        assert_eq!(false, cpu.c);

        cpu.adc_absy(0x0000_u16, &mem);

        assert_eq!(true, cpu.c);

        cpu.adc_absy(0x0000_u16, &mem);

        assert_eq!(false, cpu.c);
    }

    #[test]
    fn test_adc_absy_zero_flag() {
        let mut cpu = CPU::new();
        let mut mem = [0; 0x10000];
        cpu.y = 0x01_u8;
        mem[0x1] = 0x80;

        cpu.adc_absy(0x0000_u16, &mem);

        assert_eq!(false, cpu.z);

        cpu.adc_absy(0x0000_u16, &mem);

        assert_eq!(true, cpu.z);

        cpu.adc_absy(0x0000_u16, &mem);

        assert_eq!(false, cpu.z);
    }

    #[test]
    fn test_adc_absy_negative_flag() {
        let mut cpu = CPU::new();
        let mut mem = [0; 0x10000];
        cpu.y = 0x01_u8;
        mem[0x1] = 0x80;

        cpu.adc_absy(0x0000_u16, &mem);

        assert_eq!(true, cpu.n);

        cpu.adc_absy(0x0000_u16, &mem);

        assert_eq!(false, cpu.n);

        cpu.adc_absy(0x0000_u16, &mem);

        assert_eq!(true, cpu.n);
    }

    #[test]
    fn test_adc_absy_overflow_flag() {
        let mut cpu = CPU::new();
        let mut mem = [0; 0x10000];

        cpu.a = 0x7f; //+ve
        mem[0x0] = 0x7f; //+ve
        cpu.adc_absy(0x0, &mem);

        assert_eq!(true, cpu.n);
        assert_eq!(true, cpu.v);

        cpu.a = 0x81; //-ve
        mem[0x0] = 0x81; //-ve
        cpu.adc_absy(0x0, &mem);

        assert_eq!(false, cpu.n);
        assert_eq!(true, cpu.v);

        cpu.a = 0x1; //+ve
        mem[0x0] = 0xf0; //-ve
        cpu.adc_absy(0x0, &mem);

        assert_eq!(true, cpu.n);
        assert_eq!(false, cpu.v);

        cpu.a = 0xff; //-ve
        mem[0x0] = 0x2; //+ve
        cpu.adc_absy(0x0, &mem);

        assert_eq!(false, cpu.n);
        assert_eq!(false, cpu.v);
    }
}
