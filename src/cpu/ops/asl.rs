/*
    ASL - Arithmetic Shift Left

    Operation: C ← /M7...M0/ ← 0

    The shift left instruction shifts either the accumulator or the address 
    memory location 1 bit to the left, with the bit 0 always being set to 0 
    and the the input bit 7 being stored in the carry flag. ASL either 
    shifts the accumulator left 1 bit or is a read/modify/write instruction 
    that affects only memory.

    The instruction does not affect the overflow bit, sets N equal to the 
    result bit 7 (bit 6 in the input), sets Z flag if the result is equal 
    to 0, otherwise resets Z and stores the input bit 7 in the carry flag.
*/

use super::super::CPU;

impl CPU {
    pub fn asl_acc(&mut self) -> u8 {
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
