use super::super::super::CPU;

impl CPU {
    pub(super) fn and_absx(&mut self, addr: u16, mem: &[u8]) -> u8 {
        super::_and_abs_helper(self, addr, mem, self.x)
    }
}

#[cfg(test)]
mod and_absx_tests {
    use super::*;

    #[test]
    fn test_and_absx_correct_cycles_no_page_cross() {
        let mut cpu = CPU::new();
        let mem = [0; 0x10000];

        assert_eq!(4, cpu.and_absx(0x0, &mem));
    }

    #[test]
    fn test_and_absx_correct_cycles_with_page_cross() {
        let mut cpu = CPU::new();
        let mem = [0; 0x10000];
        cpu.x = 0xff;

        assert_eq!(5, cpu.and_absx(0x1111, &mem));
    }

    #[test]
    fn test_and_absx() {
        let mut cpu = CPU::new();
        let mut mem = [0; 0x10000];

        cpu.a = 0x11;
        mem[0x0] = 0xef;

        cpu.x = 0x1;
        cpu.and_absx(0xffff, &mem);

        assert_eq!(0x1, cpu.a);
    }

    #[test]
    fn test_and_absx_zero_flag() {
        let mut cpu = CPU::new();
        let mut mem = [0; 0x10000];

        cpu.a = 0x11;
        mem[0x0] = 0xef;

        cpu.and_absx(0x0000, &mem);

        assert_eq!(false, cpu.z);

        cpu.a = 0x11;
        mem[0x0] = 0xee;

        cpu.and_absx(0x0000, &mem);

        assert_eq!(true, cpu.z);
    }

    #[test]
    fn test_and_absx_negative_flag() {
        let mut cpu = CPU::new();
        let mut mem = [0; 0x10000];

        cpu.a = 0xff;
        mem[0x0] = 0x80;

        cpu.and_absx(0x0000, &mem);

        assert_eq!(true, cpu.n);

        cpu.a = 0xff;
        mem[0x0] = 0x00;

        cpu.and_absx(0x0000, &mem);

        assert_eq!(false, cpu.n);
    }
}