/*
    PHA - Push Accumulator On Stack
    Operation: A↓

    This instruction transfers the current value of
    the accumulator to the next location on the stack,
    automatically decrementing the stack to point to
    the next empty location.

    The Push A instruction only affects the stack pointer
    register which is decremented by 1 as a result of the
    operation. It affects no flags.
*/

use crate::{bus::Bus, cpu::addr::AddrModeResult};

use super::super::NESCPU;

impl NESCPU {
    pub(in crate::cpu) fn phac(&self, _mode: &AddrModeResult) -> u8 {
        3
    }

    pub(in crate::cpu) fn pha(&mut self, _mode: &AddrModeResult, bus: &mut dyn Bus) {
        bus.write(0x100 + (self.sp as u16), self.a);
        self.sp = self.sp.wrapping_sub(1);
    }
}

#[cfg(test)]
mod pha_tests {
    use mockall::predicate::eq;

    use crate::bus::MockBus;

    use super::*;

    #[test]
    fn test_pha_correct_number_of_cycles() {
        let cpu = NESCPU::new();
        assert_eq!(3, cpu.phac(&cpu._imp()));
    }

    #[test]
    fn test_pha_push_acc_onto_stack() {
        let mut cpu = NESCPU::new();
        let mut bus = MockBus::new();
        cpu.a = 0xee;

        bus.expect_write()
            .with(eq(0x1ff), eq(0xee))
            .times(1)
            .return_const(());

        cpu.pha(&cpu._imp(), &mut bus);
        assert_eq!(0xfe, cpu.sp);
    }

    #[test]
    fn test_pha_push_acc_onto_stack_with_overflow() {
        let mut cpu = NESCPU::new();
        let mut bus = MockBus::new();
        cpu.sp = 0x0;

        bus.expect_write().return_const(());

        cpu.pha(&cpu._imp(), &mut bus);
        assert_eq!(0xff, cpu.sp);
    }
}
