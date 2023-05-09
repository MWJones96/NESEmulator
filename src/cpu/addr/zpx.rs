/* 
    X-Indexed Zero Page

    This form of addressing is used in conjunction with the 
    X index register. The effective address is calculated by 
    adding the second byte to the contents of the index 
    register. Since this is a form of "Zero Page" addressing, 
    the content of the second byte references a location in 
    page zero. Additionally, due to the “Zero Page" addressing 
    nature of this mode, no carry is added to the high order 
    8 bits of memory and crossing of page boundaries does 
    not occur.

    Bytes: 2
*/

use crate::cpu::{bus::CPUBus, CPU};

use super::AddrModeResult;

impl CPU {
    pub(in crate::cpu) fn zpx(&self, addr: u8, bus: &dyn CPUBus) -> AddrModeResult {
        let resolved_addr = addr.wrapping_add(self.x) as u16;

        AddrModeResult {
            data: Some(bus.read(resolved_addr)),
            cycles: 2,
            mode: super::AddrMode::ZPX,
            addr: Some(resolved_addr),
        }
    }
}

#[cfg(test)]
mod zpx_tests {
    use mockall::predicate::eq;

    use super::*;
    use crate::cpu::{addr::AddrModeResult, bus::MockCPUBus};

    #[test]
    fn test_zpx_addressing_mode() {
        let mut cpu = CPU::new();
        let mut mock_bus = MockCPUBus::new();
        cpu.x = 0x2;

        mock_bus.expect_read().with(eq(0x1)).return_const(0x77);

        let result = cpu.zpx(0xff, &mock_bus);
        assert_eq!(
            AddrModeResult {
                data: Some(0x77),
                cycles: 2,
                mode: crate::cpu::addr::AddrMode::ZPX,
                addr: Some(0x1)
            },
            result
        );
    }
}
