extern crate rand;

mod chip8;

fn main() {
    let mut vm = chip8::State::new("/home/nico/Proyectos/c8games/MAZE".to_string()).unwrap();
    while true {
        vm.execute_instruction();
    }

    println!("Hello world!");
}
