use bus::main_bus::MainBus;
use cpu::CPU;

mod bus;
mod cpu;

fn main() {
    let mut cpu = CPU::new();
    cpu.system_reset();
    let mut bus = MainBus::new();

    cpu.clock(&mut bus);
}
