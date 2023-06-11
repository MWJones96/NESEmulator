/*
    INX - Increment Index Register X By One
    Operation: X + 1 → X

    Increment X adds 1 to the current value of the X register.
    This is an 8-bit increment which does not affect the carry
    operation, therefore, if the value of X before the increment
    was FF, the resulting value is 00.

    INX does not affect the carry or overflow flags; it sets the
    N flag if the result of the increment has a one in bit 7,
    otherwise resets N; sets the Z flag if the result of the
    increment is 0, otherwise it resets the Z flag.

    INX does not affect any other register other than the X register.
*/

use crate::{
    bus::Bus,
    cpu::{addr::AddrModeResult, NESCPU},
};

impl NESCPU {
    pub(in crate::cpu) fn inxc(&self, _mode: &AddrModeResult) -> u8 {
        2
    }

    pub(in crate::cpu) fn inx(&mut self, _mode: &AddrModeResult, _bus: &mut dyn Bus) {
        self.x = self.x.wrapping_add(1);

        self.n = (self.x & 0x80) > 0;
        self.z = self.x == 0;
    }
}

#[cfg(test)]
mod inx_tests {
    use crate::bus::MockBus;

    use super::*;

    #[test]
    fn test_inx_returns_correct_number_of_cycles() {
        let cpu = NESCPU::new();
        assert_eq!(2, cpu.inxc(&cpu._imp()));
    }

    #[test]
    fn test_inx() {
        let mut cpu = NESCPU::new();
        cpu.x = 0x80;

        cpu.inx(&cpu._imp(), &mut MockBus::new());

        assert_eq!(0x81, cpu.x);
    }

    #[test]
    fn test_inx_negative_flag() {
        let mut cpu = NESCPU::new();
        cpu.x = 0x7f;

        cpu.inx(&cpu._imp(), &mut MockBus::new());

        assert_eq!(0x80, cpu.x);
        assert_eq!(true, cpu.n);
    }

    #[test]
    fn test_inx_zero_flag() {
        let mut cpu = NESCPU::new();
        cpu.x = 0xff;

        cpu.inx(&cpu._imp(), &mut MockBus::new());

        assert_eq!(0x0, cpu.x);
        assert_eq!(true, cpu.z);
    }
}
