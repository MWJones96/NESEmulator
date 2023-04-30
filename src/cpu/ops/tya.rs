use crate::cpu::addr::AddrModeResult;

use super::super::CPU;

impl CPU {
    pub(in crate::cpu) fn tya_cycles(&self, _mode: &AddrModeResult) -> u8 {
        2
    }

    pub(in crate::cpu) fn tya(&mut self, _mode: &AddrModeResult) {
        self.a = self.y;

        self.n = (self.a & 0x80) > 0;
        self.z = self.a == 0;
    }
}

#[cfg(test)]
mod tya_tests {
    use super::*;

    #[test]
    fn test_tya_returns_correct_number_of_cycles() {
        let cpu = CPU::new();
        assert_eq!(2, cpu.tya_cycles(&cpu.imp()));
    }
    
    #[test]
    fn test_tya() {
        let mut cpu = CPU::new();
        cpu.y = 0xcc;

        cpu.tya(&cpu.imp());
        assert_eq!(0xcc, cpu.a);
        assert_eq!(0xcc, cpu.y);
    }
    
    #[test]
    fn test_tya_negative_flag() {
        let mut cpu = CPU::new();
        cpu.y = 0x80;

        cpu.tya(&cpu.imp());
        assert_eq!(true, cpu.n);
    }

    #[test]
    fn test_tya_zero_flag() {
        let mut cpu = CPU::new();
        cpu.a = 0xff;

        cpu.tya(&cpu.imp());
        assert_eq!(true, cpu.z);
    }
}