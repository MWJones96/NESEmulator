use crate::cpu::CPU;

impl CPU {
    pub(in crate::cpu) fn imm(&self, imm: u8) -> (u8, u8) {
        (0, imm)
    }
}

#[cfg(test)]
mod imm_tests {
    use super::*;

    #[test]
    fn test_imm_addressing_mode() {
        let cpu = CPU::new();
        let imm = cpu.imm(0x88);
        assert_eq!((0, 0x88), imm);
    }
}