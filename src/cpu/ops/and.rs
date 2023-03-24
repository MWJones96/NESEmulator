use super::super::CPU;

impl CPU {
    pub fn and_imm(&mut self, imm: u8) -> u8 {
        self.a &= imm;

        self.z = self.a == 0;
        self.n = (self.a & 0x80) > 0;
        
        2
    }

    pub fn and_zp(&mut self, addr: u8, mem: &[u8]) -> u8 {
        self.a &= mem[addr as usize];

        self.z = self.a == 0;
        self.n = (self.a & 0x80) > 0;
        
        3
    }
}

#[cfg(test)]
mod and_imm_tests {
    use super::*;

    #[test]
    fn test_and_imm_correct_cycles() {
        let mut cpu = CPU::new();
        assert_eq!(2, cpu.and_imm(0xff));
    }

    #[test]
    fn test_and_imm() {
        let mut cpu = CPU::new();
        cpu.a = 0b1010_1010_u8;

        cpu.and_imm(0b0101_0101_u8);

        assert_eq!(0x0, cpu.a);
    }

    #[test]
    fn test_and_imm_all_ones() {
        let mut cpu = CPU::new();
        cpu.a = 0xff;

        cpu.and_imm(0xff);

        assert_eq!(0xff, cpu.a);
    }

    #[test]
    fn test_and_imm_half_ones() {
        let mut cpu = CPU::new();
        cpu.a = 0b0000_1111_u8;

        cpu.and_imm(0b0000_1111_u8);

        assert_eq!(0xf, cpu.a);
    }

    #[test]
    fn test_and_imm_zero_flag() {
        let mut cpu = CPU::new();
        cpu.a = 0b0000_1111_u8;

        cpu.and_imm(0b0000_1111_u8);

        assert_eq!(false, cpu.z);

        cpu.and_imm(0b0000_0000_u8);

        assert_eq!(true, cpu.z);
    }

    #[test]
    fn test_and_imm_negative_flag() {
        let mut cpu = CPU::new();
        cpu.a = 0xff;

        cpu.and_imm(0xff);

        assert_eq!(true, cpu.n)
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
