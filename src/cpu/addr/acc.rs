/*
    Accumulator

    This form of addressing is represented with a one byte
    instruction, implying an operation on the accumulator.

    Bytes: 1
*/

use crate::cpu::CPU;

use super::AddrModeResult;

impl CPU {
    pub(in crate::cpu) fn acc(&self) -> AddrModeResult {
        AddrModeResult {
            data: Some(self.a),
            cycles: 0,
            mode: super::AddrMode::ACC,
            addr: None,
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

        assert_eq!(
            AddrModeResult {
                data: Some(0xcc),
                cycles: 0,
                mode: crate::cpu::addr::AddrMode::ACC,
                addr: None
            },
            cpu.acc()
        );
    }
}
