/*
    ADC - Add Memory to Accumulator with Carry

    Operation: A + M + C → A, C

    This instruction adds the value of memory and carry from the previous operation to the value
    of the accumulator and stores the result in the accumulator.

    This instruction affects the accumulator; sets the carry flag when the sum of a binary add 
    exceeds 255 or when the sum of a decimal add exceeds 99, otherwise carry is reset. 

    The overflow flag is set when the sign or bit 7 is changed due to the result exceeding 
    +127 or -128, otherwise overflow is reset. 

    The negative flag is set if the accumulator result contains bit 7 on, otherwise the 
    negative flag is reset. 
    
    The zero flag is set if the accumulator result is 0, otherwise the zero flag is reset.
*/

use super::super::CPU;
use super::super::bus::Bus;

mod adc_imm;
mod adc_zp;
mod adc_zpx;
mod adc_abs;
mod adc_absx;
mod adc_absy;
mod adc_indx;
mod adc_indy;

fn _adc_abs_helper(cpu: &mut CPU, addr: u16, bus: &dyn Bus, register: u8) -> u8 {
    let page_before: u8 = (addr >> 8) as u8;
    let resolved_addr = addr.wrapping_add(register as u16);
    let page_after: u8 = (resolved_addr >> 8) as u8;
    
    if page_before == page_after { 
        cpu.adc_abs(resolved_addr, bus)
    } 
    else {
        1 + cpu.adc_abs(resolved_addr, bus)
    }
}
