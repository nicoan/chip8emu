use std::{thread, time};
extern crate rand;
extern crate termion;
use termion::{clear};

mod chip8;
use chip8::{MachineState};

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
//use std::io::{Write, stdout, stdin};
use std::io::{stdin, stdout, Read, Write};

/*fn main() {
    // Get the standard input stream.
    let stdin = stdin();
    // Get the standard output stream and go to raw mode.
    let mut stdout = stdout().into_raw_mode().unwrap();

    write!(stdout, "{}{}q to exit. Type stuff, use alt, and so on.{}",
           // Clear the screen.
           termion::clear::All,
           // Goto (1,1).
           termion::cursor::Goto(1, 1),
           // Hide the cursor.
           termion::cursor::Hide).unwrap();
    // Flush stdout (i.e. make the output appear).
    stdout.flush().unwrap();

    for c in stdin.keys() {
        // Clear the current line.
        write!(stdout, "{}{}", termion::cursor::Goto(1, 1), termion::clear::CurrentLine).unwrap();

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

        // Flush again.
        stdout.flush().unwrap();
    }

    // Show the cursor again before we exit.
    write!(stdout, "{}", termion::cursor::Show).unwrap();
}*/


fn main() {
    //let mut vm = chip8::State::new("/home/nico/Downloads/chp8_220/CHIP8/GAMES/BC_test.ch8".to_string()).unwrap();
    let mut vm = chip8::State::new("/home/nico/Downloads/chp8_220/CHIP8/GAMES/TETRIS".to_string()).unwrap();
    //let mut vm = chip8::State::new("/home/nico/Downloads/chp8_220/CHIP8/GAMES/MAZE".to_string()).unwrap();
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