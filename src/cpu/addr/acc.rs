use crate::cpu::CPU;

use super::AddrModeResult;

impl CPU {
    pub(in crate::cpu) fn acc(&self) -> AddrModeResult {
        AddrModeResult {
            data: self.a,
            cycles: 0,
            mode: super::AddrMode::ACC,
            addr: None
        }
    }
}

#[cfg(test)]
mod acc_tests {
    use crate::cpu::addr::AddrModeResult;

    use super::*;

    #[test]
    fn test_acc_addressing_mode() {
        let mut cpu = CPU::new();
        cpu.a = 0xcc;

        assert_eq!(AddrModeResult {
            data: 0xcc,
            cycles: 0,
            mode: crate::cpu::addr::AddrMode::ACC,
            addr: None
        }, cpu.acc());
    }
}