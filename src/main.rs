use std::{thread, time};
extern crate rand;
extern crate termion;
use std::env;

mod chip8;
use chip8::{MachineState};

mod frontend;
use frontend::frontend::Frontend;
use frontend::termion_frontend::TermionFrontend;

use termion::event::Key;
use termion::input::TermRead;
use std::io::{stdin, stdout, Read, Write};


fn main() {
    let file = env::args().nth(1).expect("Missing argument");

    // Initialize frontend
    let mut frontend = TermionFrontend::new();

    // Initialize chip8 state
    let mut vm = chip8::State::new(file).unwrap();

    // Run game loop
    run_loop(vm, frontend);

    thread::spawn(|| { keyboard_listener() });
}

fn run_loop<T>(mut vm: chip8::State, mut frontend: T) where T: Frontend  {
    frontend.initialize();
    loop {
        //pause();
        match vm.execute_cycle() {
            Ok(MachineState::SuccessfulExecution) => continue,
            Ok(MachineState::WaitForKeyboard(k)) => keyboard_listener(),
            Ok(MachineState::Draw(screen)) => frontend.draw(screen),
            Err(error) => {
                println!("{}", error);
                break;
            }
        }
        thread::sleep(time::Duration::from_millis(1000 / 60))
    }
}

fn pause() {
    let mut stdout = stdout();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}

fn keyboard_listener() {
    let stdin = stdin();
    for c in stdin.keys() {
        // Print the key we type...
        match c.unwrap() {
            // Exit.
            Key::Char('q') => break,
            Key::Char(c)   => println!("{}", c),
            Key::Alt(c)    => println!("Alt-{}", c),
            Key::Ctrl(c)   => println!("Ctrl-{}", c),
            Key::Left      => println!("<left>"),
            Key::Right     => println!("<right>"),
            Key::Up        => println!("<up>"),
            Key::Down      => println!("<down>"),
            _              => println!("Other"),
        }
    }
}