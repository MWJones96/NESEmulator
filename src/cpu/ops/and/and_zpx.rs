use super::super::super::CPU;

impl CPU {
    pub(super) fn and_zpx(&mut self, addr: u8, mem: &[u8]) -> u8 {
        let resolved_addr = addr.wrapping_add(self.x);

        1 + self.and_zp(resolved_addr, mem)
    }
}

#[cfg(test)]
mod and_zpx_tests {
    use super::*;

    #[test]
    fn test_and_zpx_correct_cycles() {
        let mut cpu = CPU::new();

        assert_eq!(4, cpu.and_zpx(0x0, &[0x00u8]));
    }

    #[test]
    fn test_and_zpx() {
        let mut cpu = CPU::new();
        cpu.x = 0x1;
        cpu.a = 0b1010_1010;

        cpu.and_zpx(0xff, &[0b0101_0101u8]);
        assert_eq!(0x0, cpu.a);
    }

    #[test]
    fn test_and_zpx_negative_flag() {
        let mut cpu = CPU::new();
        cpu.x = 0x1;
        cpu.a = 0xff;

        cpu.and_zpx(0xff, &[0xff]);
        assert_eq!(true, cpu.n);
        cpu.and_zpx(0xff, &[0x00]);
        assert_eq!(false, cpu.n);
    }

    #[test]
    fn test_and_zpx_zero_flag() {
        let mut cpu = CPU::new();
        cpu.x = 0x1;
        cpu.a = 0xff;

        cpu.and_zpx(0xff, &[0xff]);
        assert_eq!(false, cpu.z);
        cpu.and_zpx(0xff, &[0x00]);
        assert_eq!(true, cpu.z);
    }
}