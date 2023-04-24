use crate::cpu::{bus::Bus, CPU};

use super::{AddrMode, AddrModeResult};

impl CPU {
    pub(in crate::cpu) fn ind(&self, addr: u16, bus: &dyn Bus) -> AddrModeResult {
        let low_byte = bus.read(addr) as u16;
        let high_byte =
            bus.read((addr & 0xff00) + ((addr & 0xff) as u8).wrapping_add(1) as u16) as u16;

        let resolved_addr = high_byte << 8 | low_byte;

        AddrModeResult {
            data: None,
            cycles: 4,
            mode: AddrMode::IND,
            addr: Some(resolved_addr),
        }
    }
}

#[cfg(test)]
mod ind_tests {
    use mockall::predicate::eq;

    use crate::cpu::bus::MockBus;

    use super::*;

    #[test]
    fn test_ind_addressing_mode() {
        let cpu = CPU::new();
        let mut bus = MockBus::new();

        bus.expect_read()
            .with(eq(0x0000))
            .times(1)
            .return_const(0x40);

        bus.expect_read()
            .with(eq(0x0001))
            .times(1)
            .return_const(0x20);

        let ind = cpu.ind(0x0000, &bus);
        assert_eq!(
            AddrModeResult {
                data: None,
                cycles: 4,
                mode: AddrMode::IND,
                addr: Some(0x2040)
            },
            ind
        );
    }

    #[test]
    fn test_ind_addressing_mode_hardware_bug() {
        let cpu = CPU::new();
        let mut bus = MockBus::new();

        bus.expect_read()
            .with(eq(0x80ff))
            .times(1)
            .return_const(0x40);

        bus.expect_read()
            .with(eq(0x8000))
            .times(1)
            .return_const(0x20);

        let ind = cpu.ind(0x80ff, &bus);
        assert_eq!(
            AddrModeResult {
                data: None,
                cycles: 4,
                mode: AddrMode::IND,
                addr: Some(0x2040)
            },
            ind
        );
    }
}
