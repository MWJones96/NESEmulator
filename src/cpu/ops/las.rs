/*
    LAS - "AND" Memory with Stack Pointer
    Operation: M ∧ S → A, X, S

    This undocumented instruction performs a bit-by-bit "AND"
    operation of the stack pointer and memory and stores the
    result back in the accumulator, the index register X and
    the stack pointer.

    The LAS instruction does not affect the carry or overflow flags.
    It sets N if the bit 7 of the result is on, otherwise it is reset.
    If the result is zero, then the Z flag is set, otherwise it is reset.
*/

use crate::{
    bus::Bus,
    cpu::{addr::AddrModeResult, NESCPU},
};

impl NESCPU {
    pub(in crate::cpu) fn lasc(&self, mode: &AddrModeResult) -> u8 {
        2 + mode.cycles
    }

    pub(in crate::cpu) fn las(&mut self, mode: &AddrModeResult, _bus: &mut dyn Bus) {
        let data = mode.data.unwrap() & self.sp;

        self.a = data;
        self.x = data;
        self.sp = data;
    }
}

#[cfg(test)]
mod las_tests {
    use crate::bus::MockBus;

    use super::*;

    #[test]
    fn test_las_cycles_absy_no_page_cross() {
        let cpu = NESCPU::new();
        let mut bus = MockBus::new();
        bus.expect_read().return_const(0x0);

        assert_eq!(4, cpu.lasc(&cpu._absy(0x0, &bus)));
    }

    #[test]
    fn test_las_cycles_absy_with_page_cross() {
        let mut cpu = NESCPU::new();
        cpu.y = 0xff;

        let mut bus = MockBus::new();
        bus.expect_read().return_const(0x0);

        assert_eq!(5, cpu.lasc(&cpu._absy(0x34, &bus)));
    }

    #[test]
    fn test_las() {
        let mut cpu = NESCPU::new();
        cpu.sp = 0b1111_0000;

        let mut bus = MockBus::new();
        bus.expect_read().return_const(0b1111_1100);

        cpu.las(&cpu._absy(0x0, &bus), &mut MockBus::new());

        assert_eq!(0xf0, cpu.a);
        assert_eq!(0xf0, cpu.x);
        assert_eq!(0xf0, cpu.sp);
    }
}
