use crate::cpu::{addr::AddrModeResult, CPU};

impl CPU {
    pub(in crate::cpu) fn jmp_cycles(&self, mode: &AddrModeResult) -> u8 {
        1 + mode.cycles
    }

    pub(in crate::cpu) fn jmp(&self, mode: &AddrModeResult) {}
}

#[cfg(test)]
mod jmp_tests {
    use crate::cpu::bus::MockBus;

    use super::*;

    #[test]
    fn test_jmp_abs_correct_number_of_cycles() {
        let cpu = CPU::new();
        let mut bus = MockBus::new();

        bus.expect_read().return_const(0x0);

        assert_eq!(3, cpu.jmp_cycles(&cpu.abs(0x0000, &bus)));
    }
}
