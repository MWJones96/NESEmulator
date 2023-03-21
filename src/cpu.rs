mod ops;

pub struct CPU {
    pc: u32,
    a: u8,
    c: bool,
    z: bool,
    n: bool,
    v: bool
}

impl CPU {
    pub fn new() -> Self {
        Self {
            pc: 0,
            a: 0,
            c: false,
            z: true,
            n: false,
            v: false
        }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_cpu_initial_state() {
        let cpu = CPU::new();

        assert_eq!(0, cpu.pc);
        assert_eq!(0, cpu.a);

        assert_eq!(false, cpu.c);
        assert_eq!(true, cpu.z);
        assert_eq!(false, cpu.n);
        assert_eq!(false, cpu.v);
    }
}