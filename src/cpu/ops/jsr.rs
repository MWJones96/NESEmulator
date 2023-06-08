/*
    JSR - Jump To Subroutine
    Operation: PC + 2↓, [PC + 1] → PCL, [PC + 2] → PCH

    This instruction transfers control of the program counter
    to a subroutine location but leaves a return pointer on
    the stack to allow the user to return to perform the next
    instruction in the main program after the subroutine is
    complete. To accomplish this, JSR instruction stores the
    program counter address which points to the last byte of
    the jump instruc­tion onto the stack using the stack pointer.
    The stack byte contains the program count high first,
    followed by program count low. The JSR then transfers the
    addresses following the jump instruction to the program
    counter low and the program counter high, thereby directing
    the program to begin at that new address.

    The JSR instruction affects no flags, causes the stack pointer
    to be decremented by 2 and substitutes new values into the
    program counter low and the program counter high.
*/

use crate::cpu::{addr::AddrModeResult, bus::CPUBus, CPU};

impl CPU {
    pub(in crate::cpu) fn jsrc(&self, _mode: &AddrModeResult) -> u8 {
        6
    }

    pub(in crate::cpu) fn jsr(&mut self, mode: &AddrModeResult, bus: &mut dyn CPUBus) {
        //Set return address to last byte of JSR
        let return_addr = self.pc.wrapping_sub(1);

        let pc_high_byte = (return_addr >> 8) as u8;
        let pc_low_byte = (return_addr & 0xff) as u8;

        bus.write(0x100 + self.sp.wrapping_sub(0) as u16, pc_high_byte);
        bus.write(0x100 + self.sp.wrapping_sub(1) as u16, pc_low_byte);

        self.sp = self.sp.wrapping_sub(2);
        self.pc = mode.addr.unwrap();
    }
}

#[cfg(test)]
mod jsr_tests {
    use mockall::predicate::eq;

    use crate::cpu::bus::MockCPUBus;

    use super::*;

    #[test]
    fn test_jsr_correct_number_of_cycles() {
        let cpu = CPU::new();
        let mut bus = MockCPUBus::new();

        bus.expect_read().return_const(0x0);

        assert_eq!(6, cpu.jsrc(&cpu._abs(0x0000, &bus)));
    }

    #[test]
    fn test_jsr_pushes_onto_stack() {
        let mut cpu = CPU::new();
        let mut bus = MockCPUBus::new();

        cpu.pc = 0x1234;
        cpu.sp = 0x1;

        bus.expect_read().return_const(0x0);

        bus.expect_write()
            .with(eq(0x101), eq(0x12))
            .times(1)
            .return_const(());

        bus.expect_write()
            .with(eq(0x100), eq(0x33))
            .times(1)
            .return_const(());

        cpu.jsr(&cpu._abs(0x0000, &bus), &mut bus);

        assert_eq!(0xff, cpu.sp);
    }

    #[test]
    fn test_jsr_sets_pc_to_new_value() {
        let mut cpu = CPU::new();
        let mut bus = MockCPUBus::new();

        bus.expect_read().return_const(0x0);

        bus.expect_write().return_const(());

        cpu.jsr(&cpu._abs(0xeeee, &bus), &mut bus);

        assert_eq!(0xeeee, cpu.pc);
    }
}
