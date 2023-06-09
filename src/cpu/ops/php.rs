/*
    PHP - Push Processor Status On Stack
    Operation: P↓

    This instruction transfers the contents of the processor
    status reg­ ister unchanged to the stack, as governed by
    the stack pointer.

    The PHP instruction affects no registers or flags in the
    micropro­cessor.
*/

use crate::{bus::Bus, cpu::addr::AddrModeResult};

use super::super::NESCPU;

impl NESCPU {
    pub(in crate::cpu) fn phpc(&self, _mode: &AddrModeResult) -> u8 {
        3
    }

    pub(in crate::cpu) fn php(&mut self, _mode: &AddrModeResult, bus: &mut dyn Bus) {
        bus.write(0x100 + (self.sp as u16), self.get_status_byte(true));
        self.sp = self.sp.wrapping_sub(1);
    }
}

#[cfg(test)]
mod php_tests {
    use mockall::predicate::eq;

    use crate::bus::MockBus;

    use super::*;

    #[test]
    fn test_php_correct_number_of_cycles() {
        let cpu = NESCPU::new();
        assert_eq!(3, cpu.phpc(&cpu._imp()));
    }

    #[test]
    fn test_php_push_status_onto_stack() {
        let mut cpu = NESCPU::new();
        let mut bus = MockBus::new();
        cpu.c = true;

        bus.expect_write()
            .with(eq(0x1ff), eq(0b0011_0101))
            .times(1)
            .return_const(());

        cpu.php(&cpu._imp(), &mut bus);
        assert_eq!(0xfe, cpu.sp);
    }

    #[test]
    fn test_php_push_status_onto_stack_with_overflow() {
        let mut cpu = NESCPU::new();
        let mut bus = MockBus::new();
        cpu.sp = 0x0;

        bus.expect_write().return_const(());

        cpu.php(&cpu._imp(), &mut bus);
        assert_eq!(0xff, cpu.sp);
    }
}
