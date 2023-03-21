pub mod cpu {
    pub struct CPU {
        pc: u32
    }

    impl CPU {
        pub fn new() -> Self {
            Self {
                pc: 0
            }
        }
    }

    mod adc {
        use super::CPU;

        impl CPU {
            pub fn adc_imm(&self) {
                println!("{}", self.pc.to_string())
            }

            pub fn adc_zp(&self) {

            }

            pub fn adc_zpx(&self) {

            }

            pub fn adc_abs(&self) {

            }

            pub fn adc_absx(&self) {

            }

            pub fn adc_absy(&self) {

            }

            pub fn adc_indx(&self) {

            }

            pub fn adc_indy(&self) {

            }
        }
    }
}

fn main() {
    let x = cpu::CPU::new();
    x.adc_imm();
}
