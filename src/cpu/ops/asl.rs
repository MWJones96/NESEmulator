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

use crate::{
    bus::Bus,
    cpu::addr::{AddrModeResult, AddrModeType},
};

use super::super::NESCPU;

impl NESCPU {
    pub(in crate::cpu) fn aslc(&self, mode: &AddrModeResult) -> u8 {
        match mode.mode {
            AddrModeType::Acc => 2,
            AddrModeType::Absx => 7,
            _ => 4 + mode.cycles,
        }
    }

    pub(in crate::cpu) fn asl(&mut self, mode: &AddrModeResult, bus: &mut dyn Bus) {
        if let Some(addr) = mode.addr {
            let data: u16 = (bus.read(mode.addr.unwrap()) as u16) << 1;

            self.c = data > (u8::MAX as u16);
            self.z = (data & 0xff) == 0;
            self.n = ((data & 0xff) & 0x80) != 0;

            bus.write(addr, data as u8);
        } else {
            let data: u16 = (mode.data.unwrap() as u16) << 1;
            self.c = data > (u8::MAX as u16);
            self.z = (data & 0xff) == 0;
            self.n = ((data & 0xff) & 0x80) != 0;

            self.a = data as u8;
        }
    }
}

#[cfg(test)]
mod asl_tests {
    use mockall::predicate::eq;

    use crate::bus::MockBus;

    use super::*;

    #[test]
    fn test_asl_acc() {
        let mut cpu = NESCPU::new();

        cpu.a = 0x20;
        assert_eq!(2, cpu.aslc(&cpu._acc()));
    }

    #[test]
    fn test_asl_zp() {
        let cpu = NESCPU::new();
        let bus = MockBus::new();

        assert_eq!(5, cpu.aslc(&cpu._zp(0x0, &bus)));
    }

    #[test]
    fn test_asl_zpx() {
        let mut cpu = NESCPU::new();
        let bus = MockBus::new();

        cpu.x = 0x2;

        assert_eq!(6, cpu.aslc(&cpu._zpx(0x0, &bus)));
    }

    #[test]
    fn test_asl_abs() {
        let cpu = NESCPU::new();
        let bus = MockBus::new();

        assert_eq!(6, cpu.aslc(&cpu._abs(0xffff, &bus)));
    }

    #[test]
    fn test_asl_absx() {
        let mut cpu = NESCPU::new();
        let bus = MockBus::new();

        cpu.x = 0x2;

        assert_eq!(7, cpu.aslc(&cpu._absx(0xffff, &bus)));
    }

    #[test]
    fn test_asl_shift() {
        let mut cpu = NESCPU::new();
        let mut bus = MockBus::new();

        cpu.a = 0x1;
        cpu.asl(&cpu._acc(), &mut bus);
        assert_eq!(0x2, cpu.a);

        cpu.a = 0xff;
        cpu.asl(&cpu._acc(), &mut bus);
        assert_eq!(0xfe, cpu.a);

        cpu.a = 0x00;
        cpu.asl(&cpu._acc(), &mut bus);
        assert_eq!(0x00, cpu.a);

        cpu.c = true;
        cpu.a = 0x00;
        cpu.asl(&cpu._acc(), &mut bus);
        assert_eq!(0x00, cpu.a);
    }

    #[test]
    fn test_asl_carry_flag() {
        let mut cpu = NESCPU::new();
        let mut bus = MockBus::new();

        cpu.a = 0xc0;
        cpu.asl(&cpu._acc(), &mut bus);
        assert_eq!(true, cpu.c);

        cpu.a = 0x1;
        cpu.asl(&cpu._acc(), &mut bus);
        assert_eq!(false, cpu.c);
    }

    #[test]
    fn test_asl_acc_zero_flag() {
        let mut cpu = NESCPU::new();
        let mut bus = MockBus::new();

        cpu.a = 0x80;
        cpu.asl(&cpu._acc(), &mut bus);
        assert_eq!(true, cpu.z);

        cpu.a = 0x40;
        cpu.asl(&cpu._acc(), &mut bus);
        assert_eq!(false, cpu.z);
    }

    #[test]
    fn test_asl_acc_negative_flag() {
        let mut cpu = NESCPU::new();
        let mut bus = MockBus::new();

        cpu.a = 0x40;
        cpu.asl(&cpu._acc(), &mut bus);
        assert_eq!(true, cpu.n);

        cpu.a = 0x80;
        cpu.asl(&cpu._acc(), &mut bus);
        assert_eq!(false, cpu.n);
    }

    #[test]
    fn test_asl_writes_to_memory() {
        let mut cpu = NESCPU::new();
        let mut bus = MockBus::new();

        bus.expect_read().with(eq(0x0)).times(1).return_const(0xff);

        bus.expect_write()
            .with(eq(0x0), eq(0xfe))
            .times(1)
            .return_const(());

        cpu.asl(&cpu._zp(0x0, &bus), &mut bus);
    }
}
