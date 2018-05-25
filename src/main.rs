use std::{thread, time};
extern crate rand;

mod chip8;

fn main() {
    let mut vm = chip8::State::new("/home/lilo/Proyectos/c8games/MAZE".to_string()).unwrap();
    while true {
        vm.execute_instruction();
        thread::sleep(time::Duration::from_millis(1000));
    }

    println!("Hello world!");
}
