extern crate rand;
extern crate termion;
extern crate linux_raw_input_rs;
extern crate sdl2;
extern crate clap;

use std::{thread, time};

use clap::{Arg, App};

mod chip8;
use chip8::{MachineState};

mod renderers;
use renderers::{Renderer, get_renders};
use renderers::input::{KeyboardCommand};

static ARG_GAME: &str = "arg_game";
static ARG_RENDERER: &str = "arg_renderer";
static KEYMAPPING: &str = "
Key mappings:

 Keypad                   Keyboard
+-+-+-+-+                +-+-+-+-+
|1|2|3|C|                |1|2|3|4|
+-+-+-+-+                +-+-+-+-+
|4|5|6|D|                |Q|W|E|R|
+-+-+-+-+       =>       +-+-+-+-+
|7|8|9|E|                |A|S|D|F|
+-+-+-+-+                +-+-+-+-+
|A|0|B|F|                |Z|X|C|V|
+-+-+-+-+                +-+-+-+-+
";

fn main() {
    // Argument parsing
    let matches = App::new("Chip 8 Emu")
        .version("1.0")
        .author("Nicolás Antinori <nicolas.antinori.7@gmail.com>")
        .about(KEYMAPPING)
        .arg(Arg::with_name(ARG_GAME)
            .short("g")
            .long("game")
            .value_name("FILE")
            .help("Path to the game")
            .required(true)
            .takes_value(true))
        .arg(Arg::with_name(ARG_RENDERER)
            .short("r")
            .long("renderer")
            .value_name("terminal | sdl")
            .help("Render method to use. For terminal root privileges are needed (for reading keyboard input asynchronously)")
            .takes_value(true))
        .get_matches();

    let game_file = matches.value_of(ARG_GAME).unwrap();
    let renderer_arg = matches.value_of(ARG_RENDERER).unwrap_or("sdl");

    // Initialize chip8 state
    let vm = chip8::State::new(game_file.to_string()).unwrap();

    // Initialize graphics and input;
    let renderer: Renderer = get_renders(renderer_arg.to_string());

    // Run game loop
    run_loop(vm, renderer);
}

fn run_loop(mut vm: chip8::State, renderer: Renderer) {
    let mut graphics = renderer.graphics;
    let mut input = renderer.input;
    graphics.initialize();
    input.initialize();

    'running: loop {
        match input.get_keyboard_state() {
            KeyboardCommand::KeypadState(state) => { vm.set_keys_pressed(state) },
            KeyboardCommand::SingleKey(key) => { vm.wait_key_press(key) },
            KeyboardCommand::Quit => break 'running,
        }
        
        match vm.execute_cycle() {
            Ok(MachineState::SuccessfulExecution) => continue,
            Ok(MachineState::WaitForKeyboard) => input.set_waiting_key(),
            Ok(MachineState::Draw(screen)) => graphics.draw(screen),
            Err(error) => {
                println!("{}", error);
                break 'running;
            }
        }
        thread::sleep(time::Duration::from_millis(1000 / 30));
    }
}
