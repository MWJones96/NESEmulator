/*
    BCC - Branch on Carry Clear
    Operation: Branch on C = 0

    This instruction tests the state of the carry bit and takes 
    a conditional branch if the carry bit is reset.

    It affects no flags or registers other than the program counter 
    and then only if the C flag is not on. 
*/

use super::super::CPU;

impl CPU {
    pub(in crate::cpu) fn bcc(offset: u8) -> u8 {
        0
    }
}

#[cfg(test)]
mod bcc_tests {
    use super::*;
}