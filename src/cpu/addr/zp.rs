/*
    Zero Page

    The zero page instructions allow for shorter code and
    execution times by only fetching the second byte of the
    instruction and assuming a zero high address byte.
    Careful use of the zero page can result in significant
    increase in code efficiency.

    Bytes: 2
*/

use crate::cpu::{bus::CPUBus, CPU};

use super::{AddrModeResult, AddrModeType};

impl CPU {
    #[inline]
    pub(in crate::cpu) fn zp(&self, addr: u8, bus: &impl CPUBus) -> AddrModeResult {
        AddrModeResult {
            data: Some(bus.read(addr as u16)),
            cycles: 1,
            mode: AddrModeType::ZP,
            addr: Some(addr as u16),
            bytes: 2,
        }
    }
}

#[cfg(test)]
mod zp_tests {
    use mockall::predicate::eq;

    use crate::cpu::{addr::AddrModeResult, bus::MockCPUBus};

    use super::*;

    #[test]
    fn test_zp_addressing_mode() {
        let cpu = CPU::new();
        let mut mock_bus = MockCPUBus::new();
        mock_bus.expect_read().with(eq(0x0)).return_const(0x77);

        let result = cpu.zp(0x0, &mock_bus);
        assert_eq!(
            AddrModeResult {
                data: Some(0x77),
                cycles: 1,
                mode: AddrModeType::ZP,
                addr: Some(0x0),
                bytes: 2,
            },
            result
        );
    }
}
