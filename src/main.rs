use std::{thread, time};
extern crate rand;
extern crate termion;


mod chip8;
use chip8::{MachineState};

fn main() {
    //let mut vm = chip8::State::new("/home/lilo/Proyectos/c8games/Chip8 Picture.ch8".to_string()).unwrap();
    let mut vm = chip8::State::new("/home/lilo/Proyectos/c8games/INVADERS".to_string()).unwrap();
    loop {
        match vm.execute_instruction() {
            Ok(MachineState::SuccessfulExecution) => continue,
            Ok(MachineState::WaitForKeyboard(k)) => vm.wait_key_press(k),
            Ok(MachineState::Draw) => vm.print_screen(),
            Err(error) => {
                println!("{}", error);
                break;
            }
        }
    }

    println!("Hello world!");
}
