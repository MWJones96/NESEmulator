use super::{addr::AddrModeResult, CPU};

mod adc;
mod and;
mod asl;
mod bcc;
mod bcs;
mod beq;
mod bit;
mod bmi;
mod bne;
mod bpl;

impl CPU {
    fn branch_helper(&mut self, condition: bool, mode: &AddrModeResult) -> u8 {
        match condition {
            true => {
                self.pc = mode.addr.unwrap();
                2 + 1 + mode.cycles
            }
            false => 2 + mode.cycles,
        }
    }
}
