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

use crate::cpu::bus::Bus;

use super::super::CPU;

impl CPU {
    pub(in crate::cpu) fn asl_acc(&mut self) -> u8 {
        let data = self._asl(self.a);
        self.a = data;
        
        2
    }

    pub (in crate::cpu) fn asl_zp(&mut self, addr: u8, bus: &dyn Bus) -> u8 {
        let (_, data) = self.zp(addr, bus);
        let data = self._asl(data);
        bus.write(addr as u16, data);
        
        5
    }

    fn _asl(&mut self, data: u8) -> u8 {
        let calc: u16 = (data as u16) << 1;

        self.c = calc > (u8::MAX as u16);
        self.z = (calc as u8) == 0;
        self.n = ((calc as u8) & 0x80) > 0;

        return calc as u8;
    }

}

#[cfg(test)]
mod asl_acc_tests {
    use mockall::predicate::eq;

    use crate::cpu::bus::MockBus;

    use super::*;

    #[test]
    fn test_asl_acc_correct_num_of_cycles() {
        let mut cpu = CPU::new();

        cpu.a = 0x20;
        assert_eq!(2, cpu.asl_acc());
        assert_eq!(0x40, cpu.a);
    }

    #[test]
    fn test_asl_zp_correct_number_of_cycles() {
        let mut cpu = CPU::new();
        let mut bus = MockBus::new();

        bus.expect_read()
            .with(eq(0x0))
            .times(1)
            .return_const(0xff);

        bus.expect_write()
            .with(eq(0x0), eq(0xfe))
            .times(1)
            .return_const(());

        assert_eq!(5, cpu.asl_zp(0x0, &bus));
    }

    #[test]
    fn test_asl_shift() {
        let mut cpu = CPU::new();

        assert_eq!(0x2, cpu._asl(0x1));
        assert_eq!(0xfe, cpu._asl(0xff));
        assert_eq!(0x0, cpu._asl(0x0));
        cpu.c = true;
        assert_eq!(0x0, cpu._asl(0x0));
    }

    #[test]
    fn test_asl_carry_flag() {
        let mut cpu = CPU::new();

        cpu._asl(0xC0);
        assert_eq!(true, cpu.c);

        cpu._asl(0x1);
        assert_eq!(false, cpu.c);
    }

    #[test]
    fn test_asl_acc_zero_flag() {
        let mut cpu = CPU::new();

        cpu._asl(0x80);
        assert_eq!(true, cpu.z);

        cpu._asl(0x40);
        assert_eq!(false, cpu.z);
    }

    #[test]
    fn test_asl_acc_negative_flag() {
        let mut cpu = CPU::new();

        cpu._asl(0x40);
        assert_eq!(true, cpu.n);

        cpu._asl(0x80);
        assert_eq!(false, cpu.n);
    }
}
