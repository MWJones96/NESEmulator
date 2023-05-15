/*
    PLP - Pull Processor Status From Stack
    Operation: P↑

    This instruction transfers the next value on the
    stack to the Proces­ sor Status register, thereby
    changing all of the flags and setting the mode
    switches to the values from the stack.

    The PLP instruction affects no registers in the
    processor other than the status register. This
    instruction could affect all flags in the status
    register.
*/

use crate::cpu::{addr::AddrModeResult, bus::CPUBus};

use super::super::CPU;

impl CPU {
    pub(in crate::cpu) fn plp_cycles(&self, _mode: &AddrModeResult) -> u8 {
        4
    }

    pub(in crate::cpu) fn plp(&mut self, _mode: &AddrModeResult, bus: &impl CPUBus) {
        self.sp = self.sp.wrapping_add(1);
        let data = bus.read(0x100 + (self.sp as u16));

        self.n = (data & 0x80) > 0;
        self.v = (data & 0x40) > 0;
        self.d = (data & 0x8) > 0;
        self.i = (data & 0x4) > 0;
        self.z = (data & 0x2) > 0;
        self.c = (data & 0x1) > 0;
    }
}

#[cfg(test)]
mod plp_tests {
    use mockall::predicate::eq;

    use crate::cpu::bus::MockCPUBus;

    use super::*;

    #[test]
    fn test_plp_correct_number_of_cycles() {
        let cpu = CPU::new();
        assert_eq!(4, cpu.plp_cycles(&cpu.imp()));
    }

    #[test]
    fn test_plp_fetches_all_flags() {
        let mut cpu = CPU::new();
        let mut bus = MockCPUBus::new();
        cpu.sp = 0xfe;

        bus.expect_read()
            .with(eq(0x1ff))
            .times(1)
            .return_const(0xff);

        cpu.plp(&cpu.imp(), &bus);

        assert_eq!(true, cpu.n);
        assert_eq!(true, cpu.v);
        assert_eq!(true, cpu.d);
        assert_eq!(true, cpu.i);
        assert_eq!(true, cpu.z);
        assert_eq!(true, cpu.c);

        assert_eq!(0xff, cpu.sp);
    }

    #[test]
    fn test_plp_fetches_no_flags() {
        let mut cpu = CPU::new();
        let mut bus = MockCPUBus::new();
        cpu.sp = 0xfe;

        bus.expect_read()
            .with(eq(0x1ff))
            .times(1)
            .return_const(0x00);

        cpu.plp(&cpu.imp(), &bus);

        assert_eq!(false, cpu.n);
        assert_eq!(false, cpu.v);
        assert_eq!(false, cpu.d);
        assert_eq!(false, cpu.i);
        assert_eq!(false, cpu.z);
        assert_eq!(false, cpu.c);

        assert_eq!(0xff, cpu.sp);
    }

    #[test]
    fn test_plp_pull_from_empty_stack() {
        let mut cpu = CPU::new();
        let mut bus = MockCPUBus::new();
        cpu.sp = 0xff;

        bus.expect_read().with(eq(0x100)).times(1).return_const(0x0);

        cpu.pla(&cpu.imp(), &bus);
        assert_eq!(0x0, cpu.sp);
    }
}
