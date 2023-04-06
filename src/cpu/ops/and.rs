/*
    AND - "AND" Memory with Accumulator

    Operation: A ∧ M → A

    The AND instruction transfer the accumulator and memory to the adder 
    which performs a bit-by-bit AND operation and stores the result back 
    in the accumulator.

    This instruction affects the accumulator; sets the zero flag if the 
    result in the accumulator is 0, otherwise resets the zero flag; 
    sets the negative flag if the result in the accumulator has bit 7 on, 
    otherwise resets the negative flag.
*/

use super::super::CPU;

mod and_imm;
mod and_zp;
mod and_zpx;
mod and_abs;
mod and_absx;
mod and_absy;
mod and_indx;
mod and_indy;

fn _and_abs_helper(cpu: &mut CPU, addr: u16, mem: &[u8], reg: u8) -> u8 {
    let page_before: u8 = (addr >> 8) as u8;
    let resolved_addr: u16 = addr.wrapping_add(reg as u16);
    let page_after: u8 = (resolved_addr >> 8) as u8;

    if page_before == page_after {
        cpu.and_abs(resolved_addr, mem) 
    } else {
        1 + cpu.and_abs(resolved_addr, mem) 
    }
}