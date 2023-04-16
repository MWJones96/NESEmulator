use crate::cpu::{CPU, bus::Bus};

impl CPU {
    pub(in crate::cpu) fn acc(&self) -> (u8, u8) {
        (0, self.a)
    }
}

#[cfg(test)]
mod acc_tests {
    use super::*;

    #[test]
    fn test_acc_addressing_mode() {
        let mut cpu = CPU::new();
        cpu.a = 0xcc;

        assert_eq!((0, 0xcc), cpu.acc());
    }
}