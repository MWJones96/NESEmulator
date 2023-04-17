/*
    BCS - Branch on Carry Set
    Operation: Branch on C = 1

    This instruction takes the conditional branch if the carry flag is on.

    BCS does not affect any of the flags or registers except for the program 
    counter and only then if the carry flag is on.
 */

 use crate::cpu::addr::AddrModeResult;

 use super::super::CPU;
 
 impl CPU {
     pub(in crate::cpu) fn bcs(&mut self, mode: &AddrModeResult) -> u8 {
         match self.c {
             false => {
                 2 + mode.cycles
             },
             true => {
                 self.pc = mode.addr.unwrap();
                 2 + 1 + mode.cycles
             }
         }
     }
 }
 
 #[cfg(test)]
 mod bcs_tests {
     use super::*;
 
     #[test]
     fn test_bcs_no_branch_no_page_cross() {
         let mut cpu = CPU::new();
 
         cpu.pc = 0x1234;
         cpu.c = false;
         assert_eq!(2, cpu.bcs(&cpu.rel(0x1)));
         assert_eq!(0x1234, cpu.pc);
     }
 
     #[test]
     fn test_bcs_no_branch_with_page_cross() {
         let mut cpu = CPU::new();
 
         cpu.pc = 0x12ff;
         cpu.c = false;
         assert_eq!(3, cpu.bcs(&cpu.rel(0xa)));
         assert_eq!(0x12ff, cpu.pc);
     }
 
     #[test]
     fn test_bcs_with_branch_no_page_cross() {
         let mut cpu = CPU::new();
         cpu.pc = 0x81;
         cpu.c = true;
 
         assert_eq!(3, cpu.bcs(&cpu.rel(0x80)));
         assert_eq!(0x1, cpu.pc);
     }
 
     #[test]
     fn test_bcs_with_branch_and_page_cross() {
         let mut cpu = CPU::new();
         cpu.pc = 0x8081;
         cpu.c = true;
 
         assert_eq!(4, cpu.bcs(&cpu.rel(0x7f)));
         assert_eq!(0x8100, cpu.pc);
     }
 }