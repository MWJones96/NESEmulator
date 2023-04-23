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

use crate::cpu::{
    addr::{AddrMode, AddrModeResult},
    bus::Bus,
};

use super::super::CPU;

impl CPU {
    pub(in crate::cpu) fn asl_cycles(&self, mode: &AddrModeResult) -> u8 {
        match mode.mode {
            AddrMode::ACC => 2,
            AddrMode::ABSX => 7,
            _ => 4 + mode.cycles,
        }
    }

    pub(in crate::cpu) fn asl(&mut self, mode: &AddrModeResult, bus: &dyn Bus) {
        let data = self._asl(mode.data.unwrap());

        if let Some(addr) = mode.addr {
            bus.write(addr, data);
        } else {
            self.a = data;
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

        cpu.a = 0x20;
        assert_eq!(2, cpu.asl_cycles(&cpu.acc()));
    }

    #[test]
    fn test_asl_zp() {
        let cpu = CPU::new();
        let mut bus = MockBus::new();

        bus.expect_read().with(eq(0x0)).times(1).return_const(0xff);

        assert_eq!(5, cpu.asl_cycles(&cpu.zp(0x0, &bus)));
    }

    #[test]
    fn test_asl_zpx() {
        let mut cpu = CPU::new();
        let mut bus = MockBus::new();

        cpu.x = 0x2;
        bus.expect_read().with(eq(0x2)).times(1).return_const(0x1);

        assert_eq!(6, cpu.asl_cycles(&cpu.zpx(0x0, &bus)));
    }

    #[test]
    fn test_asl_abs() {
        let cpu = CPU::new();
        let mut bus = MockBus::new();

        bus.expect_read()
            .with(eq(0xffff))
            .times(1)
            .return_const(0xaa);

        assert_eq!(6, cpu.asl_cycles(&cpu.abs(0xffff, &bus)));
    }

    #[test]
    fn test_asl_absx() {
        let mut cpu = CPU::new();
        let mut bus = MockBus::new();

        cpu.x = 0x2;

        bus.expect_read().with(eq(0x1)).times(1).return_const(0x88);

        assert_eq!(7, cpu.asl_cycles(&cpu.absx(0xffff, &bus)));
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
