use super::super::super::CPU;

impl CPU {
    pub(super) fn and_abs(&mut self, addr: u16, mem: &[u8]) -> u8 {
        2 + self.and_imm(mem[addr as usize])
    }
}

#[cfg(test)]
mod and_abs_tests {
    use super::*;

    #[test]
    fn test_and_abs_correct_cycles() {
        let mut cpu = CPU::new();
        let mem = [0; 0x10000];

        assert_eq!(4, cpu.and_abs(0x0, &mem));
    }

    #[test]
    fn test_and_abs() {
        let mut cpu = CPU::new();
        let mut mem = [0; 0x10000];

        cpu.a = 0b1010_1010;
        mem[0x1111] = 0b0101_0101;
        cpu.and_abs(0x1111, &mem);

        assert_eq!(0x0, cpu.a)
    }

    #[test]
    fn test_and_abs_zero_flag() {
        let mut cpu = CPU::new();
        let mut mem = [0; 0x10000];

        cpu.a = 0xff_u8;
        mem[0x1111] = 0xff_u8;
        cpu.and_abs(0x1111, &mem);

        assert_eq!(false, cpu.z);

        mem[0x1111] = 0x00_u8;
        cpu.and_abs(0x1111, &mem);

        assert_eq!(true, cpu.z);
    }

    #[test]
    fn test_and_abs_negative_flag() {
        let mut cpu = CPU::new();
        let mut mem = [0; 0x10000];

        cpu.a = 0xff_u8;
        mem[0x1111] = 0xff_u8;
        cpu.and_abs(0x1111, &mem);
        assert_eq!(true, cpu.n);

        mem[0x1111] = 0x00_u8;
        cpu.and_abs(0x1111, &mem);
        assert_eq!(false, cpu.n);
    }
}