use bus::main_bus::MainBus;
use cpu::CPU;
use util::read_bytes_from_file;

use crate::{util::{extract_header, extract_prg_rom, extract_chr_rom}, mapper::{mapper_factory, PRGRomMapper}};

mod bus;
mod cpu;
mod util;
mod mapper;

fn main() {
    let bytes = read_bytes_from_file("tests/roms/nestest.nes".to_owned());
    let header = extract_header(&bytes);
    let prg_rom = extract_prg_rom(&header, &bytes);
    let chr_rom = extract_chr_rom(&header, &bytes);

    let mut mapper = mapper_factory(header.mapper_num, prg_rom, chr_rom);
    PRGRomMapper::write(mapper.as_mut(), 0x0, 0x0);

    println!("{:?}", chr_rom);
}
