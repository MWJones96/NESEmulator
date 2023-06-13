use mockall::automock;

use crate::{mapper::Mapper, util::Mirroring};

#[automock]
pub trait Cartridge {
    fn cpu_read(&self, addr: u16) -> u8;
    fn cpu_write(&self, addr: u16, data: u8);
    fn ppu_read(&self, addr: u16) -> u8;
    fn ppu_write(&self, addr: u16, data: u8);

    fn get_mirroring(&self) -> Mirroring;
}

pub struct NESCartridge<'a> {
    prg_rom: &'a [u8],
    chr_rom: &'a [u8],

    prg_rom_banks: u8,
    chr_rom_banks: u8,

    mapper: Box<dyn Mapper + 'a>,
    mirroring: Mirroring,
}

impl<'a> NESCartridge<'a> {
    const BYTES_PER_PRG_BANK: u32 = 16384;
    const BYTES_PER_CHR_BANK: u32 = 8192;

    pub fn new(
        prg_rom: &'a [u8],
        chr_rom: &'a [u8],
        mapper: Box<dyn Mapper + 'a>,
        mirroring: Mirroring,
    ) -> Self {
        Self {
            prg_rom,
            chr_rom,

            prg_rom_banks: (prg_rom.len() as u32 / NESCartridge::BYTES_PER_PRG_BANK) as u8,
            chr_rom_banks: (chr_rom.len() as u32 / NESCartridge::BYTES_PER_CHR_BANK) as u8,

            mapper,
            mirroring,
        }
    }
}

impl Cartridge for NESCartridge<'_> {
    fn cpu_read(&self, addr: u16) -> u8 {
        assert!((0x8000..=0xffff).contains(&addr));
        self.prg_rom[self.mapper.read_prg(addr, self.prg_rom_banks) as usize]
    }

    fn cpu_write(&self, addr: u16, data: u8) {
        assert!((0x8000..=0xffff).contains(&addr));
        self.mapper.write_prg(addr, data, self.prg_rom_banks);
    }

    fn ppu_read(&self, addr: u16) -> u8 {
        assert!((..=0x1fff).contains(&addr));
        self.chr_rom[self.mapper.read_chr(addr, self.chr_rom_banks) as usize]
    }

    fn ppu_write(&self, addr: u16, _data: u8) {
        assert!((..=0x1fff).contains(&addr));
    }

    fn get_mirroring(&self) -> Mirroring {
        self.mirroring
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
            .expect_read_prg()
            .with(eq(0x8000), eq(1))
            .once()
            .return_const(0x0 as u16);

        let cartridge = NESCartridge::new(
            &[0; NESCartridge::BYTES_PER_PRG_BANK as usize],
            &[0; NESCartridge::BYTES_PER_CHR_BANK as usize],
            Box::new(mapper),
            Mirroring::HORIZONTAL,
        );

        cartridge.cpu_read(0x8000);
    }

    #[test]
    fn test_cartridge_write_from_cpu() {
        let mut mapper = MockMapper::new();

        mapper
            .expect_write_prg()
            .with(eq(0x8000), eq(0x0), eq(1))
            .once()
            .return_const(());

        let cartridge = NESCartridge::new(
            &[0; NESCartridge::BYTES_PER_PRG_BANK as usize],
            &[0; NESCartridge::BYTES_PER_CHR_BANK as usize],
            Box::new(mapper),
            Mirroring::HORIZONTAL,
        );

        cartridge.cpu_write(0x8000, 0x0);
    }

    #[test]
    fn test_cartridge_read_from_ppu() {
        let mut mapper = MockMapper::new();

        let mut chr_rom = [0; NESCartridge::BYTES_PER_CHR_BANK as usize];
        chr_rom[0x1234] = 0xff;

        mapper
            .expect_read_chr()
            .with(eq(0x1234), eq(1))
            .once()
            .return_const(0x1234u16);

        let cartridge = NESCartridge::new(
            &[0; NESCartridge::BYTES_PER_PRG_BANK as usize],
            &chr_rom,
            Box::new(mapper),
            Mirroring::HORIZONTAL,
        );

        assert_eq!(0xff, cartridge.ppu_read(0x1234));
    }
}
