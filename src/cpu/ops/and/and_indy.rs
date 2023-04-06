use super::super::super::CPU;

impl CPU {
    pub(super) fn and_indy(&mut self, addr: u8, mem: &[u8]) -> u8 {
        let low_byte_addr: u8 = mem[addr as usize];
        let high_byte_addr: u8 = mem[addr.wrapping_add(1) as usize];

        let page_before: u8 = high_byte_addr;

        let resolved_addr: u16 = ((high_byte_addr as u16) << 8) | (low_byte_addr as u16);
        let resolved_addr: u16 = resolved_addr.wrapping_add(self.y as u16);

        let page_after: u8 = (resolved_addr >> 8) as u8;

        if page_before == page_after {
            1 + self.and_abs(resolved_addr, mem)
        } else {
            2 + self.and_abs(resolved_addr, mem)
        }
    }
}

#[cfg(test)]
mod and_indy_tests {
    use super::*;

    #[test]
    fn test_and_indy_correct_cycles_no_page_cross() {
        let mut cpu = CPU::new();
        let mem = [0; 0x10000];

        assert_eq!(5, cpu.and_indy(0x0, &mem));
    }

    #[test]
    fn test_and_indy_correct_cycles_with_page_cross() {
        let mut cpu = CPU::new();
        let mut mem = [0; 0x10000];

        mem[0xff] = 0x77;
        mem[0x0] = 0x88;
        cpu.y = 0xff;

        assert_eq!(6, cpu.and_indy(0xff, &mem));
    }

    #[test]
    fn test_and_indy() {
        let mut cpu = CPU::new();
        let mut mem = [0; 0x10000];

        mem[0x0] = 0x77;
        mem[0x1] = 0x88;

        cpu.a = 0x3f;
        mem[0x8877] = 0xe0;

        cpu.and_indy(0x0, &mem);

        assert_eq!(0x20, cpu.a);
    }

    #[test]
    fn test_and_indy_zero_flag() {
        let mut cpu = CPU::new();
        let mut mem = [0; 0x10000];

        mem[0x0] = 0x77;
        mem[0x1] = 0x88;

        cpu.a = 0x3f;
        mem[0x8877] = 0xe0;

        cpu.and_indy(0x0, &mem);
        assert_eq!(false, cpu.z);

        mem[0x8877] = 0x0;
        cpu.and_indy(0x0, &mem);

        assert_eq!(true, cpu.z);
    } 

    #[test]
    fn test_and_indy_negative_flag() {
        let mut cpu = CPU::new();
        let mut mem = [0; 0x10000];

        mem[0x0] = 0x77;
        mem[0x1] = 0x88;

        cpu.a = 0xff;
        mem[0x8877] = 0xff;

        cpu.and_indy(0x0, &mem);
        assert_eq!(true, cpu.n);

        mem[0x8877] = 0x0;
        cpu.and_indy(0x0, &mem);
        assert_eq!(false, cpu.n);
    }
}