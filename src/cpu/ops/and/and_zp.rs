use super::super::super::CPU;

impl CPU {
    pub(super) fn and_zp(&mut self, addr: u8, mem: &[u8]) -> u8 {
        1 + self.and_imm(mem[addr as usize])
    }
}

#[cfg(test)]
mod and_zp_tests {
    use super::*;

    #[test]
    fn test_and_zp_correct_cycles() {
        let mut cpu = CPU::new();

        assert_eq!(3, cpu.and_zp(0x0, &[0x00u8]));
    }

    #[test]
    fn test_and_zp_correct_output() {
        let mut cpu = CPU::new();

        cpu.a = 0xff;
        cpu.and_zp(0x0, &[0x7u8]);

        assert_eq!(0x7, cpu.a);
    }

    #[test]
    fn test_and_zp_zero_flag() {
        let mut cpu = CPU::new();

        cpu.a = 0xff;
        cpu.and_zp(0x0, &[0xff]);

        assert_eq!(false, cpu.z);

        cpu.and_zp(0x0, &[0x0]);

        assert_eq!(true, cpu.z);
    }

    #[test]
    fn test_and_zp_negative_flag() {
        let mut cpu = CPU::new();

        cpu.a = 0xff;
        cpu.and_zp(0x0, &[0xff]);

        assert_eq!(true, cpu.n);

        cpu.and_zp(0x0, &[0x0]);

        assert_eq!(false, cpu.n);
    }
}