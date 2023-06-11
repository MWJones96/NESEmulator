/*
    DEX - Decrement Index Register X By One
    Operation: X - 1 → X

    This instruction subtracts one from the current value
    of the index register X and stores the result in the
    index register X.

    DEX does not affect the carry or overflow flag, it sets
    the N flag if it has bit 7 on as a result of the decrement,
    otherwise it resets the N flag; sets the Z flag if X is a 0
    as a result of the decrement, otherwise it resets the Z flag.
*/

use crate::{
    bus::Bus,
    cpu::{addr::AddrModeResult, NESCPU},
};

impl NESCPU {
    pub(in crate::cpu) fn dexc(&self, _mode: &AddrModeResult) -> u8 {
        2
    }

    pub(in crate::cpu) fn dex(&mut self, _mode: &AddrModeResult, _bus: &mut dyn Bus) {
        self.x = self.x.wrapping_sub(1);

        self.n = (self.x & 0x80) > 0;
        self.z = self.x == 0;
    }
}

#[cfg(test)]
mod dex_tests {
    use crate::bus::MockBus;

    use super::*;

    #[test]
    fn test_dex_returns_correct_number_of_cycles() {
        let cpu = NESCPU::new();
        assert_eq!(2, cpu.dexc(&cpu._imp()));
    }

    #[test]
    fn test_dex() {
        let mut cpu = NESCPU::new();
        cpu.x = 0x80;

        cpu.dex(&cpu._imp(), &mut MockBus::new());

        assert_eq!(0x7f, cpu.x);
    }

    #[test]
    fn test_dex_negative_flag() {
        let mut cpu = NESCPU::new();
        cpu.x = 0x0;

        cpu.dex(&cpu._imp(), &mut MockBus::new());

        assert_eq!(0xff, cpu.x);
        assert_eq!(true, cpu.n);
    }

    #[test]
    fn test_dex_zero_flag() {
        let mut cpu = NESCPU::new();
        cpu.x = 0x1;

        cpu.dex(&cpu._imp(), &mut MockBus::new());

        assert_eq!(0x0, cpu.x);
        assert_eq!(true, cpu.z);
    }
}
