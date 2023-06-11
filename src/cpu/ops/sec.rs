/*
    SEC - Set Carry Flag
    Operation: 1 → C

    This instruction initializes the carry flag to a 1. This operation should
    normally precede a SBC loop. It is also useful when used with a ROL
    instruction to initialize a bit in memory to a 1.

    This instruction affects no registers in the microprocessor and no flags
    other than the carry flag which is set.
*/

use crate::{bus::Bus, cpu::addr::AddrModeResult};

use super::super::NESCPU;

impl NESCPU {
    pub(in crate::cpu) fn secc(&self, _mode: &AddrModeResult) -> u8 {
        2
    }

    pub(in crate::cpu) fn sec(&mut self, _mode: &AddrModeResult, _bus: &mut dyn Bus) {
        self.c = true;
    }
}

#[cfg(test)]
mod sec_tests {
    use crate::bus::MockBus;

    use super::*;

    #[test]
    fn test_sec_correct_number_of_cycles() {
        let cpu = NESCPU::new();

        assert_eq!(2, cpu.secc(&cpu._imp()));
    }

    #[test]
    fn test_sec_carry_flag() {
        let mut cpu = NESCPU::new();
        cpu.c = false;

        cpu.sec(&cpu._imp(), &mut MockBus::new());
        assert_eq!(true, cpu.c);

        cpu.sec(&cpu._imp(), &mut MockBus::new());
        assert_eq!(true, cpu.c);
    }
}
