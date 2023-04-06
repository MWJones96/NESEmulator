use super::super::super::CPU;

impl CPU {
    pub(super) fn and_indx(&mut self, addr: u8, mem: &[u8]) -> u8 {
        let low_byte_addr: u8 = addr.wrapping_add(self.x);
        let high_byte_addr: u8 = low_byte_addr.wrapping_add(1);

        let resolved_addr: u16 = ((mem[high_byte_addr as usize] as u16) << 8) 
            | mem[low_byte_addr as usize] as u16;
        
        2 + self.and_abs(resolved_addr, mem)
    }
}

#[cfg(test)]
mod and_indx_tests {
    use super::*;

    #[test]
    fn test_and_indx_correct_cycles() {
        let mut cpu = CPU::new();
        let mem = [0; 0x10000];

        assert_eq!(6, cpu.and_indx(0x0, &mem));
    }

    #[test]
    fn test_and_indx() {
        let mut cpu = CPU::new();
        let mut mem = [0; 0x10000];

        cpu.x = 0x1;
        cpu.a = 0x11;

        mem[0x0] = 0x77;
        mem[0x1] = 0x88;

        mem[0x8877] = 0xee;

        cpu.and_indx(0xff, &mem);

        assert_eq!(0x0, cpu.a);
    }

    #[test]
    fn test_and_indx_zero_flag() {
        let mut cpu = CPU::new();
        let mut mem = [0; 0x10000];

        cpu.x = 0x1;
        cpu.a = 0xff;

        mem[0x0] = 0x77;
        mem[0x1] = 0x88;

        mem[0x8877] = 0x1;

        cpu.and_indx(0xff, &mem);
        assert_eq!(false, cpu.z);

        mem[0x8877] = 0x0;

        cpu.and_indx(0xff, &mem);
        assert_eq!(true, cpu.z);
    }

    #[test]
    fn test_and_indx_negative_flag() {
        let mut cpu = CPU::new();
        let mut mem = [0; 0x10000];

        cpu.x = 0x1;
        mem[0x0] = 0x77;
        mem[0x1] = 0x88;

        cpu.a = 0xff;
        mem[0x8877] = 0x80;

        cpu.and_indx(0xff, &mem);

        assert_eq!(true, cpu.n);

        mem[0x8877] = 0x0;
        cpu.and_indx(0xff, &mem);

        assert_eq!(false, cpu.n);
    }
}