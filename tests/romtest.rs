use std::fs::File;
use std::io;
use std::io::BufRead;
use std::rc::Rc;

use nes_emu::bus::cpu_bus::CPUBus;
use nes_emu::bus::MockBus;
use nes_emu::cartridge::NESCartridge;
use nes_emu::cpu::CPU;
use nes_emu::cpu::NESCPU;
use nes_emu::mapper::mapper_factory;
use nes_emu::ppu::NESPPU;
use nes_emu::util::extract_chr_rom;
use nes_emu::util::extract_header;
use nes_emu::util::extract_prg_rom;
use nes_emu::util::read_bytes_from_file;

#[test]
fn test_nestest_rom() {
    let ref_log_file =
        io::BufReader::new(File::open("tests/logs/nestest_log.txt").unwrap()).lines();

    let mut bytes = read_bytes_from_file("tests/roms/nestest.nes".to_owned());
    bytes[16396] = 0x0;

    let header = extract_header(&bytes);
    let prg_rom = extract_prg_rom(&header, &bytes);
    let chr_rom = extract_chr_rom(&header, &bytes);

    let mapper = mapper_factory(header.mapper_num);
    let cartridge = NESCartridge::new(prg_rom, chr_rom, Box::new(mapper), header.mirroring);

    let mut cpu = NESCPU::new();
    let mut main_bus = CPUBus::new(
        Box::new(NESPPU::new(Box::new(MockBus::new()))),
        Rc::new(cartridge),
    );

    //Execute reset routine
    for _ in 0..7 {
        cpu.clock(&mut main_bus);
    }

    for line in ref_log_file {
        assert_eq!(line.unwrap(), cpu.to_string());
        for _ in 0..cpu.cycles_remaining() {
            cpu.clock(&mut main_bus);
        }
    }
}
