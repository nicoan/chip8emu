use std::{thread, time};
extern crate rand;
extern crate termion;


mod chip8;

fn main() {
    //let mut vm = chip8::State::new("/home/lilo/Proyectos/c8games/Chip8 Picture.ch8".to_string()).unwrap();
    let mut vm = chip8::State::new("/home/lilo/Proyectos/c8games/CONNECT4".to_string()).unwrap();
    while true {
        vm.execute_instruction();
        thread::sleep(time::Duration::from_millis(100));
    }

    println!("Hello world!");
}
