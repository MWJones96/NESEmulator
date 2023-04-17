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

use crate::cpu::{addr::{AddrModeResult, AddrMode}, bus::Bus};

use super::super::CPU;

impl CPU {
    pub(in crate::cpu) fn asl(&mut self, mode: &AddrModeResult, bus: &dyn Bus, addr: u16) -> u8 {
        let data = self._asl(mode.data);
        match mode.mode {
            AddrMode::ACC => { self.a = data; 2 },
            AddrMode::ZP => { bus.write(addr, data); 5 }
            AddrMode::ZPX => { bus.write(addr, data); 6 }
            _ => panic!("Unimplemented")
        }
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
mod asl_tests {
    use mockall::predicate::eq;

    use crate::cpu::bus::MockBus;

    use super::*;

    #[test]
    fn test_asl_acc() {
        let mut cpu = CPU::new();
        let mut bus = MockBus::new();

        cpu.a = 0x20;
        assert_eq!(2, cpu.asl(&cpu.acc(), &bus, 0x0));
        assert_eq!(0x40, cpu.a);

        assert_eq!(false, cpu.c);
        assert_eq!(false, cpu.z);
        assert_eq!(false, cpu.n);
    }

    #[test]
    fn test_asl_zp() {
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

        assert_eq!(5, cpu.asl(&cpu.zp(0x0, &bus), &bus, 0x0));
        
        assert_eq!(true, cpu.c);
        assert_eq!(false, cpu.z);
        assert_eq!(true, cpu.n);
    }

    #[test]
    fn test_asl_zpx() {
        let mut cpu = CPU::new();
        let mut bus = MockBus::new();

        cpu.x = 0x2;
        bus.expect_read()
            .with(eq(0x2))
            .times(1)
            .return_const(0x1);

        bus.expect_write()
            .with(eq(0x2), eq(0x2))
            .times(1)
            .return_const(());

        assert_eq!(6, cpu.asl(&cpu.zpx(0x0, &bus), &bus, 0x2));

        assert_eq!(false, cpu.c);
        assert_eq!(false, cpu.z);
        assert_eq!(false, cpu.n);
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
