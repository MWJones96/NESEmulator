use crate::cpu::{bus::CPUBus, CPU};

impl CPU {
    pub(in crate::cpu) fn irq_cycles(&self) -> u8 {
        0
    }

    pub(in crate::cpu) fn irq(&mut self, bus: &mut dyn CPUBus) {}
}

#[cfg(test)]
mod irq_tests {
    use super::*;
}
