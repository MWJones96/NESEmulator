use std::fs::File;
use std::io;
use std::io::BufRead;

use nes_emu::bus::main_bus::MainBus;
use nes_emu::cpu::bus::CPUBus;
use nes_emu::cpu::CPULogger;
use nes_emu::cpu::CPU;
use nes_emu::mapper::mapper_factory;
use nes_emu::util::extract_header;
use nes_emu::util::extract_prg_rom;
use nes_emu::util::read_bytes_from_file;

#[test]
fn test_nes_logger_for_nestest_rom() {}

#[test]
fn test_nestest_rom() {
    let ref_log_file =
        io::BufReader::new(File::open("tests/fixtures/nestest_log.txt").unwrap()).lines();

    let mut bytes = read_bytes_from_file("tests/roms/nestest.nes".to_owned());
    bytes[16396] = 0x0;

    let header = extract_header(&bytes);
    let prg_rom = extract_prg_rom(&header, &bytes);

    let mapper = mapper_factory(header.mapper_num, prg_rom);
    let mut cpu = CPU::new();
    let mut main_bus = MainBus::new(&mapper);

    //Execute reset routine
    for _ in 0..7 {
        cpu.clock(&mut main_bus);
    }

    let logger = CPULogger;
    for line in ref_log_file {
        assert_eq!(
            line.unwrap(),
            logger.get_current_instruction_and_cpu_state(&cpu)
        );
    }

    assert_eq!(0x0, main_bus.read(0x2));
    assert_eq!(0x0, main_bus.read(0x3));
}
