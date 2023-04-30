use self::{addr::AddrModeResult, bus::Bus};

mod addr;
mod bus;
mod ops;

pub struct CPU {
    pc: u16,
    sp: u8,

    a: u8,
    x: u8,
    y: u8,

    n: bool, //Bit 7
    v: bool, //Bit 6
    //Bit 5 (unused)
    b: bool, //Bit 4
    d: bool, //Bit 3
    i: bool, //Bit 2
    z: bool, //Bit 1
    c: bool, //Bit 0
}

impl CPU {
    const INTERRUPT_VECTOR: u16 = 0xfffe;

    pub fn new() -> Self {
        Self {
            pc: 0,
            sp: 0xff,

            a: 0,
            x: 0,
            y: 0,

            n: false, //Bit 7
            v: false, //Bit 6
            //Bit 5 unused (always 1)
            b: false, //Bit 4
            d: false, //Bit 3
            i: false, //Bit 2
            z: false, //Bit 1
            c: false, //Bit 0
        }
    }

    fn execute(&mut self, opcode: u8, mode: &AddrModeResult, bus: &dyn Bus) {
        match opcode {
            _ => panic!("Opcode {} is unimplemented", opcode)
        }
    }

    fn get_status_byte(&self) -> u8 {
        (self.n as u8) << 7
            | (self.v as u8) << 6
            | 0x1 << 5
            | (self.b as u8) << 4
            | (self.d as u8) << 3
            | (self.i as u8) << 2
            | (self.z as u8) << 1
            | (self.c as u8) << 0
    }
}

#[cfg(test)]
mod cpu_tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_cpu_initial_state() {
        let cpu = CPU::new();

        assert_eq!(0, cpu.pc);
        assert_eq!(0xff, cpu.sp);

        assert_eq!(0, cpu.a);
        assert_eq!(0, cpu.x);
        assert_eq!(0, cpu.y);

        assert_eq!(false, cpu.n); //Bit 7
        assert_eq!(false, cpu.v); //Bit 6
                                  //Bit 5 (unused)
        assert_eq!(false, cpu.b); //Bit 4
        assert_eq!(false, cpu.d); //Bit 3
        assert_eq!(false, cpu.i); //Bit 2
        assert_eq!(false, cpu.z); //Bit 1
        assert_eq!(false, cpu.c); //Bit 0
    }

    #[test]
    fn test_get_status_byte_no_flags() {
        let cpu = CPU::new();
        assert_eq!(0b0010_0000, cpu.get_status_byte())
    }

    #[test]
    fn test_get_status_byte_negative_flag() {
        let mut cpu = CPU::new();
        cpu.n = true;
        assert_eq!(0b1010_0000, cpu.get_status_byte())
    }

    #[test]
    fn test_get_status_byte_overflow_flag() {
        let mut cpu = CPU::new();
        cpu.v = true;
        assert_eq!(0b0110_0000, cpu.get_status_byte())
    }

    #[test]
    fn test_get_status_byte_break_flag() {
        let mut cpu = CPU::new();
        cpu.b = true;
        assert_eq!(0b0011_0000, cpu.get_status_byte())
    }

    #[test]
    fn test_get_status_byte_decimal_flag() {
        let mut cpu = CPU::new();
        cpu.d = true;
        assert_eq!(0b0010_1000, cpu.get_status_byte())
    }

    #[test]
    fn test_get_status_byte_interrupt_flag() {
        let mut cpu = CPU::new();
        cpu.i = true;
        assert_eq!(0b0010_0100, cpu.get_status_byte())
    }

    #[test]
    fn test_get_status_byte_zero_flag() {
        let mut cpu = CPU::new();
        cpu.z = true;
        assert_eq!(0b0010_0010, cpu.get_status_byte())
    }

    #[test]
    fn test_get_status_byte_carry_flag() {
        let mut cpu = CPU::new();
        cpu.c = true;
        assert_eq!(0b0010_0001, cpu.get_status_byte())
    }

    #[test]
    fn test_get_status_byte_all_flags() {
        let mut cpu = CPU::new();
        cpu.n = true;
        cpu.v = true;
        cpu.b = true;
        cpu.d = true;
        cpu.i = true;
        cpu.z = true;
        cpu.c = true;

        assert_eq!(0b1111_1111, cpu.get_status_byte())
    }
}

#[cfg(test)]
mod execute_tests {
    use super::{*, bus::MockBus};

    #[test]
    #[should_panic]
    fn test_panic_on_invalid_opcode() {
        let mut cpu = CPU::new();
        let bus = MockBus::new();

        cpu.execute(0xff, &cpu.imp(), &bus)
    }
}
