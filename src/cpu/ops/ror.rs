/*
    ROR - Rotate Right
    Operation: C → /M7...M0/ → C

    The rotate right instruction shifts either the accumulator
    or addressed memory right 1 bit with bit 0 shifted into the
    carry and carry shifted into bit 7.

    The ROR instruction either shifts the accumulator right 1 bit
    and stores the carry in accumulator bit 7 or does not affect
    the internal regis­ ters at all. The ROR instruction sets carry
    equal to input bit 0, sets N equal to the input carry and sets
    the Z flag if the result of the rotate is 0; otherwise it resets
    Z and does not affect the overflow flag at all.
*/

use crate::{
    bus::Bus,
    cpu::addr::{AddrModeResult, AddrModeType},
};

use super::super::NESCPU;

impl NESCPU {
    pub(in crate::cpu) fn rorc(&self, mode: &AddrModeResult) -> u8 {
        match mode.mode {
            AddrModeType::Acc => 2,
            AddrModeType::Absx => 7,
            _ => 4 + mode.cycles,
        }
    }

    pub(in crate::cpu) fn ror(&mut self, mode: &AddrModeResult, bus: &mut dyn Bus) {
        if let Some(addr) = mode.addr {
            let before = bus.read(mode.addr.unwrap());
            let after: u8 = ((self.c as u8) << 7) | before >> 1;

            self.c = (before & 0x1) != 0;
            self.n = (after & 0x80) != 0;
            self.z = after == 0;

            bus.write(addr, after);
        } else {
            let before = mode.data.unwrap();
            let after: u8 = ((self.c as u8) << 7) | before >> 1;

            self.c = (before & 0x1) != 0;
            self.n = (after & 0x80) != 0;
            self.z = after == 0;

            self.a = after;
        }
    }
}

#[cfg(test)]
mod ror_tests {
    use crate::bus::MockBus;

    use super::*;

    #[test]
    fn test_ror_acc() {
        let mut cpu = NESCPU::new();

        cpu.a = 0x20;
        assert_eq!(2, cpu.rorc(&cpu._acc()));
    }

    #[test]
    fn test_ror_zp() {
        let cpu = NESCPU::new();
        let bus = MockBus::new();

        assert_eq!(5, cpu.rorc(&cpu._zp(0x0, &bus)));
    }

    #[test]
    fn test_ror_zpx() {
        let mut cpu = NESCPU::new();
        let bus = MockBus::new();

        cpu.x = 0x2;

        assert_eq!(6, cpu.rorc(&cpu._zpx(0x0, &bus)));
    }

    #[test]
    fn test_ror_abs() {
        let cpu = NESCPU::new();
        let bus = MockBus::new();

        assert_eq!(6, cpu.rorc(&cpu._abs(0xffff, &bus)));
    }

    #[test]
    fn test_ror_absx() {
        let mut cpu = NESCPU::new();
        let bus = MockBus::new();

        cpu.x = 0x2;

        assert_eq!(7, cpu.rorc(&cpu._absx(0xffff, &bus)));
    }

    #[test]
    fn test_ror_carry_flag() {
        let mut cpu = NESCPU::new();
        let mut bus = MockBus::new();

        cpu.a = 0b0000_0001;
        cpu.c = true;

        cpu.ror(&cpu._acc(), &mut bus);
        assert_eq!(0b1000_0000, cpu.a);
        assert_eq!(true, cpu.c);
    }

    #[test]
    fn test_ror_no_carry_flag() {
        let mut cpu = NESCPU::new();
        let mut bus = MockBus::new();

        cpu.a = 0b0000_0001;

        cpu.ror(&cpu._acc(), &mut bus);
        assert_eq!(0b0000_0000, cpu.a);
        assert_eq!(true, cpu.c);
    }

    #[test]
    fn test_ror_negative_flag() {
        let mut cpu = NESCPU::new();
        let mut bus = MockBus::new();

        cpu.a = 0x0;
        cpu.c = true;

        cpu.ror(&cpu._acc(), &mut bus);
        assert_eq!(0x80, cpu.a);
        assert_eq!(true, cpu.n);
    }

    #[test]
    fn test_ror_zero_flag() {
        let mut cpu = NESCPU::new();
        let mut bus = MockBus::new();

        cpu.a = 0x1;
        cpu.ror(&cpu._acc(), &mut bus);

        assert_eq!(true, cpu.z);
    }
}
