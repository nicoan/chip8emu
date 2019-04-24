extern crate rand;
extern crate termion;
extern crate linux_raw_input_rs;
use std::{thread, time};
use std::env;

mod chip8;
use chip8::{MachineState};

mod frontend;
use frontend::frontend::Frontend;
use frontend::termion_frontend::TermionFrontend;

fn main() {
    let file = env::args().nth(1).expect("Missing argument");

    // Initialize frontend
    let frontend = TermionFrontend::new();

    // Initialize chip8 state
    let vm = chip8::State::new(file).unwrap();

    // Run game loop
    run_loop(vm, frontend);
}

fn run_loop<T>(mut vm: chip8::State, mut frontend: T) where T: Frontend  {
    frontend.initialize();
    loop {
        vm.set_keys_pressed(frontend.get_keyboard_state());
        match vm.execute_cycle() {
            Ok(MachineState::SuccessfulExecution) => continue,
            Ok(MachineState::WaitForKeyboard) => { vm.wait_key_press(frontend.wait_for_key()) },
            Ok(MachineState::Draw(screen)) => frontend.draw(screen),
            Err(error) => {
                println!("{}", error);
                break;
            }
        }
        thread::sleep(time::Duration::from_millis(1000 / 60));
    }
}
