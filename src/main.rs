use piston_window::*;

fn main() {
    let mut window: PistonWindow = WindowSettings::new("NES Emulator", [512, 480])
        .exit_on_esc(true)
        .build()
        .unwrap();
    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g, _device| {
            clear([0.0; 4], g);
        });
    }
}
