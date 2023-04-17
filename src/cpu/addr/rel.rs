use crate::cpu::CPU;

use super::AddrModeResult;

impl CPU {
    pub(in crate::cpu) fn rel(&self, offset: u8) -> AddrModeResult {
        let page_before = (self.pc >> 8) as u8;
        let resolved_addr = self.pc.wrapping_add((offset as i8) as u16);
        let page_after = (resolved_addr >> 8) as u8;

        AddrModeResult {
            data: None,
            cycles: (page_before != page_after) as u8,
            mode: super::AddrMode::REL,
            addr: Some(resolved_addr),
        }
    }
}

#[cfg(test)]
mod rel_tests {
    use super::*;

    #[test]
    fn test_rel_forward_one() {
        let mut cpu = CPU::new();

        cpu.pc = 0x0;
        let result = cpu.rel(0x1);
        assert_eq!(
            AddrModeResult {
                data: None,
                cycles: 0,
                mode: crate::cpu::addr::AddrMode::REL,
                addr: Some(0x1)
            },
            result
        );
    }

    #[test]
    fn test_rel_back_one() {
        let mut cpu = CPU::new();

        cpu.pc = 0x1234;
        let result = cpu.rel(0xff);
        assert_eq!(
            AddrModeResult {
                data: None,
                cycles: 0,
                mode: crate::cpu::addr::AddrMode::REL,
                addr: Some(0x1233)
            },
            result
        );
    }

    #[test]
    fn test_rel_forward_cross_page_boundary() {
        let mut cpu = CPU::new();

        cpu.pc = 0xffff;
        let result = cpu.rel(0x2);
        assert_eq!(
            AddrModeResult {
                data: None,
                cycles: 1,
                mode: crate::cpu::addr::AddrMode::REL,
                addr: Some(0x1)
            },
            result
        );
    }

    #[test]
    fn test_rel_backwards_cross_page_boundary() {
        let mut cpu = CPU::new();

        cpu.pc = 0x0;
        let result = cpu.rel(0xfe);
        assert_eq!(
            AddrModeResult {
                data: None,
                cycles: 1,
                mode: crate::cpu::addr::AddrMode::REL,
                addr: Some(0xfffe)
            },
            result
        );
    }
}
