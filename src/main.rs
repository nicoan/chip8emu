use std::{thread, time};
extern crate rand;

mod chip8;

fn main() {
    let mut vm = chip8::State::new("/home/nico/Proyectos/c8games/Chip8 Picture.ch8".to_string()).unwrap();
    while true {
        vm.execute_instruction();
        thread::sleep(time::Duration::from_millis(100));
    }

    println!("Hello world!");
}
