/*
    RTS - Return From Subroutine
    Operation: PC↑, PC + 1 → PC

    This instruction loads the program count low and
    program count high from the stack into the program
    counter and increments the program counter so that
    it points to the instruction following the JSR.
    The stack pointer is adjusted by incrementing
    it twice.

    The RTS instruction does not affect any flags and
    affects only PCL and PCH.
*/

use crate::cpu::{addr::AddrModeResult, bus::CPUBus, NESCPU};

impl NESCPU {
    pub(in crate::cpu) fn rtsc(&self, _mode: &AddrModeResult) -> u8 {
        6
    }

    pub(in crate::cpu) fn rts(&mut self, _mode: &AddrModeResult, bus: &mut dyn CPUBus) {
        let pc_low = bus.read(0x100 + (self.sp.wrapping_add(1) as u16)) as u16;
        let pc_high = bus.read(0x100 + (self.sp.wrapping_add(2) as u16)) as u16;

        self.sp = self.sp.wrapping_add(2);
        //Jump to next instruction after JSR
        self.pc = (pc_high << 8 | pc_low).wrapping_add(1);
    }
}

#[cfg(test)]
mod rts_tests {
    use mockall::predicate::eq;

    use crate::cpu::bus::MockCPUBus;

    use super::*;

    #[test]
    fn test_rts_correct_number_of_cycles() {
        let cpu = NESCPU::new();

        assert_eq!(6, cpu.rtsc(&cpu._imp()));
    }

    #[test]
    fn test_rts_returns_pc() {
        let mut cpu = NESCPU::new();
        let mut bus = MockCPUBus::new();
        cpu.sp = 0xfd;

        bus.expect_read()
            .with(eq(0x1fe))
            .times(1)
            .return_const(0x40);

        bus.expect_read()
            .with(eq(0x1ff))
            .times(1)
            .return_const(0x20);

        bus.expect_read().return_const(0x0);

        cpu.rts(&cpu._imp(), &mut bus);

        assert_eq!(0x2041, cpu.pc);
        assert_eq!(0xff, cpu.sp);
    }
}
