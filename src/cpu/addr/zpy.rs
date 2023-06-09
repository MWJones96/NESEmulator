/*
    Y-Indexed Zero Page

    This form of addressing is used in conjunction with the
    Y index register. The effective address is calculated
    by adding the second byte to the contents of the index
    register. Since this is a form of "Zero Page" addressing,
    the content of the second byte references a location in
    page zero. Additionally, due to the “Zero Page" addressing
    nature of this mode, no carry is added to the high order
    8 bits of memory and crossing of page boundaries does
    not occur.

    Bytes: 2
*/

use crate::{bus::Bus, cpu::NESCPU};

use super::{AddrModeResult, AddrModeType};

impl NESCPU {
    pub(in crate::cpu) fn zpy(&mut self, bus: &dyn Bus) -> AddrModeResult {
        let addr = self.fetch_byte(bus);
        self._zpy(addr, bus)
    }

    pub(in crate::cpu) fn _zpy(&self, addr: u8, _bus: &dyn Bus) -> AddrModeResult {
        let resolved_addr = addr.wrapping_add(self.y) as u16;

        AddrModeResult {
            data: None,
            cycles: 2,
            mode: AddrModeType::Zpy,
            addr: Some(resolved_addr),
            bytes: 2,
            operands: format!("{:02X}", addr),
            repr: format!("${:02X},Y", addr),
        }
    }
}

#[cfg(test)]
mod zpy_tests {
    use mockall::predicate::eq;

    use super::*;
    use crate::{bus::MockBus, cpu::addr::AddrModeResult};

    #[test]
    fn test_zpy_addressing_mode() {
        let mut cpu = NESCPU::new();
        let mut mock_bus = MockBus::new();
        cpu.y = 0x2;

        mock_bus.expect_read().with(eq(0x1)).return_const(0x77);

        let result = cpu._zpy(0xff, &mock_bus);
        assert_eq!(
            AddrModeResult {
                data: None,
                cycles: 2,
                mode: AddrModeType::Zpy,
                addr: Some(0x1),
                bytes: 2,
                operands: "FF".to_owned(),
                repr: "$FF,Y".to_owned()
            },
            result
        );
    }
}
