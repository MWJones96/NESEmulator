use bus::main_bus::MainBus;
use cpu::CPU;
use util::read_bytes_from_file;

use crate::{
    mapper::{mapper_factory, PRGRomMapper},
    util::{extract_chr_rom, extract_header, extract_prg_rom},
};

mod bus;
mod cpu;
mod mapper;
mod util;

fn main() {
    let bytes = read_bytes_from_file("tests/roms/nestest.nes".to_owned());
    let header = extract_header(&bytes);
    let prg_rom = extract_prg_rom(&header, &bytes);
    let chr_rom = extract_chr_rom(&header, &bytes);

    let mapper = mapper_factory(header.mapper_num, prg_rom, chr_rom);
    let mut cpu = CPU::new();
    let mut main_bus = MainBus::new(&mapper);

    println!("{:02x?}", mapper.read(0xfffc));
    println!("{:02x?}", mapper.read(0xfffd));
}