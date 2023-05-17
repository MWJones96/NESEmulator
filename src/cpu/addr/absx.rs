/*
    X-Indexed Absolute

    This form of addressing is used in conjunction with the
    X index register. The effective address is formed by
    adding the contents of X to the address contained in
    the second and third bytes of the instruction. This mode
    allows the index register to contain the index or count
    value and the instruction to contain the base address.
    This type of indexing allows any location referencing
    and the index to modify multiple fields resulting in
    reduced coding and execution time.

    Bytes: 3
*/

use crate::cpu::{bus::CPUBus, CPU};

use super::{AddrMode, AddrModeResult};

impl CPU {
    #[inline]
    pub(in crate::cpu) fn absx(&self, addr: u16, bus: &impl CPUBus) -> AddrModeResult {
        self.abs_helper(addr, self.x, AddrMode::ABSX, bus)
    }
}

#[cfg(test)]
mod absx_tests {
    use mockall::predicate::eq;

    use super::*;
    use crate::cpu::{addr::AddrModeResult, bus::MockCPUBus};

    #[test]
    fn test_absx_addressing_mode_no_page_cross() {
        let mut cpu = CPU::new();
        let mut mock_bus = MockCPUBus::new();

        cpu.x = 0x2;

        mock_bus.expect_read().with(eq(0x2)).return_const(0x88);

        let result = cpu.absx(0x0, &mock_bus);
        assert_eq!(
            AddrModeResult {
                data: Some(0x88),
                cycles: 2,
                mode: crate::cpu::addr::AddrMode::ABSX,
                addr: Some(0x2)
            },
            result
        );
    }

    #[test]
    fn test_absx_addressing_mode_with_page_cross() {
        let mut cpu = CPU::new();
        let mut mock_bus = MockCPUBus::new();

        cpu.x = 0x2;

        mock_bus.expect_read().with(eq(0x1)).return_const(0x88);

        let result = cpu.absx(0xffff, &mock_bus);
        assert_eq!(
            AddrModeResult {
                data: Some(0x88),
                cycles: 3,
                mode: AddrMode::ABSX,
                addr: Some(0x1)
            },
            result
        );
    }
}
