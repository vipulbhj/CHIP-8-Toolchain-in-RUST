mod chip;

use chip::Chip8;
use clap::Parser;
use raylib::prelude::*;

const SCALE: usize = 10;
const WIDTH: usize = 64;
const HEIGHT: usize = 32;
const OPCODES_PER_FRAME: usize = 10;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path of ROM to be loaded
    rom_path: String,
}

fn handle_input(chip: &mut Chip8, rl: &RaylibHandle) {
    let key_map = [
        (KeyboardKey::KEY_X, 0x0),
        (KeyboardKey::KEY_ONE, 0x1),
        (KeyboardKey::KEY_TWO, 0x2),
        (KeyboardKey::KEY_THREE, 0x3),
        (KeyboardKey::KEY_Q, 0x4),
        (KeyboardKey::KEY_W, 0x5),
        (KeyboardKey::KEY_E, 0x6),
        (KeyboardKey::KEY_A, 0x7),
        (KeyboardKey::KEY_S, 0x8),
        (KeyboardKey::KEY_D, 0x9),
        (KeyboardKey::KEY_Z, 0xA),
        (KeyboardKey::KEY_C, 0xB),
        (KeyboardKey::KEY_FOUR, 0xC),
        (KeyboardKey::KEY_R, 0xD),
        (KeyboardKey::KEY_F, 0xE),
        (KeyboardKey::KEY_V, 0xF),
    ];

    for (key, chip8_key) in key_map {
        if rl.is_key_down(key) {
            chip.set_key(chip8_key);
        } else {
            chip.unset_key(chip8_key);
        }
    }
}

fn main() {
    let args = Args::parse();

    let mut chip8 = Chip8::new();
    chip8.load_rom(&args.rom_path);

    let (mut rl, thread) = raylib::init()
        .size((WIDTH * SCALE) as i32, (HEIGHT * SCALE) as i32)
        .title("CHIP-8 Emulator")
        .build();

    rl.set_target_fps(60);

    while !rl.window_should_close() {
        handle_input(&mut chip8, &rl);

        // --- update ---
        for _ in 0..OPCODES_PER_FRAME {
            let opcode = chip8.fetch();
            chip8.execute(opcode);
        }

        // tick timers once per frame
        if chip8.dt > 0 {
            chip8.dt -= 1;
        }
        if chip8.st > 0 {
            chip8.st -= 1;
        }

        // --- render ---
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                if chip8.display[y * WIDTH + x] {
                    d.draw_rectangle(
                        (x * SCALE) as i32,
                        (y * SCALE) as i32,
                        SCALE as i32,
                        SCALE as i32,
                        Color::WHITE,
                    );
                }
            }
        }
    }
}
