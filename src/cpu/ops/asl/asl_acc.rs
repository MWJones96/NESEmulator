use super::super::super::CPU;

impl CPU {
    pub(super) fn asl_acc(&mut self) -> u8 {
        let calc: u16 = (self.a as u16) << 1;
        self.a = calc as u8;

        self.c = calc > (u8::MAX as u16);
        self.z = self.a == 0;
        self.n = (self.a & 0x80) > 0;

        2
    }
}

#[cfg(test)]
mod asl_acc_tests {
    use super::*;

    #[test]
    fn test_asl_acc_correct_num_of_cycles() {
        let mut cpu = CPU::new();

        assert_eq!(2, cpu.asl_acc());
    }

    #[test]
    fn test_asl_acc_shift() {
        let mut cpu = CPU::new();

        cpu.a = 0x1;
        cpu.asl_acc();
        assert_eq!(0x2, cpu.a);

        cpu.a = 0xff;
        cpu.asl_acc();
        assert_eq!(0xfe, cpu.a);

        cpu.a = 0x0;
        cpu.asl_acc();
        assert_eq!(0x0, cpu.a);

        cpu.a = 0x0;
        cpu.c = true;
        cpu.asl_acc();
        assert_eq!(0x0, cpu.a);
    }

    #[test]
    fn test_asl_acc_carry_flag() {
        let mut cpu = CPU::new();

        cpu.a = 0xC0;
        cpu.asl_acc();
        assert_eq!(true, cpu.c);

        cpu.asl_acc();
        assert_eq!(true, cpu.c);

        cpu.asl_acc();
        assert_eq!(false, cpu.c);
    }

    #[test]
    fn test_asl_acc_zero_flag() {
        let mut cpu = CPU::new();

        cpu.a = 0x80;
        cpu.asl_acc();
        assert_eq!(true, cpu.z);

        cpu.a = 0x40;
        cpu.asl_acc();
        assert_eq!(false, cpu.z);

        cpu.asl_acc();
        assert_eq!(true, cpu.z);
    }

    #[test]
    fn test_asl_acc_negative_flag() {
        let mut cpu = CPU::new();

        cpu.a = 0x40;
        cpu.asl_acc();
        assert_eq!(true, cpu.n);

        cpu.asl_acc();
        assert_eq!(false, cpu.n);
    }
}
