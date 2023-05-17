/*
    Absolute Indirect
    The second byte of the instruction contains the low order
    eight bits of a memory location. The high order eight bits
    of that memory location is contained in the third byte of
    the instruction. The contents of the fully specified memory
    location is the low order byte of the effective address.
    The next memory location contains the high order byte of the
    effective address which is loaded into the sixteen bits of
    the program counter.

    Note on the MOS 6502:
    The indirect jump instruction does not increment the page
    address when the indirect pointer crosses a page boundary.
    JMP ($xxFF) will fetch the address from $xxFF and $xx00.

    Bytes: 3
*/

use crate::cpu::{bus::CPUBus, CPU};

use super::{AddrMode, AddrModeResult};

impl CPU {
    #[inline]
    pub(in crate::cpu) fn ind(&self, addr: u16, bus: &impl CPUBus) -> AddrModeResult {
        let low_byte = bus.read(addr) as u16;
        let high_byte =
            bus.read((addr & 0xff00) + ((addr & 0xff) as u8).wrapping_add(1) as u16) as u16;

        let resolved_addr = high_byte << 8 | low_byte;

        AddrModeResult {
            data: None,
            cycles: 4,
            mode: AddrMode::IND,
            addr: Some(resolved_addr),
        }
    }
}

#[cfg(test)]
mod ind_tests {
    use mockall::predicate::eq;

    use crate::cpu::bus::MockCPUBus;

    use super::*;

    #[test]
    fn test_ind_addressing_mode() {
        let cpu = CPU::new();
        let mut bus = MockCPUBus::new();

        bus.expect_read()
            .with(eq(0x0000))
            .times(1)
            .return_const(0x40);

        bus.expect_read()
            .with(eq(0x0001))
            .times(1)
            .return_const(0x20);

        let ind = cpu.ind(0x0000, &bus);
        assert_eq!(
            AddrModeResult {
                data: None,
                cycles: 4,
                mode: AddrMode::IND,
                addr: Some(0x2040)
            },
            ind
        );
    }

    #[test]
    fn test_ind_addressing_mode_hardware_bug() {
        let cpu = CPU::new();
        let mut bus = MockCPUBus::new();

        bus.expect_read()
            .with(eq(0x80ff))
            .times(1)
            .return_const(0x40);

        bus.expect_read()
            .with(eq(0x8000))
            .times(1)
            .return_const(0x20);

        let ind = cpu.ind(0x80ff, &bus);
        assert_eq!(
            AddrModeResult {
                data: None,
                cycles: 4,
                mode: AddrMode::IND,
                addr: Some(0x2040)
            },
            ind
        );
    }
}
