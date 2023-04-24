use crate::cpu::CPU;

use super::{AddrMode, AddrModeResult};

impl CPU {
    fn imp(&self) -> AddrModeResult {
        AddrModeResult {
            data: None,
            cycles: 0,
            mode: AddrMode::IMP,
            addr: None,
        }
    }
}

#[cfg(test)]
mod imp_tests {
    use super::*;

    #[test]
    fn test_imp_addressing_mode() {
        let cpu = CPU::new();
        let imp = cpu.imp();

        assert_eq!(
            AddrModeResult {
                data: None,
                cycles: 0,
                mode: AddrMode::IMP,
                addr: None
            },
            imp
        );
    }
}
