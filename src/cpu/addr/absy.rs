/*
    Y-Indexed Absolute

    This form of addressing is used in conjunction with the
    Y index register. The effective address is formed by adding
    the contents of Y to the address contained in the second
    and third bytes of the instruction. This mode allows the
    index register to contain the index or count value and the
    instruction to contain the base address. This type of
    indexing allows any location referencing and the index to
    modify multiple fields resulting in reduced coding and
    execution time.

    Bytes: 3
*/

use crate::cpu::{bus::CPUBus, CPU};

use super::{AddrModeResult, AddrModeType};

impl CPU {
    #[inline]
    pub(in crate::cpu) fn absy(&mut self, bus: &dyn CPUBus) -> AddrModeResult {
        let addr = self.fetch_two_bytes_as_u16(bus);
        self._absy(addr, bus)
    }

    #[inline]
    pub(in crate::cpu) fn _absy(&self, addr: u16, bus: &dyn CPUBus) -> AddrModeResult {
        let page_before: u8 = (addr >> 8) as u8;
        let resolved_addr = addr.wrapping_add(self.y as u16);
        let page_after: u8 = (resolved_addr >> 8) as u8;

        AddrModeResult {
            data: Some(bus.read(resolved_addr)),
            cycles: 2 + ((page_before != page_after) as u8),
            mode: AddrModeType::ABSY,
            addr: Some(resolved_addr),
            bytes: 3,
            operands: format!("{:02X} {:02X}", (addr & 0xff) as u8, (addr >> 8) as u8),
            repr: format!("${:04X},Y", addr),
        }
    }
}

#[cfg(test)]
mod absy_tests {
    use mockall::predicate::eq;

    use super::*;
    use crate::cpu::{addr::AddrModeResult, bus::MockCPUBus};

    #[test]
    fn test_absy_addressing_mode_no_page_cross() {
        let mut cpu = CPU::new();
        let mut mock_bus = MockCPUBus::new();

        cpu.y = 0x2;

        mock_bus.expect_read().with(eq(0x2)).return_const(0x88);

        let result = cpu._absy(0x0, &mock_bus);
        assert_eq!(
            AddrModeResult {
                data: Some(0x88),
                cycles: 2,
                addr: Some(0x2),
                mode: AddrModeType::ABSY,
                bytes: 3,
                operands: "00 00".to_owned(),
                repr: "$0000,Y".to_owned()
            },
            result
        );
    }

    #[test]
    fn test_absy_addressing_mode_with_page_cross() {
        let mut cpu = CPU::new();
        let mut mock_bus = MockCPUBus::new();

        cpu.y = 0x2;

        mock_bus.expect_read().with(eq(0x1)).return_const(0x88);

        let result = cpu._absy(0xffff, &mock_bus);
        assert_eq!(
            AddrModeResult {
                data: Some(0x88),
                cycles: 3,
                mode: AddrModeType::ABSY,
                addr: Some(0x1),
                bytes: 3,
                operands: "FF FF".to_owned(),
                repr: "$FFFF,Y".to_owned()
            },
            result
        );
    }
}
