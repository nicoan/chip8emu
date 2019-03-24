use std::{thread, time};
extern crate rand;
extern crate termion;
use std::env;

mod chip8;
use chip8::{MachineState};

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::io::{stdin, stdout, Read, Write};


fn main() {
    let file = env::args().nth(1).expect("Missing argument");

    let mut vm = chip8::State::new(file).unwrap();
    thread::spawn(|| { keyboard_listener() });
    vm.initialize_render();
    loop {
        //pause();
        match vm.execute_cycle() {
            Ok(MachineState::SuccessfulExecution) => continue,
            Ok(MachineState::WaitForKeyboard(k)) => keyboard_listener(),
            Ok(MachineState::Draw) => vm.print_screen(),
            Err(error) => {
                println!("{}", error);
                break;
            }
        }
        thread::sleep(time::Duration::from_millis(1000 / 60))
    }

    println!("Hello world!");
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