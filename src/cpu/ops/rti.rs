/*
    RTI - Return From Interrupt
    Operation: P↑ PC↑

    This instruction transfers from the stack into the
    microprocessor the processor status and the program
    counter location for the instruction which was
    interrupted. By virtue of the interrupt having stored
    this data before executing the instruction and the
    fact that the RTI reinitializes the microprocessor
    to the same state as when it was interrupted, the
    combination of interrupt plus RTI allows truly
    reentrant coding.

    The RTI instruction reinitializes all flags to the
    position to the point they were at the time the
    interrupt was taken and sets the program counter
    back to its pre-interrupt state. It affects no other
    registers in the microprocessor.
*/

use crate::cpu::{addr::AddrModeResult, bus::CPUBus, CPU};

impl CPU {
    #[inline]
    pub(in crate::cpu) fn rtic(&self, _mode: &AddrModeResult) -> u8 {
        6
    }

    #[inline]
    pub(in crate::cpu) fn rti(&mut self, _mode: &AddrModeResult, bus: &dyn CPUBus) {
        let reg = bus.read(0x100 + (self.sp.wrapping_add(1) as u16));
        let pc_low = bus.read(0x100 + (self.sp.wrapping_add(2) as u16)) as u16;
        let pc_high = bus.read(0x100 + (self.sp.wrapping_add(3) as u16)) as u16;

        self.sp = self.sp.wrapping_add(3);

        self.n = (reg & 0x80) > 0;
        self.v = (reg & 0x40) > 0;
        self.d = (reg & 0x8) > 0;
        self.i = (reg & 0x4) > 0;
        self.z = (reg & 0x2) > 0;
        self.c = (reg & 0x1) > 0;

        self.pc = pc_high << 8 | pc_low;
    }
}

#[cfg(test)]
mod rti_tests {
    use mockall::predicate::eq;

    use crate::cpu::bus::MockCPUBus;

    use super::*;

    #[test]
    fn test_rti_correct_number_ofc() {
        let cpu = CPU::new();

        assert_eq!(6, cpu.rtic(&cpu._imp()));
    }

    #[test]
    fn test_rti_returns_status_register_all_flags() {
        let mut cpu = CPU::new();
        let mut bus = MockCPUBus::new();
        cpu.sp = 0xfc;

        bus.expect_read()
            .with(eq(0x1fd))
            .times(1)
            .return_const(0xff);

        bus.expect_read().return_const(0x0);

        cpu.rti(&cpu._imp(), &bus);

        assert_eq!(true, cpu.n);
        assert_eq!(true, cpu.v);
        assert_eq!(true, cpu.d);
        assert_eq!(true, cpu.i);
        assert_eq!(true, cpu.z);
        assert_eq!(true, cpu.c);

        assert_eq!(0xff, cpu.sp);
    }

    #[test]
    fn test_rti_returns_status_register_no_flags() {
        let mut cpu = CPU::new();
        let mut bus = MockCPUBus::new();
        cpu.sp = 0xfc;

        bus.expect_read().return_const(0x0);

        cpu.rti(&cpu._imp(), &bus);

        assert_eq!(false, cpu.n);
        assert_eq!(false, cpu.v);
        assert_eq!(false, cpu.d);
        assert_eq!(false, cpu.i);
        assert_eq!(false, cpu.z);
        assert_eq!(false, cpu.c);

        assert_eq!(0xff, cpu.sp);
    }

    #[test]
    fn test_rti_returns_pc() {
        let mut cpu = CPU::new();
        let mut bus = MockCPUBus::new();
        cpu.sp = 0xfc;

        bus.expect_read()
            .with(eq(0x1fe))
            .times(1)
            .return_const(0x40);

        bus.expect_read()
            .with(eq(0x1ff))
            .times(1)
            .return_const(0x20);

        bus.expect_read().return_const(0x0);

        cpu.rti(&cpu._imp(), &bus);

        assert_eq!(0x2040, cpu.pc);
        assert_eq!(0xff, cpu.sp);
    }
}
