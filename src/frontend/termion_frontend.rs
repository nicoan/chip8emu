extern crate termion;
use frontend::frontend::Frontend;
use termion::{cursor, clear};
use termion::raw::IntoRawMode;
use termion::event::Key;
use std::io::{Read, Write, stdout, stdin, Stdout, Bytes};
use termion::input::TermRead;
use termion::async_stdin;

pub struct TermionFrontend {
    output_stream: termion::raw::RawTerminal<Stdout>,
    input_stream: Bytes<termion::AsyncReader>,
}

impl TermionFrontend {
    pub fn new() -> Self {
        TermionFrontend {
            output_stream: stdout().into_raw_mode().unwrap(),
            input_stream: async_stdin().bytes()
        }
    }
}

impl Frontend for TermionFrontend {
    fn initialize(&mut self) {
        // Clear and hide cursor
        write!(self.output_stream, "{}", clear::All).unwrap();
        write!(self.output_stream, "{}", cursor::Hide).unwrap();

        // Draw screen box
        write!(self.output_stream, "{}", clear::All).unwrap();
        // Top row
        write!(self.output_stream, "{}┌", cursor::Goto(1, 1)).unwrap();
        for i in 2..66 {
            write!(self.output_stream, "{}─", cursor::Goto(i, 1)).unwrap();
        }
        write!(self.output_stream, "{}┐", cursor::Goto(66, 1)).unwrap();

        // Vertical rows
        for i in 2..18 {
            write!(self.output_stream, "{}│", cursor::Goto(1, i)).unwrap();
            write!(self.output_stream, "{}│", cursor::Goto(66, i)).unwrap();
        }

        // Bottom row
        write!(self.output_stream, "{}└", cursor::Goto(1, 18)).unwrap();
        for i in 2..66 {
            write!(self.output_stream, "{}─", cursor::Goto(i, 18)).unwrap();
        }
        write!(self.output_stream, "{}┘", cursor::Goto(66, 18)).unwrap();
    }

    fn draw(&mut self, screen: [[u8; 8]; 32]) {
        const PADDING: u16 = 2;
        for y in (0..16).map(|x| x * 2) {
            for x in 0..8 {
                for i in 0..8 {
                    let top_square: bool = (screen[y as usize][x as usize] << i) & 0x80 == 0x80;
                    let bottom_square: bool = (screen[y + 1 as usize][x as usize] << i) & 0x80 == 0x80;
                    let x_coord = (x * 8) + i + PADDING;
                    let y_coord: u16 = (y / 2) as u16 + PADDING;
                    match (top_square, bottom_square) {
                        (true, true) => write!(self.output_stream, "{}█", cursor::Goto(x_coord, y_coord)).unwrap(),
                        (true, false) => write!(self.output_stream, "{}▀", cursor::Goto(x_coord, y_coord)).unwrap(),
                        (false, true) => write!(self.output_stream, "{}▄", cursor::Goto(x_coord, y_coord)).unwrap(),
                        (false, false) => write!(self.output_stream, "{} ", cursor::Goto(x_coord, y_coord)).unwrap()
                    }
                }
            }
        }
        self.output_stream.flush().unwrap();
    }

    fn check_pressed_keys(&mut self) -> u16 {
        let mut result: u16 = 0x0;
        loop {
            match self.input_stream.next() {
                Some(Ok(b'1')) => { result |= 0x1 },
                Some(Ok(b'2')) => { result |= 0x2; },
                Some(Ok(b'3')) => { result |= 0x4; },
                Some(Ok(b'4')) => { result |= 0x8; },
                Some(Ok(b'q')) => { result |= 0x10; },
                Some(Ok(b'w')) => { result |= 0x20; },
                Some(Ok(b'e')) => { result |= 0x40; },
                Some(Ok(b'r')) => { result |= 0x80; },
                Some(Ok(b'a')) => { result |= 0x100; },
                Some(Ok(b's')) => { result |= 0x200; },
                Some(Ok(b'd')) => { result |= 0x400; },
                Some(Ok(b'f')) => { result |= 0x800; },
                Some(Ok(b'z')) => { result |= 0x1000; },
                Some(Ok(b'x')) => { result |= 0x2000; },
                Some(Ok(b'v')) => { result |= 0x4000; },
                Some(Ok(b'c')) => { result |= 0x8000; },
                None => break,
                _ => {},
            }
        }
        return result;
    }

    fn wait_for_key(&mut self) -> u8 {
        let stdin = stdin();
        println!("asdas");
        for c in stdin.keys() {
            println!("asdas");
            match c.unwrap() {
                Key::Char('1') => return 0,
                Key::Char('2') => return 1,
                Key::Char('3') => return 2,
                Key::Char('4') => return 3,
                Key::Char('q') => return 4,
                Key::Char('w') => return 5,
                Key::Char('e') => return 6,
                Key::Char('r') => return 7,
                Key::Char('a') => return 8,
                Key::Char('s') => return 9,
                Key::Char('d') => return 10,
                Key::Char('f') => return 11,
                Key::Char('z') => return 12,
                Key::Char('x') => return 13,
                Key::Char('v') => return 14,
                Key::Char('c') => return 15,
                _ => return self.wait_for_key()
            }
        }
        return 0x0;
    }
}