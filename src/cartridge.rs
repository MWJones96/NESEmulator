use mockall::automock;

use crate::mapper::Mapper;

#[automock]
pub trait Cartridge {
    fn read(&self, addr: u16) -> u8;
    fn write(&self, addr: u16, data: u8);
}

pub struct NESCartridge<'a> {
    prg_rom: &'a [u8],
    mapper: Box<dyn Mapper + 'a>,
    prg_rom_banks: u8,
}

impl<'a> NESCartridge<'a> {
    const BYTES_PER_PRG_BANK: u32 = 16384;

    pub fn new(prg_rom: &'a [u8], mapper: Box<dyn Mapper + 'a>) -> Self {
        Self {
            prg_rom,
            mapper,
            prg_rom_banks: (prg_rom.len() as u32 / NESCartridge::BYTES_PER_PRG_BANK) as u8,
        }
    }
}

impl Cartridge for NESCartridge<'_> {
    fn read(&self, addr: u16) -> u8 {
        assert!((0x8000..=0xffff).contains(&addr));
        self.prg_rom[self.mapper.read(addr, self.prg_rom_banks) as usize]
    }

    fn write(&self, addr: u16, data: u8) {
        assert!((0x8000..=0xffff).contains(&addr));
        self.mapper.write(addr, data, self.prg_rom_banks);
    }
}

#[cfg(test)]
mod cartridge_tests {
    use crate::mapper::MockMapper;
    use mockall::predicate::eq;

    use super::*;

    #[test]
    fn test_cartridge_read_from_cpu() {
        let mut mapper = MockMapper::new();

        mapper
            .expect_read()
            .with(eq(0x8000), eq(1))
            .once()
            .return_const(0x0 as u16);

        let cartridge = NESCartridge::new(
            &[0; NESCartridge::BYTES_PER_PRG_BANK as usize],
            Box::new(mapper),
        );

        cartridge.read(0x8000);
    }

    #[test]
    fn test_cartridge_write_from_cpu() {
        let mut mapper = MockMapper::new();

        mapper
            .expect_write()
            .with(eq(0x8000), eq(0x0), eq(1))
            .once()
            .return_const(());

        let cartridge = NESCartridge::new(
            &[0; NESCartridge::BYTES_PER_PRG_BANK as usize],
            Box::new(mapper),
        );

        cartridge.write(0x8000, 0x0);
    }
}
