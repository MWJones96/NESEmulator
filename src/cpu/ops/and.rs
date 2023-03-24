use super::super::CPU;

impl CPU {
    pub fn and_imm(&mut self, imm: u8) -> u8 {
        self.a &= imm;

        self.z = self.a == 0;
        self.n = (self.a & 0b1000_0000) > 0;
        
        2
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