use super::{addr::AddrModeResult, CPU};

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
mod brk;
mod bvc;
mod bvs;
mod clc;
mod cld;
mod cli;
mod clv;
mod cmp;
mod cpx;
mod cpy;
mod dec;
mod dex;
mod dey;
mod eor;
