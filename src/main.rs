use bus::main_bus::MainBus;
use cpu::CPU;

mod bus;
mod cpu;

pub fn get_bytes_from_rom_file(path: String) -> Vec<u8> {
    Vec::new()
}

fn main() {
    let mut cpu = CPU::new();
    cpu.system_reset();
    let mut bus = MainBus::new();

    cpu.clock(&mut bus);
}
