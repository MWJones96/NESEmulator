/*
    PLA - Pull Accumulator From Stack
    Operation: A↑

    This instruction adds 1 to the current value of the
    stack pointer and uses it to address the stack and
    loads the contents of the stack into the A register.

    The PLA instruction does not affect the carry or
    overflow flags. It sets N if the bit 7 is on in
    accumulator A as a result of instructions, otherwise
    it is reset. If accumulator A is zero as a result of
    the PLA, then the Z flag is set, otherwise it is reset.
    The PLA instruction changes content of the accumulator
    A to the contents of the memory location at stack register
    plus 1 and also increments the stack register.
*/

use crate::{bus::Bus, cpu::addr::AddrModeResult};

use super::super::NESCPU;

impl NESCPU {
    pub(in crate::cpu) fn plac(&self, _mode: &AddrModeResult) -> u8 {
        4
    }

    pub(in crate::cpu) fn pla(&mut self, _mode: &AddrModeResult, bus: &mut dyn Bus) {
        self.sp = self.sp.wrapping_add(1);

        self.a = bus.read(0x100 + (self.sp as u16));
        self.n = (self.a & 0x80) > 0;
        self.z = self.a == 0;
    }
}

#[cfg(test)]
mod pla_tests {
    use mockall::predicate::eq;

    use crate::bus::MockBus;

    use super::*;

    #[test]
    fn test_pla_correct_number_of_cycles() {
        let cpu = NESCPU::new();
        assert_eq!(4, cpu.plac(&cpu._imp()));
    }

    #[test]
    fn test_pla_fetches_accumulator() {
        let mut cpu = NESCPU::new();
        let mut bus = MockBus::new();
        cpu.sp = 0xfe;

        bus.expect_read()
            .with(eq(0x1ff))
            .times(1)
            .return_const(0xcc);

        cpu.pla(&cpu._imp(), &mut bus);

        assert_eq!(0xcc, cpu.a);
        assert_eq!(0xff, cpu.sp);
    }

    #[test]
    fn test_pla_sets_negative_flag() {
        let mut cpu = NESCPU::new();
        let mut bus = MockBus::new();
        cpu.sp = 0xfe;

        bus.expect_read()
            .with(eq(0x1ff))
            .times(1)
            .return_const(0x80);

        cpu.pla(&cpu._imp(), &mut bus);

        assert_eq!(true, cpu.n);
    }

    #[test]
    fn test_pla_sets_zero_flag() {
        let mut cpu = NESCPU::new();
        let mut bus = MockBus::new();
        cpu.sp = 0xfe;

        bus.expect_read().with(eq(0x1ff)).times(1).return_const(0x0);

        cpu.pla(&cpu._imp(), &mut bus);

        assert_eq!(true, cpu.z);
    }

    #[test]
    fn test_pla_pull_from_empty_stack() {
        let mut cpu = NESCPU::new();
        let mut bus = MockBus::new();
        cpu.sp = 0xff;

        bus.expect_read().with(eq(0x100)).times(1).return_const(0x0);

        cpu.pla(&cpu._imp(), &mut bus);
        assert_eq!(0x0, cpu.sp);
    }
}
