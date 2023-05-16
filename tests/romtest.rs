use nes_emu::bus::main_bus::MainBus;
use nes_emu::cpu::CPU;
use nes_emu::cpu::bus::CPUBus;
use nes_emu::mapper::mapper_factory;
use nes_emu::util::extract_chr_rom;
use nes_emu::util::extract_header;
use nes_emu::util::extract_prg_rom;
use nes_emu::util::read_bytes_from_file;

#[test]
fn test_nestest_rom() {
    let mut bytes = read_bytes_from_file("tests/roms/nestest.nes".to_owned());
    bytes[16396] = 0x0;

    let header = extract_header(&bytes);
    let prg_rom = extract_prg_rom(&header, &bytes);
    let chr_rom = extract_chr_rom(&header, &bytes);

    let mapper = mapper_factory(header.mapper_num, prg_rom, chr_rom);
    let mut cpu = CPU::new();
    let mut main_bus = MainBus::new(&mapper);

    for _ in 0..=26_560 {
        main_bus.clock(&mut cpu);
    }

    assert_eq!(0x0, main_bus.read(0x2));
    assert_eq!(0x0, main_bus.read(0x3));
}