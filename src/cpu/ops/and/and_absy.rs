use super::super::super::CPU;

impl CPU {
    pub(super) fn and_absy(&mut self, addr: u16, mem: &[u8]) -> u8 {
        super::_and_abs_helper(self, addr, mem, self.y)
    }
}

#[cfg(test)]
mod and_absy_tests {
    use super::*;

    #[test]
    fn test_and_absy_correct_cycles_no_page_cross() {
        let mut cpu = CPU::new();
        let mem = [0; 0x10000];

        assert_eq!(4, cpu.and_absy(0x0, &mem));
    }

    #[test]
    fn test_and_absy_correct_cycles_with_page_cross() {
        let mut cpu = CPU::new();
        let mem = [0; 0x10000];

        cpu.y = 0xff;
        assert_eq!(5, cpu.and_absy(0x1111, &mem));
    }

    #[test]
    fn test_and_absy() {
        let mut cpu = CPU::new();
        let mut mem = [0; 0x10000];

        cpu.a = 0xff;
        cpu.y = 0x1;
        mem[0x0] = 0xee;
        cpu.and_absy(0xffff, &mem);

        assert_eq!(0xee, cpu.a);
    }

    #[test]
    fn test_and_absy_zero_flag() {
        let mut cpu = CPU::new();
        let mut mem = [0; 0x10000];

        cpu.a = 0x11;
        mem[0x0] = 0xef;

        cpu.and_absy(0x0000, &mem);

        assert_eq!(false, cpu.z);

        cpu.a = 0x11;
        mem[0x0] = 0xee;

        cpu.and_absy(0x0000, &mem);

        assert_eq!(true, cpu.z);
    }

    #[test]
    fn test_and_absy_negative_flag() {
        let mut cpu = CPU::new();
        let mut mem = [0; 0x10000];

        cpu.a = 0xff;
        mem[0x0] = 0x80;

        cpu.and_absy(0x0000, &mem);

        assert_eq!(true, cpu.n);

        cpu.a = 0xff;
        mem[0x0] = 0x00;

        cpu.and_absy(0x0000, &mem);

        assert_eq!(false, cpu.n);
    }
}
