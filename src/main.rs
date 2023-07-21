use minifb::{Icon, Key, Window, WindowOptions};

use nes_emu::{
    bus::{cpu_bus::CPUBus, ppu_bus::PPUBus},
    cartridge::NESCartridge,
    cpu::NESCPU,
    mapper::mapper_factory,
    ppu::NESPPU,
    util::{extract_chr_rom, extract_header, extract_prg_rom, read_bytes_from_file},
};
use std::{
    rc::Rc,
    str::FromStr,
    time::{Duration, Instant},
};

#[rustfmt::skip]
const SCREEN_COLORS: [(u8, u8, u8); 0x40] = [
    ( 84,  84,  84), (  0,  30, 116), (  8,  16, 144), ( 48,   0, 136), ( 68,   0, 100), ( 92,   0,  48), ( 84,   4,   0), ( 60,  24,   0),
    ( 32,  42,   0), (  8,  58,   0), (  0,  64,   0), (  0,  60,   0), (  0,  50,  60), (  0,   0,   0), (  0,   0,   0), (  0,   0,   0),
    (152, 150, 152), (  8,  76, 196), ( 48,  50, 236), ( 92,  30, 228), (136,  20, 176), (160,  20, 100), (152,  34,  32), (120,  60,   0),
    ( 84,  90,   0), ( 40, 114,   0), (  8, 124,   0), (  0, 118,  40), (  0, 102, 120), (  0,   0,   0), (  0,   0,   0), (  0,   0,   0),
    (236, 238, 236), ( 76, 154, 236), (120, 124, 236), (176,  98, 236), (228,  84, 236), (236,  88, 180), (236, 106, 100), (212, 136,  32),
    (160, 170,   0), (116, 196,   0), ( 76, 208,  32), ( 56, 204, 108), ( 56, 180, 204), ( 60,  60,  60), (  0,   0,   0), (  0,   0,   0),
    (236, 238, 236), (168, 204, 236), (188, 188, 236), (212, 178, 236), (236, 174, 236), (236, 174, 212), (236, 180, 176), (228, 196, 144),
    (204, 210, 120), (180, 222, 120), (168, 226, 144), (152, 226, 180), (160, 214, 228), (160, 162, 160), (  0,   0,   0), (  0,   0,   0),
];

fn main() {
    let bytes = read_bytes_from_file("roms/dk.nes".to_owned());

    let header = extract_header(&bytes);
    let prg_rom = extract_prg_rom(&header, &bytes);
    let chr_rom = extract_chr_rom(&header, &bytes);

    let mapper = mapper_factory(header.mapper_num);

    let cartridge_cpu = Rc::new(NESCartridge::new(
        prg_rom,
        chr_rom,
        Box::new(mapper),
        header.mirroring,
    ));
    let cartridge_ppu = Rc::clone(&cartridge_cpu);

    let mut cpu = NESCPU::new();
    let ppu = NESPPU::new(Box::new(PPUBus::new(cartridge_ppu)));
    let mut main_bus = CPUBus::new(Box::new(ppu), cartridge_cpu);

    let mut window = Window::new("NES Emulator", 512, 480, WindowOptions::default())
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });
    window.set_icon(Icon::from_str("res/icon.ico").unwrap());

    let frame_duration = Duration::new(0, 16_666_600);
    while window.is_open() && !window.is_key_down(Key::Escape) {
        update_screen_buffer(&main_bus, &mut window);

        //Update controller input
        update_controller_input(&window);

        let start = Instant::now();

        //Emulate one frame's worth of cycles
        while !main_bus.is_frame_completed() {
            main_bus.clock(&mut cpu);
        }

        while start.elapsed() < frame_duration {}
    }
}

fn update_controller_input(window: &Window) {
    if window.is_key_down(Key::W) {
        println!("Up pressed");
    }
    if window.is_key_down(Key::A) {
        println!("Left pressed");
    }
    if window.is_key_down(Key::S) {
        println!("Down pressed");
    }
    if window.is_key_down(Key::D) {
        println!("Right pressed");
    }
    if window.is_key_down(Key::K) {
        println!("B pressed");
    }
    if window.is_key_down(Key::L) {
        println!("A pressed");
    }
    if window.is_key_down(Key::I) {
        println!("Start pressed");
    }
    if window.is_key_down(Key::O) {
        println!("Select pressed");
    }
}

#[inline]
fn update_screen_buffer(main_bus: &CPUBus<'_>, window: &mut Window) {
    let buffer: Vec<u32> = main_bus
        .get_frame_from_ppu()
        .iter()
        .flatten()
        .map(|&value| {
            let rgb = SCREEN_COLORS[value as usize];
            (rgb.0 as u32) << 16 | (rgb.1 as u32) << 8 | rgb.2 as u32
        })
        .collect();

    window.update_with_buffer(&buffer, 256, 240).unwrap();
}
